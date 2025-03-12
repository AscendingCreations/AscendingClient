use graphics::*;
use snafu::{Backtrace, Whatever, prelude::*};
use std::sync::{PoisonError, TryLockError};
pub type Result<T> = std::result::Result<T, ClientError>;

#[allow(unreachable_code)]
#[derive(Debug, snafu::Snafu)]
pub enum ClientError {
    #[snafu(display(
        "Currently Unhandled data error. BACKTRACE: {backtrace:?}"
    ))]
    Unhandled {
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    AddrParseError {
        source: std::net::AddrParseError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    GraphicsError {
        source: GraphicsError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    Io {
        source: std::io::Error,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    UnicodeError {
        source: std::str::Utf8Error,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    ParseError {
        source: std::string::ParseError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    ParseNum {
        source: std::num::ParseIntError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    RodioAudio {
        source: rodio::PlayError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    RodioStreamError {
        source: rodio::StreamError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    RodioDecoderError {
        source: rodio::decoder::DecoderError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    Surface {
        source: wgpu::SurfaceError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    WGpu {
        source: wgpu::Error,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    Device {
        source: wgpu::RequestDeviceError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    ImageError {
        source: image::ImageError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    Other {
        source: OtherError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    EventLoop {
        source: winit::error::EventLoopError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    EventLoopExternal {
        source: winit::error::ExternalError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    OsError {
        source: winit::error::OsError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(display("Multiple Logins Detected. BACKTRACE: {backtrace:?}"))]
    MultiLogin {
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(display("Failed to register account. BACKTRACE: {backtrace:?}"))]
    RegisterFail {
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(display(
        "Failed to find the user account. BACKTRACE: {backtrace:?}"
    ))]
    UserNotFound {
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(display(
        "Attempted usage of Socket when connection was not accepted. BACKTRACE: {backtrace:?}"
    ))]
    InvalidSocket {
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(display(
        "Packet Manipulation detect from {name}. BACKTRACE: {backtrace:?}"
    ))]
    PacketManipulation {
        name: String,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(display(
        "Failed Packet Handling at {num} with message: {message}. BACKTRACE: {backtrace:?}"
    ))]
    PacketReject {
        num: usize,
        message: String,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(display(
        "Failed Packet Handling at {num} with message: {message}. BACKTRACE: {backtrace:?}"
    ))]
    BadConnection {
        num: usize,
        message: String,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(display("Packet id was invalid. BACKTRACE: {backtrace:?}"))]
    InvalidPacket {
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(display("Password was incorrect. BACKTRACE: {backtrace:?}"))]
    IncorrectPassword {
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(display("No username was set. BACKTRACE: {backtrace:?}"))]
    NoUsernameSet {
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(display("No password was set. BACKTRACE: {backtrace:?}"))]
    NoPasswordSet {
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    ByteyError {
        source: bytey::ByteBufferError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    MByteyError {
        source: mmap_bytey::MByteBufferError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    Rustls {
        source: rustls::Error,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    TomlDe {
        source: toml::de::Error,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(display("Missing Kind. BACKTRACE: {backtrace:?}"))]
    MissingKind {
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(display("Missing Entity. BACKTRACE: {backtrace:?}"))]
    MissingEntity {
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    RustlsInvalidDns {
        source: pki_types::InvalidDnsNameError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(display("Mutex PoisonError Occured. BACKTRACE: {backtrace:?}"))]
    MutexLockError {
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(display("TryLock Error. BACKTRACE: {backtrace:?}"))]
    TryLockError {
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
}

impl<T> From<TryLockError<T>> for ClientError {
    fn from(_: TryLockError<T>) -> Self {
        Self::TryLockError {
            backtrace: Backtrace::new(),
        }
    }
}

impl<T> From<PoisonError<T>> for ClientError {
    fn from(_: PoisonError<T>) -> Self {
        Self::MutexLockError {
            backtrace: Backtrace::new(),
        }
    }
}

impl ClientError {
    pub fn missing_kind() -> Self {
        ClientError::MissingKind {
            backtrace: Backtrace::new(),
        }
    }

    pub fn missing_entity() -> Self {
        ClientError::MissingEntity {
            backtrace: Backtrace::new(),
        }
    }
}
