use crate::{
    Alert, BufferTask, ClientError, Content, Result, SERVER_ID, SERVER_PORT,
    SystemHolder, config::*,
};
pub use bytey::{ByteBuffer, ByteBufferError, ByteBufferRead, ByteBufferWrite};

use log::{info, trace, warn};
use mio::net::TcpStream;
use mio::{Events, Interest, Poll};
pub use mmap_bytey::{
    MByteBuffer, MByteBufferError, MByteBufferRead, MByteBufferWrite,
};
use pki_types::ServerName;
use serde::{Deserialize, Serialize};
use snafu::Backtrace;
use std::io::ErrorKind;
use std::os;
use std::{
    collections::VecDeque,
    convert::TryFrom,
    io::{self, Read, Write},
    net::ToSocketAddrs,
    str,
    sync::Arc,
    time::Duration,
};

use super::PacketRouter;
use super::states::PollState;
use super::{ClientState, bufer_ext::ByteBufferExt};

pub struct Socket {
    pub socket: TcpStream,
    pub token: mio::Token,
    pub state: ClientState,
    pub buffer: ByteBuffer,
    pub sends: VecDeque<MByteBuffer>,
    pub poll_state: PollState,
}

impl Socket {
    pub fn new(poll: &mio::Poll) -> Result<Self> {
        let host = SERVER_ID;
        let socket = connect(host, SERVER_PORT)?;

        let mut socket = Socket {
            socket,
            token: mio::Token(1),
            state: ClientState::New,
            sends: VecDeque::with_capacity(32),
            buffer: ByteBuffer::new_packet_with(8192)?,
            poll_state: PollState::ReadWrite,
        };

        socket.register(poll)?;

        Ok(socket)
    }

    pub fn clear_sends(&mut self) {
        self.sends.clear();
    }

    pub fn clear_buffer(&mut self) -> Result<()> {
        self.buffer.truncate(0)?;
        Ok(())
    }

    pub fn reconnect(&mut self, poll: &mio::Poll) -> Result<()> {
        let host = SERVER_ID;
        let socket = connect(host, SERVER_PORT)?;

        self.socket = socket;
        self.state = ClientState::New;
        self.poll_state = PollState::ReadWrite;
        self.clear_sends();
        self.clear_buffer()?;
        self.register(poll)?;

        Ok(())
    }

    #[inline]
    #[allow(dead_code)]
    /// this should properly Close the socket.
    pub fn set_to_closing(&mut self, poll: &mio::Poll) {
        self.state = ClientState::Closing;
        self.reregister(poll).unwrap();
    }

    pub fn shutdown(&mut self, poll: &mio::Poll) -> Result<()> {
        poll.registry().deregister(&mut self.socket)?;
        self.state = ClientState::Closed;

        Ok(())
    }

    pub fn process(
        &mut self,
        event: &mio::event::Event,
        poll: &mio::Poll,
    ) -> Result<()> {
        if self.state == ClientState::New
            && (event.is_writable() || event.is_readable())
        {
            match self.socket.take_error() {
                Ok(None) => match self.socket.peer_addr() {
                    Ok(_) => {
                        self.state = ClientState::Open;
                        self.socket.set_nodelay(true)?;
                    }
                    Err(err) => {
                        if let Some(os_err) = err.raw_os_error() {
                            if os_err == 115 {
                                self.reregister(poll)?;
                                return Ok(());
                            } else {
                                log::error!("Connection OS Error id: {os_err}");
                                return self.shutdown(poll);
                            }
                        }

                        if err.kind() == ErrorKind::NotConnected {
                            self.reregister(poll)?;
                            return Ok(());
                        } else {
                            log::error!(
                                "Connection Peer Address Error Kind: {:?}",
                                err.kind()
                            );
                            return self.shutdown(poll);
                        }
                    }
                },
                Ok(Some(err)) | Err(err) => {
                    log::error!("Connection Error: {err:?}");
                    return self.shutdown(poll);
                }
            }
        }

        if event.is_readable() {
            self.read()?;
        }

        if event.is_writable() {
            self.write();
        }

        match self.state {
            ClientState::Closing => {
                poll.registry().deregister(&mut self.socket)?;
                self.state = ClientState::Closed;
            }
            _ => self.reregister(poll)?,
        }

        Ok(())
    }

    fn read(&mut self) -> Result<()> {
        let pos = self.buffer.cursor();
        self.buffer.move_cursor_to_end();
        let mut buf: [u8; 4096] = [0; 4096];

        loop {
            match self.socket.read(&mut buf) {
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                    break;
                }
                Err(ref err) if err.kind() == io::ErrorKind::Interrupted => {
                    continue;
                }
                Ok(0) => {
                    log::error!("Disconnected on Socket read");
                    self.state = ClientState::Closing;
                    return Ok(());
                }
                Err(err) => {
                    log::error!("Disconnected on Socket read err {err:?}");
                    self.state = ClientState::Closing;
                    return Ok(());
                }
                Ok(n) => {
                    if self.buffer.write_slice(&buf[0..n]).is_err() {
                        log::error!(
                            "Disconnected on Socket buffer write slice"
                        );
                        self.state = ClientState::Closing;
                        return Ok(());
                    }
                }
            }
        }

        self.buffer.move_cursor(pos)?;
        Ok(())
    }

    pub fn write(&mut self) {
        loop {
            let mut packet = match self.sends.pop_front() {
                Some(packet) => packet,
                None => break,
            };

            match self.socket.write_all(packet.as_slice()) {
                Ok(()) => {}
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                    //Operation would block so we insert it back in to try again later.
                    self.sends.push_front(packet);
                    break;
                }
                Err(_) => {
                    log::error!("Disconnected on Socket Write All");
                    self.state = ClientState::Closing;
                    break;
                }
            }
        }

        if self.sends.is_empty() {
            self.poll_state.remove(PollState::Write);
        }
    }

    pub fn register(&mut self, poll: &mio::Poll) -> Result<()> {
        poll.registry().register(
            &mut self.socket,
            self.token,
            self.poll_state.to_interest(),
        )?;

        Ok(())
    }

    fn reregister(&mut self, poll: &mio::Poll) -> Result<()> {
        if self.state == ClientState::New || self.state == ClientState::Open {
            poll.registry().reregister(
                &mut self.socket,
                self.token,
                self.poll_state.to_interest(),
            )?;
        }

        Ok(())
    }

    #[inline]
    #[allow(dead_code)]
    pub fn send(&mut self, buf: MByteBuffer, poll: &mio::Poll) -> Result<()> {
        self.sends.push_back(buf);
        self.poll_state.add(PollState::Write);
        self.reregister(poll)
    }

    pub fn get_length(&mut self, poll: &Poll) -> Option<u64> {
        if self.buffer.length() - self.buffer.cursor() >= 8 {
            let length = self.buffer.read::<u64>().ok()?;

            trace!("Length is {length}");
            if !(2..=8192).contains(&length) {
                log::error!("Disconnected on packet get_length");
                self.set_to_closing(poll);
            }

            Some(length)
        } else {
            None
        }
    }
}

pub fn connect(host: &str, port: u16) -> Result<TcpStream> {
    let addrs = (host, port).to_socket_addrs()?;

    for addr in addrs {
        info!("{addr:?}");
        let stream = TcpStream::connect(addr);

        if let Ok(stream) = stream {
            return Ok(stream);
        }
    }

    Err(ClientError::BadConnection {
        num: port as usize,
        message: format!("Cannot connect to {host}:{port}"),
        backtrace: Backtrace::new_unresolved(),
    })
}
