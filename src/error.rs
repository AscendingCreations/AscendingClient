use graphics::*;
use std::backtrace::Backtrace;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ClientError>;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Currently Unhandled data error")]
    Unhandled,
    #[error("Error: {error}, BackTrace: {backtrace}")]
    AddrParseError {
        #[from]
        error: std::net::AddrParseError,
        #[backtrace]
        backtrace: Box<Backtrace>,
    },
    #[error("Error: {error}, BackTrace: {backtrace}")]
    Io {
        #[from]
        error: std::io::Error,
        #[backtrace]
        backtrace: Box<Backtrace>,
    },
    #[error("Error: {error}, BackTrace: {backtrace}")]
    UnicodeError {
        #[from]
        error: std::str::Utf8Error,
        #[backtrace]
        backtrace: Box<Backtrace>,
    },
    #[error("Error: {error}, BackTrace: {backtrace}")]
    ParseError {
        #[from]
        error: std::string::ParseError,
        #[backtrace]
        backtrace: Box<Backtrace>,
    },
    #[error(transparent)]
    ParseNum(#[from] std::num::ParseIntError),
    #[error(transparent)]
    RodioAudio(#[from] rodio::PlayError),
    #[error(transparent)]
    RodioStreamError(#[from] rodio::StreamError),
    #[error(transparent)]
    RodioDecoderError(#[from] rodio::decoder::DecoderError),
    #[error(transparent)]
    Surface(#[from] wgpu::SurfaceError),
    #[error(transparent)]
    WGpu(#[from] wgpu::Error),
    #[error(transparent)]
    Device(#[from] wgpu::RequestDeviceError),
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
    #[error(transparent)]
    Other(#[from] OtherError),
    #[error(transparent)]
    EventLoop(#[from] winit::error::EventLoopError),
    #[error(transparent)]
    EventLoopExternal(#[from] winit::error::ExternalError),
    #[error(transparent)]
    OsError(#[from] winit::error::OsError),
    #[error("Multiple Logins Detected")]
    MultiLogin,
    #[error("Failed to register account")]
    RegisterFail,
    #[error("Failed to find the user account")]
    UserNotFound,
    #[error("Attempted usage of Socket when connection was not accepted")]
    InvalidSocket,
    #[error("Packet Manipulation detect from {name}")]
    PacketManipulation { name: String },
    #[error("Failed Packet Handling at {num} with message: {message}")]
    PacketReject { num: usize, message: String },
    #[error("Failed Packet Handling at {num} with message: {message}")]
    BadConnection { num: usize, message: String },
    #[error("Packet id was invalid")]
    InvalidPacket,
    #[error("Password was incorrect")]
    IncorrectPassword,
    #[error("No username was set.")]
    NoUsernameSet,
    #[error("No password was set")]
    NoPasswordSet,
    #[error("Error: {error}, BackTrace: {backtrace}")]
    ByteyError {
        #[from]
        error: bytey::ByteBufferError,
        #[backtrace]
        backtrace: Box<Backtrace>,
    },
    #[error("Error: {error}, BackTrace: {backtrace}")]
    Rustls {
        #[from]
        error: rustls::Error,
        #[backtrace]
        backtrace: Box<Backtrace>,
    },
    #[error("Error: {error}, BackTrace: {backtrace}")]
    TomlDe {
        #[from]
        error: toml::de::Error,
        #[backtrace]
        backtrace: Box<Backtrace>,
    },
    #[error("Error: {error}, BackTrace: {backtrace}")]
    HecsComponent {
        #[from]
        error: hecs::ComponentError,
        #[backtrace]
        backtrace: Box<Backtrace>,
    },
    #[error("Error: {error}, BackTrace: {backtrace}")]
    HecNoEntity {
        #[from]
        error: hecs::NoSuchEntity,
        #[backtrace]
        backtrace: Box<Backtrace>,
    },
    #[error("Error: {error}, BackTrace: {backtrace}")]
    RustlsInvalidDns {
        #[from]
        error: pki_types::InvalidDnsNameError,
        #[backtrace]
        backtrace: Box<Backtrace>,
    },
}
