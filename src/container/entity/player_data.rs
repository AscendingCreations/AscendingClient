use std::collections::VecDeque;

use educe::Educe;
use graphics::*;
use mmap_bytey::{MByteBuffer, MByteBufferRead, MByteBufferWrite};
use serde::{Deserialize, Serialize};

use crate::{
    AttackFrame, Attacking, DeathType, EntityName, EntityNameMap, GlobalKey,
    HPBar, LightData, MAX_EQPT, Movement, MovementData, Physical, Position,
    SpriteImage, SpriteIndex, Vitals, content::PlayerPvP,
};

#[derive(Debug, Clone, Default)]
pub struct PlayerEntity {
    // General
    pub user_access: UserAccess,
    pub entity_name: EntityName,
    pub pvp: PlayerPvP,

    // Map
    pub hp_bar: HPBar,
    pub name_map: EntityNameMap,
    pub light: Option<Index>,
    pub light_data: LightData,
    pub finalized: bool,
    pub visible: bool,
    pub equipment: Equipment,

    // Appearance
    pub sprite: SpriteImage,
    pub sprite_index: SpriteIndex,

    // Frame
    pub last_move_frame: usize,
    pub attack_frame: AttackFrame,

    // Combat
    pub level: i32,
    pub vitals: Vitals,
    pub death_type: DeathType,
    pub attacking: Attacking,
    pub attack_timer: f32,
    pub physical: Physical,

    // Movement
    pub movement: Movement,
    pub end_movement: Position,
    pub movement_buffer: VecDeque<MovementData>,
    pub dir: u8,

    // Location
    pub pos: Position,
    pub pos_offset: Vec2,
}

#[derive(Copy, Clone, Debug, Default, MByteBufferRead, MByteBufferWrite)]
pub enum UserAccess {
    #[default]
    None,
    Monitor,
    Admin,
}

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Default,
    MByteBufferRead,
    MByteBufferWrite,
)]
pub enum IsUsingType {
    #[default]
    None,
    Bank,
    Fishing(i64),
    Crafting(i64),
    Trading(GlobalKey),
    Store(i64),
    Other(i64),
}

impl IsUsingType {
    pub fn inuse(self) -> bool {
        !matches!(self, IsUsingType::None)
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Educe,
    MByteBufferRead,
    MByteBufferWrite,
)]
#[educe(Default)]
pub struct Item {
    pub num: u32,
    pub val: u16,
    #[educe(Default = 1)]
    pub level: u8,
    pub data: [i16; 5],
}

#[derive(
    PartialEq,
    Eq,
    Clone,
    Debug,
    Educe,
    Deserialize,
    Serialize,
    MByteBufferRead,
    MByteBufferWrite,
)]
#[educe(Default)]
pub struct Equipment {
    #[educe(Default = (0..MAX_EQPT).map(|_| Item::default()).collect())]
    pub items: Vec<Item>,
}
