use graphics::*;

use indexmap::IndexSet;

pub mod content_input;
pub mod interface;

pub use content_input::*;
pub use interface::*;

use crate::{
    values::*,
    logic::*,
    Direction,
    DrawSetting,
    content::*,
    database::*,
};
use hecs::World;

mod player;
mod npc;
pub mod map;
mod camera;
pub mod entity;

use player::*;
use npc::*;
pub use map::*;
use camera::*;
pub use entity::*;

const KEY_ATTACK: usize = 0;
const KEY_MOVEUP: usize = 1;
const KEY_MOVELEFT: usize = 2;
const KEY_MOVEDOWN: usize = 3;
const KEY_MOVERIGHT: usize = 4;
const MAX_KEY: usize = 5;

pub struct GameContent {
    players: IndexSet<Entity>,
    npcs: IndexSet<Entity>,
    mapitems: IndexSet<Entity>,
    pub map: MapContent,
    camera: Camera,
    interface: Interface,
    keyinput: [bool; MAX_KEY],
    // Test
    myentity: Option<Entity>,
}

impl GameContent {
    pub fn new(systems: &mut DrawSetting) -> Self {
        GameContent {
            players: IndexSet::default(),
            npcs: IndexSet::default(),
            mapitems: IndexSet::default(),
            map: MapContent::new(systems),
            camera: Camera::new(Vec2::new(0.0, 0.0)),
            interface: Interface::new(systems),
            keyinput: [false; MAX_KEY],

            myentity: None,
        }
    }

    pub fn unload(&mut self, world: &mut World, systems: &mut DrawSetting) {
        for entity in self.players.iter() {
            unload_player(world, systems, entity);
        }
        for entity in self.npcs.iter() {
            unload_npc(world, systems, entity);
        }
        self.myentity = None;
        self.interface.unload(systems);
        self.map.unload(systems);
    }

    pub fn init_map(&mut self, systems: &mut DrawSetting, database: &mut Database, map: MapPosition) {
        self.map.map_pos = map;

        self.map.map_attribute.clear();
        for i in 0..9 {
            load_map_data(systems, &database.map[i], self.map.index[i].0);

            self.map.map_attribute.push(
                MapAttributes {
                    attribute: database.map[i].attribute.clone(),
                }
            )
        }
    }

    pub fn move_map(&mut self, systems: &mut DrawSetting, database: &mut Database, dir: Direction) {
        /*1 => Vec2::new(map_size.x * -1.0, map_size.y * -1.0), // Top Left
        2 => Vec2::new(0.0, map_size.y * -1.0), // Top
        3 => Vec2::new(map_size.x, map_size.y * -1.0), // Top Right
        4 => Vec2::new(map_size.x * -1.0, 0.0), // Left
        5 => Vec2::new(map_size.x, 0.0), // Right
        6 => Vec2::new(map_size.x * -1.0, map_size.y), // Bottom Left
        7 => Vec2::new(0.0, map_size.y), // Bottom
        8 => Vec2::new(map_size.x, map_size.y), // Bottom Right
        _ => Vec2::new(0.0, 0.0), // Center*/

        match dir {
            Direction::Down => {
                self.map.index[0].1 = 2; // Center to Top
                self.map.index[4].1 = 1; // Left to Top Left
                self.map.index[5].1 = 3; // Right to Top Right
                self.map.index[7].1 = 0; // Bottom to Center
                self.map.index[6].1 = 4; // Bottom Left to Left
                self.map.index[8].1 = 5; // Bottom Right to Right

                //load_map_data(systems, &database.map[i], self.map.index[i].0);
                self.map.index[1].1 = 6;
                self.map.index[2].1 = 7;
                self.map.index[3].1 = 8;
            }
            Direction::Left => {

            }
            Direction::Right => {

            }
            Direction::Up => {

            }
        }
    }

