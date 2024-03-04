use serde::{Deserialize, Serialize};
use crate::game_content::entity::*;

pub const TILE_SIZE: usize = 20;
pub const SCREEN_WIDTH: usize = 800;
pub const SCREEN_HEIGHT: usize = 600;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Default,
    Deserialize,
    Serialize,
)]
pub enum EntityType {
    #[default]
    None,
    Player(Entity),
    Npc(Entity),
}