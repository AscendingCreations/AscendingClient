use std::collections::VecDeque;

use graphics::*;

use crate::{
    AttackFrame, Attacking, DeathType, EntityName, EntityNameMap, HPBar,
    LightData, Movement, MovementData, Physical, Position, SpriteImage,
    SpriteIndex, Vitals,
};
use mmap_bytey::{MByteBuffer, MByteBufferRead, MByteBufferWrite};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default)]
pub struct NpcEntity {
    // General
    pub entity_index: u64,
    pub entity_name: EntityName,
    pub mode: NpcMode,

    // Map
    pub hp_bar: HPBar,
    pub name_map: EntityNameMap,
    pub light: Option<Index>,
    pub light_data: LightData,
    pub finalized: bool,
    pub visible: bool,

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

#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Default,
    MByteBufferRead,
    MByteBufferWrite,
)]
pub enum NpcMode {
    None,
    #[default]
    Normal,
    Pet,
    Summon,
    Boss,
}

#[derive(Copy, Clone, Debug, Default, MByteBufferRead, MByteBufferWrite)]
pub struct NpcIndex(pub u64);
