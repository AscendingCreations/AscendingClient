use bytey::{ByteBufferRead, ByteBufferWrite};
use graphics::*;

use serde::{Deserialize, Serialize};

use crate::{
    Item, MapItemEntity, SystemHolder, data_types::*, game_content::*,
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
        entity: GlobalKey,
    ) -> Result<GlobalKey> {
        let start_pos = get_map_pos(systems, pos.map);
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
            CameraView::MainView,
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
        game_light: GfxType,
        update_position: bool,
    ) -> Result<()> {
        if let Some(Entity::MapItem(i_data)) = world.entities.get(entity) {
            Self::finalized_data(systems, i_data.sprite_index);

            if update_position {
                update_mapitem_position(
                    systems,
                    game_light,
                    i_data.sprite_index,
                    &i_data.pos,
                    i_data.pos_offset,
                    i_data.light,
                )?;
            }
        }
        Ok(())
    }

    pub fn finalized_data(systems: &mut SystemHolder, sprite: GfxType) {
        systems.gfx.set_visible(&sprite, true);
    }
}

pub fn update_mapitem_position(
    systems: &mut SystemHolder,
    game_light: GfxType,
    sprite: GfxType,
    pos: &Position,
    pos_offset: Vec2,
    light_key: Option<Index>,
) -> Result<()> {
    let start_pos = get_map_pos(systems, pos.map);
    let cur_pos = systems.gfx.get_pos(&sprite);
    let texture_pos = start_pos
        + (Vec2::new(pos.x as f32, pos.y as f32) * TILE_SIZE as f32)
        + pos_offset;
    if texture_pos == Vec2::new(cur_pos.x, cur_pos.y) {
        return Ok(());
    }

    systems
        .gfx
        .set_pos(&sprite, Vec3::new(texture_pos.x, texture_pos.y, cur_pos.z));

    if let Some(light) = light_key {
        systems.gfx.set_area_light_pos(
            &game_light,
            light,
            texture_pos + TILE_SIZE as f32,
        )
    }
    Ok(())
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
