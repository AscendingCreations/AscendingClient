use graphics::*;

use crate::{
    DrawSetting,
    gfx_order::*,
    values::TILE_SIZE,
};

#[derive(Clone, Debug)]
pub struct MapContent {
    pub index: usize,
}

impl MapContent {
    pub fn new(systems: &mut DrawSetting) -> Self {
        // Create Map
        let mut mapdata = Map::new(&mut systems.renderer, TILE_SIZE as u32);
        mapdata.pos = Vec2::new(0.0, 0.0);
        mapdata.can_render = true;
        // TEMP
        (0..32).for_each(|x| {
            (0..32).for_each(|y| {
                mapdata.set_tile((x, y, 0),
                    TileData {
                        id: 12,
                        color: Color::rgba(255, 255, 255, 255),
                    });
            });
        });
        //
        let index = systems.gfx.add_map(mapdata, 0);

        Self {
            index,
        }
    }

    pub fn unload(&mut self, systems: &mut DrawSetting) {
        systems.gfx.remove_gfx(self.index);
    }

    pub fn move_pos(&mut self, systems: &mut DrawSetting, pos: Vec2) {
        systems.gfx.set_pos(self.index,
            Vec3::new(pos.x, pos.y, 0.0));
    }

    pub fn get_pos(&mut self, systems: &mut DrawSetting) -> Vec2 {
        let pos = systems.gfx.get_pos(self.index);
        Vec2::new(pos.x, pos.y)
    }
}

pub fn get_world_pos(tile_pos: Vec2) -> Vec2 {
    tile_pos * TILE_SIZE as f32
}