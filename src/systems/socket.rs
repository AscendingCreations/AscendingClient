use crate::{
    config::*, Alert, BufferTask, ClientError, Content, Result, SystemHolder,
    SERVER_ID, SERVER_PORT,
};
pub use bytey::{ByteBuffer, ByteBufferError, ByteBufferRead, ByteBufferWrite};
use hecs::World;
use log::{info, trace, warn};
use mio::net::TcpStream;
use mio::{Events, Interest, Poll};
pub use mmap_bytey::{
    MByteBuffer, MByteBufferError, MByteBufferRead, MByteBufferWrite,
};
use pki_types::ServerName;
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    convert::TryFrom,
    io::{self, Read, Write},
    net::ToSocketAddrs,
    str,
    sync::Arc,
    time::Duration,
};

pub mod handledata;
pub mod sends;
pub use handledata::*;
pub use sends::*;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EncryptionState {
    /// Send Unencrypted packets only.
    None,
    /// Send Encrypted for both read and write.
    ReadWrite,
    ///Migrating from encrypted to unencrypted after the last send.
    ///Read will start to read unencrypted traffic at this point.
    ///Only call this when we send the last nagotiation packet.
    WriteTransfering,
}

pub struct Socket {
    pub client: Client,
    pub poll: mio::Poll,
    pub buffer: ByteBuffer,
    pub encrypt_state: EncryptionState,
}

impl Socket {
    pub fn new(_config: &Config) -> Result<Self> {
        let tls_config = build_tls_config()?;

        Ok(Socket {
            client: Client::new(
                SERVER_ID,
                SERVER_PORT,
                mio::Token(0),
                tls_config,
            )?,
            poll: Poll::new()?,
            buffer: ByteBuffer::new_packet_with(8192)?,
            encrypt_state: EncryptionState::ReadWrite,
        })
    }

    pub fn reconnect(&mut self) -> Result<()> {
        let tls_config = build_tls_config()?;
        self.client =
            Client::new(SERVER_ID, SERVER_PORT, mio::Token(0), tls_config)?;
        self.encrypt_state = EncryptionState::ReadWrite;
        self.buffer.truncate(0)?;
        Ok(())
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_to_closing(&mut self) {
        self.client.state = ClientState::Closing;
        self.client.poll_state.add(SocketPollState::Write);
        self.reregister().unwrap();
    }

    fn process(&mut self, event: &mio::event::Event) -> Result<()> {
        self.client.poll_state.set(SocketPollState::Read);

        if event.is_readable() {
            if matches!(self.encrypt_state, EncryptionState::ReadWrite) {
                self.tls_read()?;
            } else {
                self.read()?;
            }
        }

        if event.is_writable() {
            if matches!(
                self.encrypt_state,
                EncryptionState::WriteTransfering | EncryptionState::ReadWrite
            ) {
                self.tls_write();
            } else {
                self.write();
            }
        }

        if self.encrypt_state == EncryptionState::WriteTransfering
            && self.client.tls_sends.is_empty()
        {
            self.client.tls_sends = VecDeque::new();
            self.encrypt_state = EncryptionState::None;
        } else {
            self.client.poll_state.add(SocketPollState::Write);
        }

        match self.client.state {
            ClientState::Closing => {
                self.poll.registry().deregister(&mut self.client.socket)?;
                //self.client.socket.shutdown(std::net::Shutdown::Both)?;
                self.client.state = ClientState::Closed;
            }
            _ => self.reregister()?,
        }

        Ok(())
    }

    fn tls_read(&mut self) -> Result<()> {
        let pos = self.buffer.cursor();
        self.buffer.move_cursor_to_end();

        loop {
            match self.client.tls.read_tls(&mut self.client.socket) {
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                    break;
                }
                Err(ref err) if err.kind() == io::ErrorKind::Interrupted => {
                    continue;
                }
                Err(error) => {
                    log::error!("TLS read error: {:?}", error);
                    self.client.state = ClientState::Closing;
                    return Ok(());
                }
                Ok(0) => {
                    log::info!("Disconnected on read tls ok(0)");
                    self.client.state = ClientState::Closing;
                    return Ok(());
                }
                Ok(_) => {}
            }

            let io_state = match self.client.tls.process_new_packets() {
                Ok(io_state) => io_state,
                Err(err) => {
                    log::error!("TLS error: {:?}", err);
                    self.client.state = ClientState::Closing;
                    return Ok(());
                }
            };

            if io_state.plaintext_bytes_to_read() > 0 {
                let mut buf = vec![0u8; io_state.plaintext_bytes_to_read()];
                if self.client.tls.reader().read_exact(&mut buf).is_err() {
                    log::error!("Disconnected on tls plaintext_bytes_to_read read exact");
                    self.client.state = ClientState::Closing;
                    return Ok(());
                }

                if self.buffer.write_slice(&buf).is_err() {
                    log::error!("Disconnected on tls plaintext_bytes_to_read write slice");
                    self.client.state = ClientState::Closing;
                    return Ok(());
                }
            }

            if io_state.peer_has_closed() {
                log::error!("Disconnected on peer_has_closed");
                self.client.state = ClientState::Closing;
            }

            break; //If we make it here means we read all we could get and had no Interruptions on the I/O.
        }

        self.buffer.move_cursor(pos)?;
        Ok(())
    }

