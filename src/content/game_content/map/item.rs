use bytey::{ByteBufferRead, ByteBufferWrite};
use graphics::*;

use serde::{Deserialize, Serialize};

use crate::{
    Item, MapItemEntity, SystemHolder,
    data_types::*,
    game_content::{Camera, *},
    get_start_map_pos,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MapItem {
    pub id: GlobalKey,
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
        entity: GlobalKey,
    ) -> Result<GlobalKey> {
        let start_pos = get_start_map_pos(cur_map, pos.map)
            .unwrap_or_else(|| Vec2::new(0.0, 0.0));
        let texture_pos =
            Vec2::new(pos.x as f32, pos.y as f32) * TILE_SIZE as f32;
        let image = Image::new(
            Some(systems.resource.items[sprite].allocation),
            &mut systems.renderer,
            Vec3::new(
                start_pos.x + texture_pos.x,
                start_pos.y + texture_pos.y,
                ORDER_MAP_ITEM,
            ),
            Vec2::new(20.0, 20.0),
            Vec4::new(0.0, 0.0, 20.0, 20.0),
            0,
        );

        let sprite_index = systems.gfx.add_image(
            image,
            0,
            "Map Item",
            false,
            CameraView::SubView1,
        );

        let _ = world.kinds.insert(entity, EntityKind::MapItem);
        let _ = world.entities.insert(
            entity,
            Entity::MapItem(Box::new(MapItemEntity {
                pos,
                sprite_image: sprite as u16,
                sprite_index,
                ..Default::default()
            })),
        );

        Ok(entity)
    }

    pub fn finalized(
        world: &mut World,
        systems: &mut SystemHolder,
        entity: GlobalKey,
    ) -> Result<()> {
        if let Some(Entity::MapItem(i_data)) = world.entities.get(entity) {
            Self::finalized_data(systems, i_data.sprite_index);
        }
        Ok(())
    }

    pub fn finalized_data(systems: &mut SystemHolder, sprite: GfxType) {
        systems.gfx.set_visible(&sprite, true);
    }
}

pub fn update_mapitem_position(
    systems: &mut SystemHolder,
    content: &GameContent,
    sprite: GfxType,
    pos: &Position,
    pos_offset: Vec2,
    light_key: Option<Index>,
) {
    let start_pos = get_start_map_pos(content.map.map_pos, pos.map)
        .unwrap_or_else(|| {
            Vec2::new(systems.size.width * 2.0, systems.size.height * 2.0)
        });
    let cur_pos = systems.gfx.get_pos(&sprite);
    let texture_pos = content.camera.0
        + (Vec2::new(pos.x as f32, pos.y as f32) * TILE_SIZE as f32)
        + pos_offset;
    if start_pos + texture_pos == Vec2::new(cur_pos.x, cur_pos.y) {
        return;
    }

    let pos = start_pos + texture_pos;

    systems
        .gfx
        .set_pos(&sprite, Vec3::new(pos.x, pos.y, cur_pos.z));

    if let Some(light) = light_key {
        systems.gfx.set_area_light_pos(
            &content.game_lights,
            light,
            pos + TILE_SIZE as f32,
        )
    }
}

pub fn unload_mapitems(
    world: &mut World,
    systems: &mut SystemHolder,
    content: &GameContent,
    entity: GlobalKey,
) -> Result<()> {
    if let Some(Entity::MapItem(i_data)) = world.entities.get(entity) {
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, &i_data.sprite_index);
        if let Some(entitylight) = i_data.light {
            systems
                .gfx
                .remove_area_light(&content.game_lights, entitylight);
        }
    }

    let _ = world.entities.remove(entity);
    let _ = world.kinds.remove(entity);
    Ok(())
}
