use graphics::*;
use thiserror::Error;

pub type ClientResult<T> = std::result::Result<T, ClientError>;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Currently Unhandled data error")]
    Unhandled,
    #[error(transparent)]
    AddrParseError(#[from] std::net::AddrParseError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    UnicodeError(#[from] std::str::Utf8Error),
    #[error(transparent)]
    ParseError(#[from] std::string::ParseError),
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
    #[error(transparent)]
    ByteyError(#[from] bytey::ByteBufferError),
    #[error(transparent)]
    Rustls(#[from] rustls::Error),
    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),
}
