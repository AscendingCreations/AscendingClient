use thiserror::Error;

pub type SocketResult<T> = std::result::Result<T, AscendingSocketError>;

#[derive(Error, Debug)]
pub enum AscendingSocketError {
    #[error("Currently Unhandled data error")]
    Unhandled,
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
    AddrParseError(#[from] std::net::AddrParseError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    UnicodeError(#[from] std::str::Utf8Error),
    #[error(transparent)]
    ByteyError(#[from] bytey::ByteBufferError),
    #[error(transparent)]
    ParseError(#[from] std::string::ParseError),
    #[error(transparent)]
    Rustls(#[from] rustls::Error),
    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),
    #[error(transparent)]
    ParseNum(#[from] std::num::ParseIntError),
}
