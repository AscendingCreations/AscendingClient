use graphics::*;
use bytey::{ByteBufferRead, ByteBufferWrite};
use hecs::World;
use serde::{Deserialize, Serialize};

use crate::{game_content::Camera, get_start_map_pos, DrawSetting, Entity, EntityType, GameContent, MapPosition, Position, PositionOffset, Sprite, WorldEntityType, WorldExtras, ORDER_MAP_ITEM, TILE_SIZE};

#[derive(
    Debug,
    Copy,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    ByteBufferWrite,
    ByteBufferRead,
)]
pub struct Item {
    pub num: u32,
    pub val: u16,
    pub level: u8,
    pub data: [i16; 5],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ByteBufferRead, ByteBufferWrite)]
pub struct MapItem {
    pub id: Entity,
    pub item: Item,
    pub pos: Position,
}

impl MapItem {
    pub fn new(
        world: &mut World,
        systems: &mut DrawSetting, 
        sprite: usize,
        pos: Position,
        cur_map: MapPosition,
    ) -> Entity {
        let start_pos = get_start_map_pos(cur_map, pos.map).unwrap_or_else(|| Vec2::new(0.0, 0.0));
        println!("Pos {:?}", start_pos);
        let mut image = Image::new(Some(systems.resource.items[sprite].allocation),
            &mut systems.renderer, 0);
        let texture_pos = Vec2::new(pos.x as f32, pos.y as f32) * TILE_SIZE as f32;
        image.pos = Vec3::new(start_pos.x + texture_pos.x, start_pos.y + texture_pos.y, ORDER_MAP_ITEM);
        image.uv = Vec4::new(0.0, 0.0, 20.0, 20.0);
        image.hw = Vec2::new(20.0, 20.0);
        let index = systems.gfx.add_image(image, 0);

        let entity = world.spawn((
            pos,
            WorldEntityType::MapItem,
            Sprite(index),
            PositionOffset::default(),
        ));
        let _ = world.insert_one(entity, EntityType::MapItem(Entity(entity)));
        Entity(entity)
    }
}

pub fn update_mapitem_position(
    systems: &mut DrawSetting,
    content: &GameContent,
    sprite: usize,
    pos: &Position,
    pos_offset: &PositionOffset,
) {
    let start_pos = get_start_map_pos(content.map.map_pos, pos.map).unwrap_or_else(|| Vec2::new(0.0, 0.0));
    let cur_pos = systems.gfx.get_pos(sprite);
    let texture_pos = content.camera.pos + 
        (Vec2::new(pos.x as f32, pos.y as f32) * TILE_SIZE as f32) + pos_offset.offset;
    if start_pos + texture_pos == Vec2::new(cur_pos.x, cur_pos.y) {
        return;
    }
    systems.gfx.set_pos(sprite,
        Vec3::new(start_pos.x + texture_pos.x, 
                start_pos.y + texture_pos.y,
                cur_pos.z));
}