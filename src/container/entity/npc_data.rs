use std::collections::VecDeque;

use graphics::*;

use crate::{
    AttackFrame, Attacking, DeathType, EntityName, EntityNameMap, HPBar,
    Movement, MovementData, NpcMode, Physical, Position, SpriteImage,
    SpriteIndex, Vitals,
};

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
    pub finalized: bool,

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
