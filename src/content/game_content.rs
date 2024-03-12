use graphics::*;

use indexmap::IndexSet;

pub mod content_input;
pub mod interface;

pub use content_input::*;
pub use interface::*;

use crate::{
    content::*, database::*, logic::*, values::*, buffer::*, Direction, DrawSetting
};
use hecs::World;

pub mod player;
pub mod npc;
pub mod map;
mod camera;
pub mod entity;

pub use player::*;
pub use npc::*;
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
    pub myentity: Option<Entity>,
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

    pub fn show(&mut self, systems: &mut DrawSetting) {
        self.map.recreate(systems);
        self.interface.recreate(systems);
        self.keyinput.iter_mut().for_each(|key| {
            *key = false;
        });
    }

    pub fn hide(&mut self, world: &mut World, systems: &mut DrawSetting) {
        for entity in self.players.iter() {
            unload_player(world, systems, entity);
        }
        for entity in self.npcs.iter() {
            unload_npc(world, systems, entity);
        }
        for entity in self.mapitems.iter() {
            unload_mapitems(world, systems, entity);
        }
        self.players.clear();
        self.npcs.clear();
        self.mapitems.clear();
        
        self.myentity = None;
        self.interface.unload(systems);
        self.map.unload(systems);
    }

    pub fn init_map(&mut self, systems: &mut DrawSetting, map: MapPosition) {
        self.map.map_pos = map;

        self.map.map_attribute.clear();
        for i in 0..9 {
            let (mx, my) = get_map_loc(map.x, map.y, i);
            let mapdata = load_file(mx, my, map.group as u64);
            load_map_data(systems, &mapdata, self.map.index[i].0);

            self.map.map_attribute.push(
                (MapAttributes { attribute: mapdata.attribute.clone() }, i)
            )
        }
    }

    pub fn move_map(&mut self, world: &mut World, systems: &mut DrawSetting, dir: Direction, buffer: &mut BufferTask) {
        match dir {
            Direction::Down => self.map.map_pos.y -= 1,
            Direction::Left => self.map.map_pos.x -= 1,
            Direction::Right => self.map.map_pos.x += 1,
            Direction::Up => self.map.map_pos.y += 1,
        }
        
        let move_maps = match dir {
            Direction::Up => [(0, 2), (4, 1), (5, 3), (7, 0), (6, 4), (8, 5)],
            Direction::Left => [(0, 5), (2, 3), (7, 8), (1, 2), (4, 0), (6, 7)],
            Direction::Right => [(0, 4), (2, 1), (7, 6), (3, 2), (5, 0), (8, 7)],
            Direction::Down => [(0, 7), (4, 6), (5, 8), (2, 0), (1, 4), (3, 5)],
        };
        for (from, to) in move_maps {
            self.map.index[from].1 = to;
            self.map.map_attribute[from].1 = to;
        }

        let load_maps = match dir {
            Direction::Up => [(1, 6), (2, 7), (3, 8)],
            Direction::Left => [(3, 1), (5, 4), (8, 6)],
            Direction::Right => [(1, 3), (4, 5), (6, 8)],
            Direction::Down => [(6, 1), (7, 2), (8, 3)],
        };
        for (from, to) in load_maps {
            let (mx, my) = get_map_loc(
                self.map.map_pos.x, 
                self.map.map_pos.y, to);
            self.map.index[from].1 = to;
            self.map.map_attribute[from].1 = to;
            
            buffer.add_task(BufferTaskEnum::LoadMap(mx, my, self.map.map_pos.group as u64));
            buffer.add_task(BufferTaskEnum::ApplyMap(mx, my, self.map.map_pos.group as u64, to));
            buffer.add_task(BufferTaskEnum::ApplyMapAttribute(mx, my, self.map.map_pos.group as u64, to));
            buffer.add_task(BufferTaskEnum::UnloadMap(mx, my, self.map.map_pos.group as u64));   
        }

        self.map.sort_map();
        update_camera(world, self, systems);
    }

    pub fn handle_key_input(&mut self, world: &mut World, systems: &mut DrawSetting, seconds: f32) {
        for i in 0..MAX_KEY {
            if self.keyinput[i] {
                match i {
                    KEY_ATTACK => self.player_attack(world, systems, seconds),
                    KEY_MOVEDOWN => self.move_player(world, systems, &Direction::Down),
                    KEY_MOVELEFT => self.move_player(world, systems, &Direction::Left),
                    KEY_MOVEUP => self.move_player(world, systems,&Direction::Up),
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
        println!("Creating Temp Data");
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
        move_player(world, systems, &myentity, self, &dir, None);
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

pub fn update_player(world: &mut World, systems: &mut DrawSetting, content: &mut GameContent, buffer: &mut BufferTask, seconds: f32) {
    let players = content.players.clone();
    for entity in players.iter() {
        process_player_movement(world, systems, entity, content, buffer);
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
    
    for (_, (worldentitytype, sprite, pos, pos_offset)) in 
        world.query_mut::<(&WorldEntityType, &SpriteIndex, &Position, &PositionOffset)>().into_iter() {
        match worldentitytype {
            WorldEntityType::Player => {
                update_player_position(systems, &content, sprite.0, pos, pos_offset);
            }
            WorldEntityType::Npc => {
                update_npc_position(systems, &content, sprite.0, pos, pos_offset);
            }
            WorldEntityType::MapItem => {
                update_mapitem_position(systems, &content, sprite.0, pos, pos_offset);
            }
            _ => {}
        }
    }
}