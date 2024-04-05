use graphics::*;
use hecs::World;

use crate::{
    content::game_content::player::*, content::game_content::*,
    database::map::*, entity::*, values::*, Direction, EntityType,
    MapAttribute, SystemHolder, WorldExtras,
};

pub mod item;

pub use item::*;

const MAX_MAP_ITEMS: usize = 30;

#[derive(Clone, Debug)]
pub struct MapAttributes {
    pub attribute: Vec<MapAttribute>,
}

impl MapAttributes {
    pub fn default() -> Self {
        MapAttributes {
            attribute: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MapContent {
    pub map_pos: MapPosition,
    pub index: [(usize, usize); 9], // (MapIndex, Order)
    pub map_attribute: Vec<(MapAttributes, usize)>,
    pub music: Vec<(Option<String>, usize)>,
}

impl MapContent {
    pub fn new(systems: &mut SystemHolder) -> Self {
        let mut index = [(0, 0); 9];

        for (i, index) in index.iter_mut().enumerate() {
            let mut mapdata = Map::new(&mut systems.renderer, TILE_SIZE as u32);
            mapdata.pos = get_mapindex_base_pos(i);
            mapdata.can_render = true;
            *index = (systems.gfx.add_map(mapdata, 0), i);
        }

        Self {
            map_pos: MapPosition::default(),
            index,
            map_attribute: Vec::with_capacity(9),
            music: Vec::with_capacity(9),
        }
    }

    pub fn recreate(&mut self, systems: &mut SystemHolder) {
        for i in 0..9 {
            let mut mapdata = Map::new(&mut systems.renderer, TILE_SIZE as u32);
            mapdata.pos = get_mapindex_base_pos(i);
            mapdata.can_render = true;
            self.index[i] = (systems.gfx.add_map(mapdata, 0), i);
        }
        self.map_pos = MapPosition::default();
    }

    pub fn unload(&mut self, systems: &mut SystemHolder) {
        self.index.iter().for_each(|(index, _)| {
            systems.gfx.remove_gfx(*index);
        });
        self.map_attribute.clear();
        self.music.clear();
    }

    pub fn sort_map(&mut self) {
        self.index.sort_by(|a, b| a.1.cmp(&b.1));
        self.map_attribute.sort_by(|a, b| a.1.cmp(&b.1));
        self.music.sort_by(|a, b| a.1.cmp(&b.1));
    }

    pub fn move_pos(&mut self, systems: &mut SystemHolder, pos: Vec2) {
        self.index
            .iter()
            .enumerate()
            .for_each(|(index, (map_index, _))| {
                let add_pos = get_mapindex_base_pos(index);
                systems.gfx.set_pos(
                    *map_index,
                    Vec3::new(add_pos.x + pos.x, add_pos.y + pos.y, 0.0),
                );
            });
    }

    pub fn get_pos(&mut self, systems: &mut SystemHolder) -> Vec2 {
        let pos = systems.gfx.get_pos(self.index[0].0);
        Vec2::new(pos.x, pos.y)
    }

    pub fn get_attribute(
        &self,
        pos: Vec2,
        direction: &Direction,
    ) -> MapAttribute {
        let mut new_pos = match direction {
            Direction::Down => Vec2::new(pos.x, pos.y - 1.0),
            Direction::Left => Vec2::new(pos.x - 1.0, pos.y),
            Direction::Right => Vec2::new(pos.x + 1.0, pos.y),
            Direction::Up => Vec2::new(pos.x, pos.y + 1.0),
        };
        let map_index = match (
            new_pos.x < 0.0,
            new_pos.y < 0.0,
            new_pos.x >= 32.0,
            new_pos.y >= 32.0,
        ) {
            (true, true, _, _) => 1,
            (true, false, _, true) => 6,
            (true, false, _, _) => 4,
            (_, _, true, true) => 8,
            (_, true, true, _) => 3,
            (_, false, true, _) => 5,
            (_, true, _, _) => 2,
            (_, _, _, true) => 7,
            _ => 0,
        };
        if new_pos.x < 0.0 {
            new_pos.x = 31.0;
        }
        if new_pos.y < 0.0 {
            new_pos.y = 31.0;
        }
        if new_pos.x >= 32.0 {
            new_pos.x = 0.0;
        }
        if new_pos.y >= 32.0 {
            new_pos.y = 0.0;
        }
        let tile_num = get_tile_pos(new_pos.x as i32, new_pos.y as i32);
        self.map_attribute[map_index].0.attribute[tile_num].clone()
    }
}

pub fn find_entity(
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut GameContent,
    screen_pos: Vec2,
) -> Option<Entity> {
    let center_pos = systems.gfx.get_pos(content.map.index[0].0);
    let adjusted_pos =
        Vec2::new(screen_pos.x - center_pos.x, screen_pos.y - center_pos.y);
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

    let target_entity = world
        .query::<(&Position, &WorldEntityType)>()
        .iter()
        .find_map(|(entity, (pos, world_type))| {
            if *pos == target_pos
                && (*world_type == WorldEntityType::Npc
                    || *world_type == WorldEntityType::Player)
            {
                if let Some(myentity) = content.myentity {
                    if myentity.0 != entity {
                        Some(Entity(entity))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        });

    target_entity
}

pub fn can_move(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: &Entity,
    content: &mut GameContent,
    direction: &Direction,
) -> Result<bool> {
    let pos = world.get_or_err::<Position>(entity)?;
    {
        world.get::<&mut Dir>(entity.0)?.0 = match direction {
            Direction::Up => 2,
            Direction::Down => 0,
            Direction::Left => 3,
            Direction::Right => 1,
        };
    }
    let entity_type = world.get_or_err::<EntityType>(entity)?;
    match entity_type {
        EntityType::Player(_) => {
            let frame = world.get_or_err::<Dir>(entity)?.0
                * PLAYER_SPRITE_FRAME_X as u8;
            set_player_frame(world, systems, entity, frame as usize)?;
        }
        EntityType::Npc(_) => {
            let frame =
                world.get_or_err::<Dir>(entity)?.0 * NPC_SPRITE_FRAME_X as u8;
            set_npc_frame(world, systems, entity, frame as usize)?;
        }
        _ => {}
    }
    if content.player_data.is_using_type.inuse() {
        return Ok(false);
    }
    let attribute = content
        .map
        .get_attribute(Vec2::new(pos.x as f32, pos.y as f32), direction);
    Ok(!matches!(
        attribute,
        MapAttribute::Blocked | MapAttribute::Storage | MapAttribute::Shop(_)
    ))
}

pub fn get_world_pos(tile_pos: Vec2) -> Vec2 {
    tile_pos * TILE_SIZE as f32
}

pub fn get_mapindex_base_pos(index: usize) -> Vec2 {
    let map_size = Vec2::new(32.0 * TILE_SIZE as f32, 32.0 * TILE_SIZE as f32);
    match index {
        1 => Vec2::new(map_size.x * -1.0, map_size.y * -1.0), // Top Left
        2 => Vec2::new(0.0, map_size.y * -1.0),               // Top
        3 => Vec2::new(map_size.x, map_size.y * -1.0),        // Top Right
        4 => Vec2::new(map_size.x * -1.0, 0.0),               // Left
        5 => Vec2::new(map_size.x, 0.0),                      // Right
        6 => Vec2::new(map_size.x * -1.0, map_size.y),        // Bottom Left
        7 => Vec2::new(0.0, map_size.y),                      // Bottom
        8 => Vec2::new(map_size.x, map_size.y),               // Bottom Right
        _ => Vec2::new(0.0, 0.0),                             // Center
    }
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

pub fn get_start_map_pos(from: MapPosition, to: MapPosition) -> Option<Vec2> {
    if from.group != to.group {
        return None;
    }
    let from_vec = Vec2::new(from.x as f32, from.y as f32);
    let to_vec = Vec2::new(to.x as f32, to.y as f32);
    match from_vec {
        Vec2 { x, y } if x + 1.0 == to_vec.x && y + 1.0 == to_vec.y => {
            Some(get_mapindex_base_pos(8))
        } // Bottom Right
        Vec2 { x, y } if x - 1.0 == to_vec.x && y + 1.0 == to_vec.y => {
            Some(get_mapindex_base_pos(6))
        } // Bottom Left
        Vec2 { x, y } if x + 1.0 == to_vec.x && y - 1.0 == to_vec.y => {
            Some(get_mapindex_base_pos(3))
        } // Top Right
        Vec2 { x, y } if x - 1.0 == to_vec.x && y - 1.0 == to_vec.y => {
            Some(get_mapindex_base_pos(1))
        } // Top Left
        Vec2 { x, y } if x + 1.0 == to_vec.x && y == to_vec.y => {
            Some(get_mapindex_base_pos(5))
        } // Right
        Vec2 { x, y } if x - 1.0 == to_vec.x && y == to_vec.y => {
            Some(get_mapindex_base_pos(4))
        } // Left
        Vec2 { x, y } if x == to_vec.x && y + 1.0 == to_vec.y => {
            Some(get_mapindex_base_pos(7))
        } // Bottom
        Vec2 { x, y } if x == to_vec.x && y - 1.0 == to_vec.y => {
            Some(get_mapindex_base_pos(2))
        } // Top
        Vec2 { x, y } if x == to_vec.x && y == to_vec.y => {
            Some(get_mapindex_base_pos(0))
        } // Center
        _ => None,
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