    fn read(&mut self) -> Result<()> {
        let pos = self.buffer.cursor();
        self.buffer.move_cursor_to_end();
        let mut buf: [u8; 4096] = [0; 4096];

        loop {
            match self.client.socket.read(&mut buf) {
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                    break
                }
                Err(ref err) if err.kind() == io::ErrorKind::Interrupted => {
                    continue
                }
                Ok(0) | Err(_) => {
                    log::error!("Disconnected on Socket read");
                    self.client.state = ClientState::Closing;
                    return Ok(());
                }
                Ok(n) => {
                    if self.buffer.write_slice(&buf[0..n]).is_err() {
                        log::error!(
                            "Disconnected on Socket buffer write slice"
                        );
                        self.client.state = ClientState::Closing;
                        return Ok(());
                    }
                }
            }
        }

        self.buffer.move_cursor(pos)?;
        Ok(())
    }

    pub fn tls_write(&mut self) {
        loop {
            let mut packet = match self.client.tls_sends.pop_front() {
                Some(packet) => packet,
                None => {
                    if self.client.tls_sends.capacity() > 25 {
                        self.client.tls_sends.shrink_to_fit();
                    }
                    break;
                }
            };

            match self.client.tls.writer().write_all(packet.as_slice()) {
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                    self.client.tls_sends.push_front(packet);
                    break;
                }
                Err(_) => {
                    log::error!("Disconnected on Socket tls writer Write All");
                    self.client.state = ClientState::Closing;
                    return;
                }
                Ok(_) => {}
            }
        }

        loop {
            if self.client.tls.wants_write() {
                match self.client.tls.write_tls(&mut self.client.socket) {
                    Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                        return;
                    }
                    Err(ref err)
                        if err.kind() == io::ErrorKind::Interrupted =>
                    {
                        continue;
                    }
                    Err(_) => {
                        log::error!("Disconnected on Socket tls Wants Write");
                        self.client.state = ClientState::Closing;
                        return;
                    }
                    Ok(_) => {}
                };
            } else {
                break;
            }
        }

        if !self.client.tls_sends.is_empty() {
            self.client.poll_state.add(SocketPollState::Write);
        }
    }

    pub fn write(&mut self) {
        let mut packet = match self.client.sends.pop_front() {
            Some(packet) => packet,
            None => return,
        };

        match self.client.socket.write_all(packet.as_slice()) {
            Ok(()) => {}
            Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                //Operation would block so we insert it back in to try again later.
                self.client.sends.push_front(packet);
            }
            Err(_) => {
                log::error!("Disconnected on Socket Write All");
                self.client.state = ClientState::Closing;
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

    pub fn register(&mut self) -> Result<()> {
        if let Some(interest) = self.event_set() {
            self.poll.registry().register(
                &mut self.client.socket,
                self.client.token,
                interest,
            )?;
        }
        Ok(())
    }

    fn reregister(&mut self) -> Result<()> {
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
    pub fn send(&mut self, buf: MByteBuffer) -> Result<()> {
        self.client.sends.push_back(buf);

        self.client.poll_state.add(SocketPollState::Write);
        self.reregister()
    }

    #[inline]
    #[allow(dead_code)]
    pub fn tls_send(&mut self, buf: MByteBuffer) -> Result<()> {
        self.client.tls_sends.push_back(buf);

        self.client.poll_state.add(SocketPollState::Write);
        self.reregister()
    }
}

pub struct Client {
    pub socket: TcpStream,
    pub token: mio::Token,
    pub tls: rustls::ClientConnection,
    pub state: ClientState,
    pub poll_state: SocketPollState,
    /// for unencrypted sends only.
    pub sends: VecDeque<MByteBuffer>,
    /// for encrypted sends only.
    pub tls_sends: VecDeque<MByteBuffer>,
}

pub trait MByteBufferExt {
    fn new_packet() -> Result<MByteBuffer>;
    fn write_str(&mut self, str: &str) -> Result<&mut MByteBuffer>;
    fn read_str(&mut self) -> Result<String>;
    fn finish(&mut self) -> Result<&mut MByteBuffer>;
}

impl MByteBufferExt for MByteBuffer {
    fn new_packet() -> Result<MByteBuffer> {
        let mut buffer = MByteBuffer::new()?;
        buffer.write(0u64)?;
        Ok(buffer)
    }

    #[inline]
    fn write_str(&mut self, str: &str) -> Result<&mut Self> {
        let bytestr = str.as_bytes();
        self.write(bytestr.len() as u64)?;
        Ok(self.write_slice(bytestr)?)
    }

    #[inline]
    fn read_str(&mut self) -> Result<String> {
        let size = self.read::<u64>()? as usize;

        if size == 0 {
            return Ok(String::new());
        }

        match str::from_utf8(self.read_slice(size)?) {
            Ok(string) => Ok(String::from(string)),
            Err(_) => Ok(String::new()),
        }
    }

    #[inline]
    fn finish(&mut self) -> Result<&mut MByteBuffer> {
        self.move_cursor(0)?;
        self.write((self.length() - 8) as u64)?;
        Ok(self.move_cursor(0)?)
    }
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
        self.move_cursor(0)?;
        self.write((self.length() - 8) as u64)?;
        self.move_cursor(0)
    }
}

fn connect(host: &str, port: u16) -> Result<TcpStream> {
    let addrs = (host, port).to_socket_addrs()?;

    for addr in addrs {
        info!("{:?}", addr);
        let stream = TcpStream::connect(addr);

        if let Ok(stream) = stream {
            return Ok(stream);
        }
    }

    Err(ClientError::BadConnection {
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
    ) -> Result<Client> {
        let socket = connect(host, port)?;
        let server_name = ServerName::try_from(host)?.to_owned();
        Ok(Client {
            socket,
            token,
            tls: rustls::ClientConnection::new(config, server_name)?,
            state: ClientState::Open,
            poll_state: SocketPollState::Read,
            sends: VecDeque::with_capacity(32),
            tls_sends: VecDeque::with_capacity(4),
        })
    }
}

pub fn poll_events(socket: &mut Socket) -> Result<bool> {
    let mut events = Events::with_capacity(32);
    socket.poll.poll(&mut events, Some(Duration::new(0, 0)))?;

    for event in events.into_iter() {
        socket.process(event)?;

        if let ClientState::Closed = socket.client.state {
            warn!("Disconnected");
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn get_length(socket: &mut Socket) -> Option<u64> {
    if socket.buffer.length() - socket.buffer.cursor() >= 8 {
        let length = socket.buffer.read::<u64>().ok()?;

        trace!("Length is {}", length);
        if !(2..=8192).contains(&length) {
            log::error!("Disconnected on packet get_length");
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
    systems: &mut SystemHolder,
    content: &mut Content,
    alert: &mut Alert,
    seconds: f32,
    buffertask: &mut BufferTask,
) -> Result<()> {
    let mut packet = MByteBuffer::new()?;

    loop {
        packet.move_cursor_to_start();
        let length = match get_length(socket) {
            Some(n) => n,
            None => return Ok(()),
        };

        if length == 0 {
            log::error!("Length was Zero. Bad or malformed packet.");
            socket.set_to_closing();
            break;
        }

        if length <= (socket.buffer.length() - socket.buffer.cursor()) as u64 {
            let mut errored = false;

            if let Ok(bytes) = socket.buffer.read_slice(length as usize) {
                if packet.write_slice(bytes).is_err() {
                    errored = true;
                }

                packet.move_cursor_to_start();
            } else {
                errored = true;
            }

            if errored {
                log::error!("Disconnected on packet read to buffer");
                socket.set_to_closing();
                break;
            }

            if handle_data(
                socket,
                router,
                world,
                systems,
                content,
                alert,
                &mut packet,
                seconds,
                buffertask,
            )
            .is_err()
            {
                log::error!("Disconnected on handle_data");
                socket.set_to_closing();
                break;
            }
        } else {
            socket.buffer.move_cursor(socket.buffer.cursor() - 8)?;
            break;
        }
    }

    if socket.buffer.cursor() == socket.buffer.length() {
        socket.buffer.truncate(0)?;
    }

    Ok(())
}
