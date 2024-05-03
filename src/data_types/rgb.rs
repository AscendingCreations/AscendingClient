use crate::socket::*;
use serde::{Deserialize, Serialize};
use speedy::{Endianness, Readable, Writable};

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Default,
    Deserialize,
    Serialize,
    Readable,
    Writable,
    MByteBufferRead,
    MByteBufferWrite,
)]
pub struct Rgba {
    pub r: i16,
    pub g: i16,
    pub b: i16,
    pub a: i16,
}
