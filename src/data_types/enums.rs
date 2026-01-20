use crate::socket::*;
use graphics::*;
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

#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Default,
    Readable,
    Writable,
    MByteBufferRead,
    MByteBufferWrite,
)]
#[repr(u8)]
pub enum AIBehavior {
    #[default]
    Friendly, //Never Attack or be attacked
    Agressive,       //Will attack on sight
    Reactive,        //Will attack when attacked
    HelpReactive, //for npcs that when one gets attacked all in the area target the attacker.
    Healer,       //Will never Attack only heal other npcs
    AgressiveHealer, //Will attack on sight and heal
    ReactiveHealer, //Will attack when attacked and heal
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ControlKey {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Count,
}

#[derive(Debug, Clone, Default)]
pub enum LightData {
    #[default]
    None,
    AreaLight(AreaLight),
    DirLight(DirectionalLight),
}
