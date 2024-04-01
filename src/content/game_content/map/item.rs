use bytey::{ByteBufferRead, ByteBufferWrite};
use graphics::*;
use hecs::World;
use serde::{Deserialize, Serialize};

use crate::{
    game_content::{entity::*, Camera, *},
    get_start_map_pos,
    values::*,
    SystemHolder,
};

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
    Default,
)]
pub struct Item {
    pub num: u32,
    pub val: u16,
    pub level: u8,
    pub data: [i16; 5],
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, ByteBufferRead, ByteBufferWrite,
)]
pub struct MapItem {
    pub id: Entity,
    pub item: Item,
    pub pos: Position,
}

impl MapItem {
    pub fn create(
        world: &mut World,
        systems: &mut SystemHolder,
        sprite: usize,
        pos: Position,
        cur_map: MapPosition,
        entity: Option<&Entity>,
    ) -> Entity {
        let start_pos = get_start_map_pos(cur_map, pos.map)
            .unwrap_or_else(|| Vec2::new(0.0, 0.0));
        println!("Pos {:?}", start_pos);
        let mut image = Image::new(
            Some(systems.resource.items[sprite].allocation),
            &mut systems.renderer,
            0,
        );
        let texture_pos =
            Vec2::new(pos.x as f32, pos.y as f32) * TILE_SIZE as f32;
        image.pos = Vec3::new(
            start_pos.x + texture_pos.x,
            start_pos.y + texture_pos.y,
            ORDER_MAP_ITEM,
        );
        image.uv = Vec4::new(0.0, 0.0, 20.0, 20.0);
        image.hw = Vec2::new(20.0, 20.0);
        let index = systems.gfx.add_image(image, 0);
        systems.gfx.set_visible(index, false);

        let component1 = (
            pos,
            WorldEntityType::MapItem,
            SpriteIndex(index),
            SpriteImage(sprite as u8),
            PositionOffset::default(),
            Finalized::default(),
        );

        if let Some(data) = entity {
            world.spawn_at(data.0, component1);
            let _ =
                world.insert_one(data.0, EntityType::MapItem(Entity(data.0)));
            Entity(data.0)
        } else {
            let entity = world.spawn(component1);
            let _ =
                world.insert_one(entity, EntityType::MapItem(Entity(entity)));
            Entity(entity)
        }
    }

    pub fn finalized(
        world: &mut World,
        systems: &mut SystemHolder,
        entity: &Entity,
    ) -> Result<()> {
        if !world.contains(entity.0) {
            return Ok(());
        }
        let sprite = world.get_or_err::<SpriteIndex>(entity)?.0;
        Self::finalized_data(systems, sprite);
        Ok(())
    }

    pub fn finalized_data(systems: &mut SystemHolder, sprite: usize) {
        systems.gfx.set_visible(sprite, true);
    }
}

pub fn update_mapitem_position(
    systems: &mut SystemHolder,
    content: &GameContent,
    sprite: usize,
    pos: &Position,
    pos_offset: &PositionOffset,
) {
    let start_pos = get_start_map_pos(content.map.map_pos, pos.map)
        .unwrap_or_else(|| Vec2::new(0.0, 0.0));
    let cur_pos = systems.gfx.get_pos(sprite);
    let texture_pos = content.camera.pos
        + (Vec2::new(pos.x as f32, pos.y as f32) * TILE_SIZE as f32)
        + pos_offset.offset;
    if start_pos + texture_pos == Vec2::new(cur_pos.x, cur_pos.y) {
        return;
    }
    systems.gfx.set_pos(
        sprite,
        Vec3::new(
            start_pos.x + texture_pos.x,
            start_pos.y + texture_pos.y,
            cur_pos.z,
        ),
    );
}

pub fn unload_mapitems(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: &Entity,
) -> Result<()> {
    let item_sprite = world.get_or_err::<SpriteIndex>(entity)?.0;
    systems.gfx.remove_gfx(item_sprite);
    let _ = world.despawn(entity.0);
    Ok(())
}
