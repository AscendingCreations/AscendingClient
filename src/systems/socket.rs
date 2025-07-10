use crate::{
    Alert, BufferTask, ClientError, Content, Result, SERVER_ID, SERVER_PORT,
    SystemHolder, World, config::*,
};
pub use bytey::{ByteBuffer, ByteBufferError, ByteBufferRead, ByteBufferWrite};

use graphics::MapRenderer;
use log::{info, trace, warn};
use mio::net::TcpStream;
use mio::{Events, Interest, Poll, Token};
pub use mmap_bytey::{
    MByteBuffer, MByteBufferError, MByteBufferRead, MByteBufferWrite,
};
use notls_socket::Socket;
use pki_types::ServerName;
use serde::{Deserialize, Serialize};
use states::ClientState;
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
use tls_socket::TlsSocket;

pub mod handledata;
pub mod sends;
pub use handledata::*;
pub use sends::*;

pub mod bufer_ext;
pub mod notls_socket;
pub mod states;
pub mod tls_socket;

pub struct Poller {
    pub tls_socket: TlsSocket,
    pub socket: Socket,
    pub poll: mio::Poll,
}

impl Poller {
    pub fn new() -> Result<Self> {
        let poll = Poll::new()?;
        let tls_socket = TlsSocket::new(&poll)?;
        let socket = Socket::new(&poll)?;

        Ok(Poller {
            tls_socket,
            socket,
            poll,
        })
    }

    pub fn reconnect(&mut self, is_tls: bool) -> Result<()> {
        if is_tls {
            self.tls_socket.reconnect(&self.poll)
        } else {
            self.socket.reconnect(&self.poll)
        }
    }

    #[inline]
    #[allow(dead_code)]
    /// this should properly Close the socket.
    pub fn set_to_closing(&mut self, is_tls: bool) {
        if is_tls {
            self.tls_socket.set_to_closing(&self.poll);
        } else {
            self.socket.set_to_closing(&self.poll);
        }
    }

    pub fn shutdown(&mut self, is_tls: bool) -> Result<()> {
        if is_tls {
            self.tls_socket.shutdown(&self.poll)
        } else {
            self.socket.shutdown(&self.poll)
        }
    }

    fn process(
        &mut self,
        event: &mio::event::Event,
        is_tls: bool,
    ) -> Result<()> {
        if is_tls {
            self.tls_socket.process(event, &self.poll)
        } else {
            self.socket.process(event, &self.poll)
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn send(&mut self, buf: MByteBuffer, is_tls: bool) -> Result<()> {
        if is_tls {
            self.tls_socket.send(buf, &self.poll)
        } else {
            self.socket.send(buf, &self.poll)
        }
    }

    /// Returns (is_tls, is_dead). Will return is_tls false if both succeed.
    pub fn poll_events(&mut self) -> Result<()> {
        let mut events = Events::with_capacity(32);
        self.poll.poll(&mut events, Some(Duration::new(0, 0)))?;

        for event in events.into_iter() {
            if event.token() == Token(0) {
                self.tls_socket.process(event, &self.poll)?;

                if let ClientState::Closed = self.tls_socket.state {
                    warn!("Disconnected TLS");
                }
            } else {
                self.socket.process(event, &self.poll)?;

                if let ClientState::Closed = self.socket.state {
                    warn!("Disconnected");
                }
            }
        }

        Ok(())
    }

    pub fn process_packets(
        &mut self,
        router: &PacketRouter,
        world: &mut World,
        systems: &mut SystemHolder,
        map_renderer: &mut MapRenderer,
        content: &mut Content,
        alert: &mut Alert,
        seconds: f32,
        buffertask: &mut BufferTask,
    ) -> Result<()> {
        let mut packet = MByteBuffer::new()?;

        //check TLS first
        loop {
            packet.move_cursor_to_start();
            let length = match self.tls_socket.get_length(&self.poll) {
                Some(n) => n,
                None => break,
            };

            if length == 0 {
                log::error!("Length was Zero. Bad or malformed packet.");
                self.tls_socket.set_to_closing(&self.poll);
                break;
            }

            if length
                <= (self.tls_socket.buffer.length()
                    - self.tls_socket.buffer.cursor()) as u64
            {
                let mut errored = false;

                if let Ok(bytes) =
                    self.tls_socket.buffer.read_slice(length as usize)
                {
                    if packet.write_slice(bytes).is_err() {
                        errored = true;
                    }

                    packet.move_cursor_to_start();
                } else {
                    errored = true;
                }

                if errored {
                    log::error!("Disconnected on packet read to buffer");
                    self.tls_socket.set_to_closing(&self.poll);
                    break;
                }

                if handle_data(
                    self,
                    router,
                    world,
                    systems,
                    map_renderer,
                    content,
                    alert,
                    &mut packet,
                    seconds,
                    buffertask,
                )
                .is_err()
                {
                    log::error!("Disconnected on handle_data");
                    self.tls_socket.set_to_closing(&self.poll);
                    break;
                }
            } else {
                self.tls_socket
                    .buffer
                    .move_cursor(self.tls_socket.buffer.cursor() - 8)?;
                break;
            }
        }

        if self.tls_socket.buffer.cursor() == self.tls_socket.buffer.length() {
            self.tls_socket.buffer.truncate(0)?;
        }

        loop {
            packet.move_cursor_to_start();
            let length = match self.socket.get_length(&self.poll) {
                Some(n) => n,
                None => break,
            };

            if length == 0 {
                log::error!("Length was Zero. Bad or malformed packet.");
                self.socket.set_to_closing(&self.poll);
                break;
            }

            if length
                <= (self.socket.buffer.length() - self.socket.buffer.cursor())
                    as u64
            {
                let mut errored = false;

                if let Ok(bytes) =
                    self.socket.buffer.read_slice(length as usize)
                {
                    if packet.write_slice(bytes).is_err() {
                        errored = true;
                    }

                    packet.move_cursor_to_start();
                } else {
                    errored = true;
                }

                if errored {
                    log::error!("Disconnected on packet read to buffer");
                    self.socket.set_to_closing(&self.poll);
                    break;
                }

                if handle_data(
                    self,
                    router,
                    world,
                    systems,
                    map_renderer,
                    content,
                    alert,
                    &mut packet,
                    seconds,
                    buffertask,
                )
                .is_err()
                {
                    log::error!("Disconnected on handle_data");
                    self.socket.set_to_closing(&self.poll);
                    break;
                }
            } else {
                self.socket
                    .buffer
                    .move_cursor(self.socket.buffer.cursor() - 8)?;
                break;
            }
        }

        if self.socket.buffer.cursor() == self.socket.buffer.length() {
            self.socket.buffer.truncate(0)?;
        }

        Ok(())
    }
}
