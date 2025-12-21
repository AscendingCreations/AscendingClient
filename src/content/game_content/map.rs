use bit_op::{BitOp, bit_u8::*};
use graphics::*;

use crate::{
    Direction, MapAttribute, SystemHolder, content::game_content::player::*,
    content::game_content::*, data_types::*, database::map::*,
};

pub mod item;

pub use item::*;

const MAX_MAP_ITEMS: usize = 30;
pub const MAP_SIZE: Vec2 = Vec2 {
    x: 640.0, // 32 x TEXTURE_SIZE
    y: 640.0, // 32 x TEXTURE_SIZE
};

#[derive(Clone, Debug, Default)]
pub struct MapAttributes {
    pub attribute: Vec<MapAttribute>,
}

impl MapAttributes {
    pub fn default() -> Self {
        MapAttributes {
            attribute: Vec::with_capacity(1024),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct MapDirBlock {
    pub dir: Vec<u8>,
}

impl MapDirBlock {
    pub fn default() -> Self {
        MapDirBlock {
            dir: Vec::with_capacity(1024),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct MapContent {
    pub map_pos: MapPosition,
    pub mapindex: [Index; 9],
}

impl MapContent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn recreate(&mut self) {
        self.mapindex = [Index::default(); 9];
        self.map_pos = MapPosition::default();
    }

    pub fn unload(
        &mut self,
        systems: &mut SystemHolder,
        map_renderer: &mut MapRenderer,
    ) {
        clear_map_data(systems, map_renderer);
    }
}

pub fn find_entity(
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut GameContent,
    screen_pos: Vec2,
) -> Option<GlobalKey> {
    let center_pos = get_map_render_pos(systems, content.map.map_pos);
    let adjusted_pos = screen_pos - center_pos;
    let tile_pos = Vec2::new(
        (adjusted_pos.x / 20.0).floor(),
        (adjusted_pos.y / 20.0).floor(),
    );
    let mut target_pos = Position {
        x: tile_pos.x as i32,
        y: tile_pos.y as i32,
        map: content.map.map_pos,
    };

    if target_pos.x >= 32 {
        target_pos.x -= 32;
        target_pos.map.x += 1;
    }

    if target_pos.y >= 32 {
        target_pos.y -= 32;
        target_pos.map.y += 1;
    }

    if target_pos.x < 0 {
        target_pos.x += 32;
        target_pos.map.x -= 1;
    }

    if target_pos.y < 0 {
        target_pos.y += 32;
        target_pos.map.y -= 1;
    }

    world.entities.iter().find_map(|(key, entity_data)| {
        match entity_data {
            Entity::Player(p_data) => {
                if p_data.pos == target_pos
                    && let Some(myentity) = content.myentity
                    && myentity != key
                {
                    return Some(key);
                }
            }

            Entity::Npc(n_data) => {
                if n_data.pos == target_pos {
                    return Some(key);
                }
            }
            _ => {}
        }
        None
    })
}

pub fn get_map_loc(mx: i32, my: i32, index: usize) -> (i32, i32) {
    match index {
        1 => (mx - 1, my - 1), // Top Left
        2 => (mx, my - 1),     // Top
        3 => (mx + 1, my - 1), // Top Right
        4 => (mx - 1, my),     // Left
        5 => (mx + 1, my),     // Right
        6 => (mx - 1, my + 1), // Bottom Left
        7 => (mx, my + 1),     // Bottom
        8 => (mx + 1, my + 1), // Bottom Right
        _ => (mx, my),         // Center
    }
}

pub fn is_map_connected(from: MapPosition, to: MapPosition) -> bool {
    if from.group != to.group {
        return false;
    }

    to.x >= from.x - 1
        && to.x <= from.x + 1
        && to.y >= from.y - 1
        && to.y <= from.y + 1
}
