use educe::Educe;
use graphics::*;
use mmap_bytey::{MByteBuffer, MByteBufferRead, MByteBufferWrite};
use serde::{Deserialize, Serialize};
use speedy::{Endianness, Readable, Writable};
use std::collections::VecDeque;

use crate::{Direction, GfxType, VITALS_MAX};

pub enum MovementType {
    MovementBuffer,
    Manual(u8, Option<Position>),
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Finalized(pub bool);

#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    Default,
    PartialEq,
    Eq,
    Readable,
    Writable,
    MByteBufferRead,
    MByteBufferWrite,
    Hash,
)]
pub struct MapPosition {
    pub x: i32,
    pub y: i32,
    pub group: i32,
}

impl MapPosition {
    pub fn checkdistance(&self, target: MapPosition) -> i32 {
        if self.group != target.group {
            return 2;
        }

        let x = self.x - target.x;
        let y = self.y - target.y;

        if x == 0 {
            return y.abs();
        }

        if y == 0 {
            return x.abs();
        }

        x.abs() + y.abs() - 1
    }
}

impl MapPosition {
    pub fn new(x: i32, y: i32, group: i32) -> Self {
        MapPosition { x, y, group }
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Readable,
    Writable,
    MByteBufferRead,
    MByteBufferWrite,
)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub map: MapPosition,
}

impl Position {
    pub fn convert_to_screen_tile(&self, cur_map: MapPosition) -> Self {
        let mut cur_pos = *self;

        cur_pos.x = match self.map.x.cmp(&cur_map.x) {
            std::cmp::Ordering::Greater => cur_pos.x + 64,
            std::cmp::Ordering::Less => cur_pos.x,
            std::cmp::Ordering::Equal => cur_pos.x + 32,
        };

        cur_pos.y = match self.map.y.cmp(&cur_map.y) {
            std::cmp::Ordering::Greater => cur_pos.y + 64,
            std::cmp::Ordering::Less => cur_pos.y,
            std::cmp::Ordering::Equal => cur_pos.y + 32,
        };

        cur_pos
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct PositionOffset {
    pub offset: Vec2,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct HPBar {
    pub visible: bool,
    pub bg_index: GfxType,
    pub bar_index: GfxType,
}

#[derive(Copy, Clone, Debug, Default, MByteBufferRead, MByteBufferWrite)]
pub struct Dir(pub u8);

#[derive(Copy, Clone, Debug, Default, MByteBufferRead, MByteBufferWrite)]
pub struct Level(pub i32);

#[derive(Copy, Clone, Debug, Default, MByteBufferRead, MByteBufferWrite)]
pub struct LastMoveFrame(pub usize);

#[derive(Copy, Clone, Debug, Default)]
pub struct SpriteIndex(pub GfxType);

#[derive(Copy, Clone, Debug, Default, MByteBufferRead, MByteBufferWrite)]
pub struct SpriteImage(pub u8);

#[derive(Copy, Clone, Debug, Default, MByteBufferRead, MByteBufferWrite)]
pub struct Attacking(pub bool);

#[derive(Copy, Clone, Debug, Default, MByteBufferRead, MByteBufferWrite)]
pub struct AttackTimer(pub f32);

#[derive(Copy, Clone, Debug, Default)]
pub struct EntityLight(pub Option<Index>);

#[derive(Copy, Clone, Debug, Default, MByteBufferRead, MByteBufferWrite)]
pub struct AttackFrame {
    pub frame: usize,
    pub timer: f32,
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Default,
    MByteBufferRead,
    MByteBufferWrite,
)]
pub struct Physical {
    pub damage: u32,
    pub defense: u32,
}

#[derive(
    Educe, Debug, Copy, Clone, PartialEq, Eq, MByteBufferRead, MByteBufferWrite,
)]
#[educe(Default)]
pub struct Vitals {
    #[educe(Default = [25, 2, 100])]
    pub vital: [i32; VITALS_MAX],
    #[educe(Default = [25, 2, 100])]
    pub vitalmax: [i32; VITALS_MAX],
    #[educe(Default = [0; VITALS_MAX])]
    pub vitalbuffs: [i32; VITALS_MAX],
    #[educe(Default = [0; VITALS_MAX])]
    pub regens: [u32; VITALS_MAX],
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Movement {
    pub is_moving: bool,
    pub move_direction: Direction,
    pub move_timer: f32,
    pub move_offset: f32,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct EndMovement(pub Position);

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct MovementData {
    pub end_pos: Position,
    pub dir: u8,
}

#[derive(Debug, Clone, Default)]
pub struct MovementBuffer {
    pub data: VecDeque<MovementData>,
}

#[derive(Debug, Clone, Default)]
pub struct EntityName(pub String);

#[derive(Debug, Clone, Copy, Default)]
pub struct EntityNameMap(pub GfxType);

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Default,
    Serialize,
    Deserialize,
    MByteBufferRead,
    MByteBufferWrite,
)]
pub enum DeathType {
    #[default]
    Alive,
    Spirit,
    Dead,
    UnSpawned,
    Spawning,
}
