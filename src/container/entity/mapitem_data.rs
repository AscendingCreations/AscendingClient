use graphics::*;

use crate::{GfxType, Position};

#[derive(Debug, Clone, Default)]
pub struct MapItemEntity {
    // Map
    pub light: Option<Index>,
    pub finalized: bool,
    pub visible: bool,

    // Appearance
    pub sprite_index: GfxType,
    pub sprite_image: u16,

    // Location
    pub pos: Position,
    pub pos_offset: Vec2,
}
