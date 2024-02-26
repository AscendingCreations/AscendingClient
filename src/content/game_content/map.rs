use graphics::*;

use crate::{
    DrawSetting,
    gfx_order::*,
    values::TILE_SIZE,
};

#[derive(Clone, Debug)]
pub struct MapContent {
    pub index: [usize; 9],
}

impl MapContent {
    pub fn new(systems: &mut DrawSetting) -> Self {
        let mut index = [0; 9];
        for i in 0..9 {
            let mut mapdata = Map::new(&mut systems.renderer, TILE_SIZE as u32);
            mapdata.pos = get_mapindex_base_pos(i);
            mapdata.can_render = true;
            index[i] = systems.gfx.add_map(mapdata, 0);
        }

        Self {
            index,
        }
    }

    pub fn unload(&mut self, systems: &mut DrawSetting) {
        self.index.iter().for_each(|index| {
            systems.gfx.remove_gfx(*index);
        });
    }

    pub fn move_pos(&mut self, systems: &mut DrawSetting, pos: Vec2) {
        self.index.iter().enumerate().for_each(|(index, map_index)| {
            let add_pos = get_mapindex_base_pos(index);
            systems.gfx.set_pos(*map_index,
                Vec3::new(add_pos.x + pos.x, add_pos.y + pos.y, 0.0));
        });
    }

    pub fn get_pos(&mut self, systems: &mut DrawSetting) -> Vec2 {
        let pos = systems.gfx.get_pos(self.index[0]);
        Vec2::new(pos.x, pos.y)
    }
}

pub fn get_world_pos(tile_pos: Vec2) -> Vec2 {
    tile_pos * TILE_SIZE as f32
}

pub fn get_mapindex_base_pos(index: usize) -> Vec2 {
    let map_size = Vec2::new(32.0 * TILE_SIZE as f32, 32.0 * TILE_SIZE as f32);
    match index {
        1 => Vec2::new(map_size.x * -1.0, map_size.y * -1.0), // Top Left
        2 => Vec2::new(0.0, map_size.y * -1.0), // Top
        3 => Vec2::new(map_size.x, map_size.y * -1.0), // Top Right
        4 => Vec2::new(map_size.x * -1.0, 0.0), // Left
        5 => Vec2::new(map_size.x, 0.0), // Right
        6 => Vec2::new(map_size.x * -1.0, map_size.y), // Bottom Left
        7 => Vec2::new(0.0, map_size.y), // Bottom
        8 => Vec2::new(map_size.x, map_size.y), // Bottom Right
        _ => Vec2::new(0.0, 0.0), // Center
    }
}