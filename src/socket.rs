use bytey::ByteBuffer;
use hecs::World;
use mio::net::TcpStream;
use mio::{Events, Interest, Poll};
use pki_types::ServerName;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::io::{self, Read, Write};
use std::net::ToSocketAddrs;
use std::sync::Arc;
use std::time::Duration;

use bytey::{ByteBufferRead, ByteBufferWrite};
use serde_repr::*;

pub mod error;
pub mod handledata;
pub mod sends;

pub use error::*;
pub use handledata::*;
pub use sends::*;

use crate::{config::*, Alert, BufferTask, Content, DrawSetting};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ClientState {
    Open,
    Closing,
    Closed,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SocketPollState {
    None,
    Read,
    Write,
    ReadWrite,
}

impl SocketPollState {
    #[inline]
    pub fn add(&mut self, state: SocketPollState) {
        match (*self, state) {
            (SocketPollState::None, _) => *self = state,
            (SocketPollState::Read, SocketPollState::Write) => {
                *self = SocketPollState::ReadWrite
            }
            (SocketPollState::Write, SocketPollState::Read) => {
                *self = SocketPollState::ReadWrite
            }
            (_, _) => {}
        }
    }

    #[inline]
    pub fn set(&mut self, state: SocketPollState) {
        *self = state;
    }

    #[inline]
    pub fn remove(&mut self, state: SocketPollState) {
        match (*self, state) {
            (SocketPollState::Read, SocketPollState::Read) => {
                *self = SocketPollState::None
            }
            (SocketPollState::Write, SocketPollState::Write) => {
                *self = SocketPollState::None
            }
            (SocketPollState::ReadWrite, SocketPollState::Write) => {
                *self = SocketPollState::Read
            }
            (SocketPollState::ReadWrite, SocketPollState::Read) => {
                *self = SocketPollState::Write
            }
            (_, SocketPollState::ReadWrite) => *self = SocketPollState::None,
            (_, _) => {}
        }
    }
}

pub struct Socket {
    pub client: Client,
    pub poll: mio::Poll,
    pub buffer: ByteBuffer,
}

impl Socket {
    pub fn new(_config: &Config) -> Self {
        let tls_config = build_tls_config("keys/ca.pem")
            .expect("Could not create tls config");

        Socket {
            client: Client::new("127.0.0.1", 7010, mio::Token(0), tls_config)
                .expect("Could not create Client"),
            poll: Poll::new().expect("Could not create poll"),

            buffer: ByteBuffer::new_packet_with(8192)
                .expect("Could not create buffer"),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_to_closing(&mut self) {
        self.client.state = ClientState::Closing;
        self.client.poll_state.add(SocketPollState::Write);
        self.reregister().unwrap();
    }

    fn process(&mut self, event: &mio::event::Event) -> SocketResult<()> {
        self.client.poll_state.set(SocketPollState::Read);

        if event.is_readable() {
            self.read();
        }

        if event.is_writable() {
            self.write();
        }

        match self.client.state {
            ClientState::Closing => {
                self.client.socket.shutdown(std::net::Shutdown::Both)?;
                self.client.state = ClientState::Closed;
            }
            _ => self.reregister()?,
        }

        Ok(())
    }

    fn read(&mut self) {
        let pos = self.buffer.cursor();
        let _ = self.buffer.move_cursor_to_end();

        loop {
            let mut buf: [u8; 2048] = [0; 2048];
            match self.client.socket.read(&mut buf) {
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                    break
                }
                Err(ref err) if err.kind() == io::ErrorKind::Interrupted => {
                    continue
                }
                Ok(0) | Err(_) => {
                    self.client.state = ClientState::Closing;
                    return;
                }
                Ok(n) => {
                    if self.buffer.write_slice(&buf[0..n]).is_err() {
                        self.client.state = ClientState::Closing;
                        return;
                    }
                }
            }
        }

        let _ = self.buffer.move_cursor(pos);
    }

    pub fn write(&mut self) {
        let mut count: usize = 0;

        //make sure the player exists before we send anything.
        while count < 25 {
            let mut packet = match self.client.sends.pop() {
                Some(packet) => packet,
                None => {
                    self.client.sends.shrink_to_fit();
                    return;
                }
            };

            match self.client.socket.write_all(packet.as_slice()) {
                Ok(()) => count += 1,
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                    //Operation would block so we insert it back in to try again later.
                    self.client.sends.push(packet);
                    return;
                }
                Err(_) => {
                    self.client.state = ClientState::Closing;
                    return;
                }
            }
        }
    }

    #[inline]
    pub fn event_set(&mut self) -> Option<Interest> {
        match self.client.poll_state {
            SocketPollState::None => None,
            SocketPollState::Read => Some(Interest::READABLE),
            SocketPollState::Write => Some(Interest::WRITABLE),
            SocketPollState::ReadWrite => {
                Some(Interest::READABLE.add(Interest::WRITABLE))
            }
        }
    }

    pub fn register(&mut self) -> SocketResult<()> {
        if let Some(interest) = self.event_set() {
            self.poll.registry().register(
                &mut self.client.socket,
                self.client.token,
                interest,
            )?;
        }
        Ok(())
    }

    fn reregister(&mut self) -> SocketResult<()> {
        if let Some(interest) = self.event_set() {
            self.poll.registry().reregister(
                &mut self.client.socket,
                self.client.token,
                interest,
            )?;
        }
        Ok(())
    }

    #[inline]
    #[allow(dead_code)]
    pub fn send(&mut self, buf: ByteBuffer) {
        self.client.sends.insert(0, buf);

        self.client.poll_state.add(SocketPollState::Write);
        match self.reregister() {
            Ok(_) => {}
            Err(_) => panic!("Socket did not reregister on Send write update."),
        }
    }
}

pub struct Client {
    pub socket: TcpStream,
    pub token: mio::Token,
    pub tls_conn: rustls::ClientConnection,
    pub state: ClientState,
    pub poll_state: SocketPollState,
    pub sends: Vec<ByteBuffer>,
}

pub trait ByteBufferExt {
    fn new_packet() -> bytey::Result<ByteBuffer>;
    fn new_packet_with(len: usize) -> bytey::Result<ByteBuffer>;
    fn write_str(&mut self, str: &str) -> bytey::Result<&mut ByteBuffer>;
    fn read_str(&mut self) -> bytey::Result<String>;
    fn finish(&mut self) -> bytey::Result<&mut ByteBuffer>;
}

impl ByteBufferExt for ByteBuffer {
    fn new_packet() -> bytey::Result<ByteBuffer> {
        ByteBuffer::new_packet_with(8)
    }

    fn new_packet_with(len: usize) -> bytey::Result<ByteBuffer> {
        let mut buffer = ByteBuffer::with_capacity(len + 8)?;
        buffer.write(0u64)?;
        Ok(buffer)
    }

    #[inline]
    fn write_str(&mut self, str: &str) -> bytey::Result<&mut Self> {
        let bytestr = str.as_bytes();
        self.write(bytestr.len() as u64)?;
        self.write_slice(bytestr)
    }

    #[inline]
    fn read_str(&mut self) -> bytey::Result<String> {
        let size = self.read::<u64>()? as usize;

        if size == 0 {
            return Ok(String::new());
        }

        match std::str::from_utf8(self.read_slice(size)?) {
            Ok(string) => Ok(String::from(string)),
            Err(_) => Ok(String::new()),
        }
    }

    #[inline]
    fn finish(&mut self) -> bytey::Result<&mut ByteBuffer> {
        let _ = self.move_cursor(0)?;
        let _ = self.write((self.length() - 8) as u64)?;
        self.move_cursor(0)
    }
}

fn connect(host: &str, port: u16) -> SocketResult<TcpStream> {
    let addrs = (host, port).to_socket_addrs()?;

    for addr in addrs {
        println!("{:?}", addr);
        let stream = TcpStream::connect(addr);

        if let Ok(stream) = stream {
            return Ok(stream);
        }
    }

    Err(AscendingSocketError::BadConnection {
        num: port as usize,
        message: format!("Cannot connect to {}:{}", host, port),
    })
}

impl Client {
    pub fn new(
        host: &str,
        port: u16,
        token: mio::Token,
        config: Arc<rustls::ClientConfig>,
    ) -> SocketResult<Client> {
        let socket = connect(host, port)?;

        let servername = ServerName::try_from(host)
            .expect("invalid DNS name")
            .to_owned();
        Ok(Client {
            socket,
            token,
            tls_conn: rustls::ClientConnection::new(config, servername)?,
            state: ClientState::Open,
            poll_state: SocketPollState::Read,
            sends: Vec::new(),
        })
    }
}

pub fn poll_events(socket: &mut Socket) -> SocketResult<bool> {
    let mut events = Events::with_capacity(32);
    socket.poll.poll(&mut events, Some(Duration::new(0, 0)))?;

    for event in events.into_iter() {
        socket.process(event)?;

        if let ClientState::Closed = socket.client.state {
            println!("Disconnected");
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn get_length(socket: &mut Socket) -> Option<u64> {
    if socket.buffer.length() - socket.buffer.cursor() >= 8 {
        let length = socket.buffer.read::<u64>().ok()?;

        if !(4..=8192).contains(&length) {
            socket.set_to_closing();
        }

        Some(length)
    } else {
        None
    }
}

pub fn process_packets(
    socket: &mut Socket,
    router: &PacketRouter,
    world: &mut World,
    systems: &mut DrawSetting,
    content: &mut Content,
    alert: &mut Alert,
    seconds: f32,
    buffertask: &mut BufferTask,
) {
    let mut count: usize = 0;
    let mut length: u64;

    loop {
        length = match get_length(socket) {
            Some(n) => n,
            None => return,
        };

        if length > 0
            && length
                <= (socket.buffer.length() - socket.buffer.cursor()) as u64
        {
            let mut buffer = match socket.buffer.read_to_buffer(length as usize)
            {
                Ok(n) => n,
                Err(_) => {
                    socket.set_to_closing();
                    break;
                }
            };

            if handle_data(
                socket,
                router,
                world,
                systems,
                content,
                alert,
                &mut buffer,
                seconds,
                buffertask,
            )
            .is_err()
            {
                socket.set_to_closing();
                break;
            }

            count += 1
        } else {
            let _ = socket.buffer.move_cursor(socket.buffer.cursor() - 8);
            break;
        }

        if count == 25 {
            break;
        }
    }

    if socket.buffer.cursor() == socket.buffer.length() {
        let _ = socket.buffer.truncate(0);
    }

    if socket.buffer.capacity() > 25000 {
        let _ = socket.buffer.resize(4096);
    }
}