    pub fn handle_key_input(&mut self, world: &mut World, systems: &mut DrawSetting, seconds: f32) {
        for i in 0..MAX_KEY {
            if self.keyinput[i] {
                match i {
                    KEY_ATTACK => self.player_attack(world, systems, seconds),
                    KEY_MOVEDOWN => self.move_player(world, systems, &Direction::Down),
                    KEY_MOVELEFT => self.move_player(world, systems, &Direction::Left),
                    KEY_MOVEUP => self.move_player(world, systems, &Direction::Up),
                    KEY_MOVERIGHT => self.move_player(world, systems, &Direction::Right),
                    _ => {}
                }
            }
        }
    }

    pub fn spawn_item(
        &mut self,
        world: &mut World,
        systems: &mut DrawSetting,
        pos: Position,
        cur_map: MapPosition,
        sprite: usize,
    ) {
        let entity = MapItem::new(world, systems, sprite, pos, cur_map);
        self.mapitems.insert(entity);
    }

    // TEMP //
    pub fn init_data(
        &mut self,
        world: &mut World,
        systems: &mut DrawSetting,
    ) {
        // TEMP //
        let player = add_player(world, systems,
            Position {
                x: 1,
                y: 1,
                map: MapPosition {
                    x: 0,
                    y: 0,
                    group: 0,
                },
            },
            self.map.map_pos,
        );
        self.myentity = Some(player.clone());
        self.players.insert(player);
        let npcentity = add_npc(world, systems,
            Position {
                x: 3,
                y: 2,
                map: MapPosition {
                    x: 0,
                    y: 0,
                    group: 0,
                },
            },
            self.map.map_pos,
        );
        self.npcs.insert(npcentity);
        self.spawn_item(world, systems, 
            Position {
                x: 0,
                y: 3,
                map: MapPosition {
                    x: 0,
                    y: 0,
                    group: 0,
                },
            },
            self.map.map_pos,
            1,
        );
        self.spawn_item(world, systems, 
            Position {
                x: 0,
                y: 29,
                map: MapPosition {
                    x: 0,
                    y: -1,
                    group: 0,
                },
            },
            self.map.map_pos,
            0,
        );
        // ---

        update_camera(world, self, systems);
    }
    pub fn move_player(
        &mut self,
        world: &mut World,
        systems: &mut DrawSetting,
        dir: &Direction,
    ) {
        let myentity = self.myentity.expect("Could not find myentity");
        move_player(world, systems, &myentity, self, &dir);
    }
    pub fn player_attack(
        &mut self,
        world: &mut World,
        systems: &mut DrawSetting,
        seconds: f32,
    ) {
        let myentity = self.myentity.expect("Could not find myentity");
        init_player_attack(world, systems, &myentity, seconds);
    }
    // ---
}

pub fn update_player(world: &mut World, systems: &mut DrawSetting, content: &mut GameContent, seconds: f32) {
    let players = content.players.clone();
    for entity in players.iter() {
        process_player_movement(world, systems, entity);
        process_player_attack(world, systems, entity, seconds)
    }
}

pub fn update_npc(world: &mut World, systems: &mut DrawSetting, content: &mut GameContent, seconds: f32) {
    let npcs = content.npcs.clone();
    for entity in npcs.iter() {
        process_npc_movement(world, systems, entity);
        process_npc_attack(world, systems, entity, seconds)
    }
}

pub fn update_camera(world: &mut World, content: &mut GameContent, systems: &mut DrawSetting) {
    let player_pos = if let Some(entity) = content.myentity {
        let pos_offset = world.get_or_panic::<PositionOffset>(&entity);
        let pos = world.get_or_panic::<Position>(&entity);
        (Vec2::new(pos.x as f32, pos.y as f32) * TILE_SIZE as f32) + pos_offset.offset
    } else {
        Vec2::new(0.0, 0.0)
    };
    let adjust_pos = get_screen_center(&systems.size) - player_pos;
    content.camera.pos = adjust_pos;

    content.map.move_pos(systems, content.camera.pos);
    
    for entity in content.players.iter() {
        update_player_position(world, systems, &content, entity);
    }
    for entity in content.npcs.iter() {
        update_npc_position(world, systems, &content, entity);
    }
    for entity in content.mapitems.iter() {
        update_mapitem_position(world, systems, &content, entity);
    }
}