use crate::{
    Alert, BufferTask, ClientError, Content, Result, SERVER_ID, SystemHolder,
    TLS_SERVER_PORT, config::*,
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

use super::notls_socket::connect;
use super::{
    PacketRouter,
    bufer_ext::ByteBufferExt,
    handle_data,
    states::{ClientState, PollState},
};

pub struct TlsSocket {
    pub socket: TcpStream,
    pub token: mio::Token,
    pub tls: rustls::ClientConnection,
    pub state: ClientState,
    pub buffer: ByteBuffer,
    pub sends: VecDeque<MByteBuffer>,
    pub poll_state: PollState,
}

///Creates the Socket and TLS Streams
fn tls_socket_setup() -> Result<(TcpStream, rustls::ClientConnection)> {
    let tls_config = build_tls_config()?;
    let host = SERVER_ID;
    let socket = connect(host, TLS_SERVER_PORT)?;
    let server_name = ServerName::try_from(host)?.to_owned();

    Ok((
        socket,
        rustls::ClientConnection::new(tls_config, server_name)?,
    ))
}

impl TlsSocket {
    pub fn new(poll: &mio::Poll) -> Result<Self> {
        let (socket, tls) = tls_socket_setup()?;
        let mut socket = TlsSocket {
            socket,
            token: mio::Token(0),
            tls,
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
        let (socket, tls) = tls_socket_setup()?;

        self.socket = socket;
        self.tls = tls;
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
                        //self.socket.set_nodelay(true)?;
                    }
                    Err(err) => {
                        if let Some(os_err) = err.raw_os_error() {
                            if os_err == 115 {
                                self.reregister(poll)?;
                                return Ok(());
                            } else {
                                log::error!(
                                    "Connection OS Error id: {}",
                                    os_err
                                );
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
                    log::error!("TLS Connection Error: {:?}", err);
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

        loop {
            match self.tls.read_tls(&mut self.socket) {
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                    break;
                }
                Err(ref err) if err.kind() == io::ErrorKind::Interrupted => {
                    continue;
                }
                Err(error) => {
                    log::error!("TLS read error: {:?}", error);
                    self.state = ClientState::Closing;
                    return Ok(());
                }
                Ok(0) => {
                    log::info!("Disconnected on read tls ok(0)");
                    self.state = ClientState::Closing;
                    return Ok(());
                }
                Ok(_) => {}
            }

            let io_state = match self.tls.process_new_packets() {
                Ok(io_state) => io_state,
                Err(err) => {
                    log::error!("TLS error: {:?}", err);
                    self.state = ClientState::Closing;
                    return Ok(());
                }
            };

            if io_state.plaintext_bytes_to_read() > 0 {
                let mut buf = vec![0u8; io_state.plaintext_bytes_to_read()];
                if self.tls.reader().read_exact(&mut buf).is_err() {
                    log::error!(
                        "Disconnected on tls plaintext_bytes_to_read read exact"
                    );
                    self.state = ClientState::Closing;
                    return Ok(());
                }

                if self.buffer.write_slice(&buf).is_err() {
                    log::error!(
                        "Disconnected on tls plaintext_bytes_to_read write slice"
                    );
                    self.state = ClientState::Closing;
                    return Ok(());
                }
            }

            if io_state.peer_has_closed() {
                log::error!("Disconnected on peer_has_closed");
                self.state = ClientState::Closing;
            }

            break; //If we make it here means we read all we could get and had no Interruptions on the I/O.
        }

        self.buffer.move_cursor(pos)?;
        Ok(())
    }

    pub fn write(&mut self) {
        loop {
            let mut packet = match self.sends.pop_front() {
                Some(packet) => packet,
                None => {
                    if self.sends.capacity() > 25 {
                        self.sends.shrink_to_fit();
                    }
                    break;
                }
            };

            match self.tls.writer().write_all(packet.as_slice()) {
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                    self.sends.push_front(packet);
                    break;
                }
                Err(_) => {
                    log::error!("Disconnected on Socket tls writer Write All");
                    self.state = ClientState::Closing;
                    return;
                }
                Ok(_) => {}
            }
        }

        loop {
            if self.tls.wants_write() {
                match self.tls.write_tls(&mut self.socket) {
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
                        self.state = ClientState::Closing;
                        return;
                    }
                    Ok(_) => {}
                };
            } else {
                break;
            }
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

            trace!("Length is {}", length);
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
