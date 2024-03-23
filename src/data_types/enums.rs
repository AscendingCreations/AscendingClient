use bytey::{ByteBufferRead, ByteBufferWrite};
use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Default,
    Serialize_repr,
    Deserialize_repr,
    ByteBufferRead,
    ByteBufferWrite,
)]
#[repr(u8)]
pub enum ItemTypes {
    #[default]
    None,
    Weapon,
    Accessory,
    Cosmetic,
    Helmet,
    Armor,
    Trouser,
    Boots,
    Consume,
    Tool,
    Blueprint,
    Book,
    Questitem,
    Trap,
    Heavyobject,
    Key,
    Count,
}
