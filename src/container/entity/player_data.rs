use std::collections::VecDeque;

use graphics::*;
use mmap_bytey::{MByteBuffer, MByteBufferRead, MByteBufferWrite};

use crate::{
    AttackFrame, Attacking, DeathType, EntityName, EntityNameMap, Equipment,
    HPBar, Movement, MovementData, Physical, Position, SpriteImage,
    SpriteIndex, Vitals, content::PlayerPvP,
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
    pub finalized: bool,
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
