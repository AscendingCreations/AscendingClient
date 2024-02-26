use graphics::*;

use indexmap::IndexSet;

pub mod content_input;
pub mod interface;

pub use content_input::*;
pub use interface::*;

use crate::{
    gfx_order::*,
    logic::*,
    Direction,
    DrawSetting,
    values::TILE_SIZE,
    content::*,
    database::*,
};
use hecs::World;

mod player;
mod map;
mod camera;
pub mod entity;

use player::*;
use map::*;
use camera::*;
pub use entity::*;

pub struct GameContent {
    players: IndexSet<Entity>,
    map: MapContent,
    camera: Camera,
    interface: Interface,
    // Test
    myentity: Option<Entity>,
    otherentity: Option<Entity>,
}

impl GameContent {
    pub fn new(world: &mut World, systems: &mut DrawSetting) -> Self {
        let mut content = GameContent {
            players: IndexSet::default(),
            map: MapContent::new(systems),
            camera: Camera::new(Vec2::new(0.0, 0.0)),
            interface: Interface::new(systems),

            myentity: None,
            otherentity: None,
        };

        // TEMP //
        let player = add_player(world, systems, Vec2::new(0.0, 0.0));
        content.myentity = Some(player.clone());
        content.players.insert(player);
        let otherplayer = add_player(world, systems, Vec2::new(3.0, 2.0));
        content.otherentity = Some(otherplayer.clone());
        content.players.insert(otherplayer);
        // ---

        update_camera(world, &mut content, systems);

        content
    }

    pub fn unload(&mut self, world: &mut World, systems: &mut DrawSetting) {
        for entity in self.players.iter() {
            unload_player(world, systems, entity);
        }
        self.myentity = None;
        self.otherentity = None;
        self.interface.unload(systems);
        self.map.unload(systems);
    }

    pub fn setup_map(&mut self, systems: &mut DrawSetting, database: &mut Database) {
        for i in 0..9 {
            load_map_data(systems, &database.map[i], self.map.index[i]);
        }
    }

    // TEMP //
    pub fn move_player(
        &self,
        world: &mut World,
        systems: &mut DrawSetting,
        dir: &Direction,
    ) {
        let myentity = self.myentity.expect("Could not find myentity");
        move_player(world, systems, &myentity, &dir);
    }
    pub fn move_other_player(
        &self,
        world: &mut World,
        systems: &mut DrawSetting,
        dir: &Direction,
    ) {
        let otherentity = self.otherentity.expect("Could not find myentity");
        move_player(world, systems, &otherentity, &dir);
    }
    // ---
}

pub fn update_player(world: &mut World, systems: &mut DrawSetting, content: &mut GameContent) {
    let players = content.players.clone();
    for entity in players.iter() {
        if world.get_or_panic::<Movement>(&entity).is_moving {
            process_player_movement(world, systems, entity);
        }
    }
}

pub fn update_camera(world: &mut World, content: &mut GameContent, systems: &mut DrawSetting) {
    let player_pos = if let Some(entity) = content.myentity {
        let pos = world.get_or_panic::<Position>(&entity);
        (pos.pos * TILE_SIZE as f32) + pos.offset
    } else {
        Vec2::new(0.0, 0.0)
    };
    let adjust_pos = get_screen_center(&systems.size) - player_pos;
    content.camera.pos = adjust_pos;

    content.map.move_pos(systems, content.camera.pos);
    
    for entity in content.players.iter() {
        update_player_position(world, systems, &content.camera, entity);
    }
}
