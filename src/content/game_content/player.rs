use crate::{
    EntityNameMap, HPBar, PlayerEntity, Result, SpriteIndex, create_label,
};
use bytey::{ByteBufferError, ByteBufferRead, ByteBufferWrite};
use graphics::*;

pub const PLAYER_SPRITE_FRAME_X: f32 = 6.0;

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Default, ByteBufferRead, ByteBufferWrite,
)]
pub struct PlayerPvP {
    pub pk: bool,
    pub pvpon: bool,
}

use crate::{
    Direction, SystemHolder, data_types::*, fade::*, game_content::*, send_move,
};

pub fn add_player(
    world: &mut World,
    systems: &mut SystemHolder,
    pos: Position,
    cur_map: MapPosition,
    entity: GlobalKey,
    sprite: usize,
) -> Result<GlobalKey> {
    let start_pos = get_start_map_pos(cur_map, pos.map)
        .unwrap_or_else(|| Vec2::new(0.0, 0.0));
    let texture_pos = Vec2::new(pos.x as f32, pos.y as f32) * TILE_SIZE as f32;
    let mut image = Image::new(
        Some(systems.resource.players[sprite].allocation),
        &mut systems.renderer,
        0,
    );

    image.pos = Vec3::new(
        start_pos.x + texture_pos.x,
        start_pos.y + texture_pos.y,
        ORDER_PLAYER,
    );
    image.hw = Vec2::new(40.0, 40.0);
    image.uv = Vec4::new(0.0, 0.0, 40.0, 40.0);

    let sprite_index = systems.gfx.add_image(image, 0, "Player Sprite", false);
    let mut bg_image = Rect::new(&mut systems.renderer, 0);

    let screen_pos = pos.convert_to_screen_tile(cur_map);

    systems.gfx.set_override_pos(
        &sprite_index,
        Vec3::new(screen_pos.x as f32, screen_pos.y as f32, ORDER_PLAYER),
    );

    bg_image
        .set_size(Vec2::new(20.0, 6.0))
        .set_position(Vec3::new(0.0, 0.0, ORDER_HPBAR_BG))
        .set_color(Color::rgba(80, 80, 80, 255))
        .set_border_width(1.0)
        .set_border_color(Color::rgba(10, 10, 10, 255));

    let bg_index = systems.gfx.add_rect(bg_image, 0, "Player HP BG", false);
    let mut bar_image = Rect::new(&mut systems.renderer, 0);

    bar_image
        .set_size(Vec2::new(18.0, 4.0))
        .set_position(Vec3::new(1.0, 1.0, ORDER_HPBAR))
        .set_color(Color::rgba(180, 30, 30, 255));

    let bar_index = systems.gfx.add_rect(bar_image, 0, "Player HP Bar", false);
    let entity_name = create_label(
        systems,
        Vec3::new(0.0, 0.0, ORDER_ENTITY_NAME),
        Vec2::new(20.0, 20.0),
        Bounds::new(0.0, 0.0, systems.size.width, systems.size.height),
        Color::rgba(230, 230, 230, 255),
    );
    let name_index = systems.gfx.add_text(entity_name, 2, "Player Name", false);
    let name_map = EntityNameMap(name_index);
    let hp_bar = HPBar {
        visible: false,
        bg_index,
        bar_index,
    };

    let _ = world.kinds.insert(entity, EntityKind::Player);
    let _ = world.entities.insert(
        entity,
        Entity::Player(Box::new(PlayerEntity {
            pos,
            sprite_index: SpriteIndex(sprite_index),
            hp_bar,
            name_map,
            ..Default::default()
        })),
    );

    Ok(entity)
}

pub fn player_finalized(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: GlobalKey,
) -> Result<()> {
    if let Some(Entity::Player(p_data)) = world.entities.get_mut(entity) {
        player_finalized_data(
            systems,
            p_data.sprite_index.0,
            p_data.name_map.0,
            &p_data.hp_bar,
        );
    }
    Ok(())
}

pub fn player_finalized_data(
    systems: &mut SystemHolder,
    sprite: GfxType,
    name: GfxType,
    hpbar: &HPBar,
) {
    systems.gfx.set_visible(&sprite, true);
    systems.gfx.set_visible(&name, true);
    systems.gfx.set_visible(&hpbar.bg_index, hpbar.visible);
    systems.gfx.set_visible(&hpbar.bar_index, hpbar.visible);
}

pub fn unload_player(
    world: &mut World,
    systems: &mut SystemHolder,
    content: &GameContent,
    entity: GlobalKey,
) -> Result<()> {
    if let Some(Entity::Player(p_data)) = world.entities.get(entity) {
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, &p_data.sprite_index.0);
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, &p_data.hp_bar.bar_index);
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, &p_data.hp_bar.bg_index);

        systems
            .gfx
            .remove_gfx(&mut systems.renderer, &p_data.name_map.0);

        if let Some(entitylight) = p_data.light {
            systems
                .gfx
                .remove_area_light(&content.game_lights, entitylight);
        }
    }

    let _ = world.entities.remove(entity);
    let _ = world.kinds.remove(entity);
    Ok(())
}

pub fn move_player(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: GlobalKey,
    move_type: MovementType,
) -> Result<()> {
    if !world.entities.contains_key(entity) {
        return Ok(());
    }

    let (frame, last_frame) = if let Some(Entity::Player(p_data)) =
        world.entities.get_mut(entity)
    {
        if p_data.attacking.0 || p_data.movement.is_moving {
            return Ok(());
        }

        let (dir, end) = match move_type {
            MovementType::MovementBuffer => {
                if p_data.movement_buffer.is_empty() {
                    return Ok(());
                }
                let movement_data = p_data.movement_buffer.pop_front();

                if let Some(data) = movement_data {
                    (dir_to_enum(data.dir), Some(data.end_pos))
                } else {
                    return Ok(());
                }
            }
            MovementType::Manual(m_dir, m_end) => (dir_to_enum(m_dir), m_end),
        };

        if let Some(end_pos) = end {
            p_data.end_movement = end_pos;
        } else {
            let adj = [(0, -1), (1, 0), (0, 1), (-1, 0)];
            let dir_index = enum_to_dir(dir) as usize;
            let mut end_move = Position {
                x: p_data.pos.x + adj[dir_index].0,
                y: p_data.pos.y + adj[dir_index].1,
                map: p_data.pos.map,
            };

            if end_move.x < 0
                || end_move.x >= 32
                || end_move.y < 0
                || end_move.y >= 32
            {
                let new_pos = [
                    (end_move.x, 31),
                    (0, end_move.y),
                    (end_move.x, 0),
                    (31, end_move.y),
                ];

                end_move.x = new_pos[dir_index].0;
                end_move.y = new_pos[dir_index].1;
                end_move.map.x += adj[dir_index].0;
                end_move.map.y += adj[dir_index].1;
            }

            p_data.end_movement = end_move;
        }

        let dir_u8 = enum_to_dir(dir);

        p_data.movement.is_moving = true;
        p_data.movement.move_direction = dir;
        p_data.movement.move_offset = 0.0;
        p_data.movement.move_timer = 0.0;
        p_data.dir = dir_u8;

        let last_frame = if p_data.last_move_frame == 1 { 2 } else { 1 };

        p_data.last_move_frame = last_frame;

        (p_data.dir * PLAYER_SPRITE_FRAME_X as u8, last_frame)
    } else {
        return Ok(());
    };

    set_player_frame(world, systems, entity, frame as usize + last_frame)
}

pub fn end_player_move(
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut GameContent,
    socket: &mut Poller,
    entity: GlobalKey,
    buffer: &mut BufferTask,
) -> Result<()> {
    if !world.entities.contains_key(entity) {
        return Ok(());
    }

    let mut move_map: bool = false;

    let (direction, dir) =
        if let Some(Entity::Player(p_data)) = world.entities.get_mut(entity) {
            if !p_data.movement.is_moving {
                return Ok(());
            }
            p_data.movement.is_moving = false;
            p_data.movement.move_offset = 0.0;
            p_data.movement.move_timer = 0.0;

            p_data.pos.x = p_data.end_movement.x;
            p_data.pos.y = p_data.end_movement.y;
            if p_data.pos.map != p_data.end_movement.map {
                p_data.pos.map = p_data.end_movement.map;
                move_map = true;
            }

            p_data.pos_offset = Vec2::new(0.0, 0.0);

            (p_data.movement.move_direction, p_data.dir)
        } else {
            return Ok(());
        };

    if let Some(p) = &content.myentity {
        if *p == entity && move_map {
            content.move_map(world, systems, socket, direction, buffer)?;
            finalize_entity(world, systems)?;
            content.refresh_map = true;
        }
    }

    let frame = dir * PLAYER_SPRITE_FRAME_X as u8;
    set_player_frame(world, systems, entity, frame as usize)
}

pub fn update_player_position(
    systems: &mut SystemHolder,
    content: &mut GameContent,
    sprite: GfxType,
    pos: &Position,
    pos_offset: Vec2,
    hpbar: &HPBar,
    entitynamemap: &EntityNameMap,
    light_key: Option<Index>,
) -> Result<()> {
    let start_pos = get_start_map_pos(content.map.map_pos, pos.map)
        .unwrap_or_else(|| {
            Vec2::new(systems.size.width * 2.0, systems.size.height * 2.0)
        });
    let cur_pos = systems.gfx.get_pos(&sprite);
    let texture_pos = content.camera.0
        + (Vec2::new(pos.x as f32, pos.y as f32) * TILE_SIZE as f32)
        + pos_offset
        - Vec2::new(10.0, 4.0);
    let t_pos =
        Vec2::new(start_pos.x + texture_pos.x, start_pos.y + texture_pos.y);

    let screen_pos = pos.convert_to_screen_tile(content.map.map_pos);

    systems.gfx.set_override_pos(
        &sprite,
        Vec3::new(screen_pos.x as f32, screen_pos.y as f32, ORDER_PLAYER),
    );

    if t_pos == Vec2::new(cur_pos.x, cur_pos.y) {
        return Ok(());
    }

    systems
        .gfx
        .set_pos(&sprite, Vec3::new(t_pos.x, t_pos.y, cur_pos.z));

    if let Some(light) = light_key {
        systems.gfx.set_area_light_pos(
            &content.game_lights,
            light,
            t_pos + TILE_SIZE as f32,
        )
    }

    let sprite_size = systems.gfx.get_size(&sprite);
    let bar_pos =
        t_pos + Vec2::new(((sprite_size.x - 20.0) * 0.5).floor(), 0.0);

    systems.gfx.set_pos(
        &hpbar.bar_index,
        Vec3::new(bar_pos.x + 1.0, bar_pos.y + 1.0, ORDER_HPBAR),
    );
    systems.gfx.set_pos(
        &hpbar.bg_index,
        Vec3::new(bar_pos.x, bar_pos.y, ORDER_HPBAR_BG),
    );

    let textsize = systems.gfx.get_measure(&entitynamemap.0).floor();
    let name_pos =
        t_pos + Vec2::new(((sprite_size.x - textsize.x) * 0.5).floor(), 40.0);

    systems.gfx.set_pos(
        &entitynamemap.0,
        Vec3::new(name_pos.x, name_pos.y, ORDER_ENTITY_NAME),
    );
    Ok(())
}

pub fn set_player_frame(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: GlobalKey,
    frame_index: usize,
) -> Result<()> {
    if let Some(Entity::Player(p_data)) = world.entities.get(entity) {
        let size = systems.gfx.get_size(&p_data.sprite_index.0);
        let frame_pos = Vec2::new(
            frame_index as f32 % PLAYER_SPRITE_FRAME_X,
            (frame_index as f32 / PLAYER_SPRITE_FRAME_X).floor(),
        );

        systems.gfx.set_uv(
            &p_data.sprite_index.0,
            Vec4::new(
                size.x * frame_pos.x,
                size.y * frame_pos.y,
                size.x,
                size.y,
            ),
        );
    }
    Ok(())
}

pub fn init_player_attack(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: GlobalKey,
    seconds: f32,
) -> Result<()> {
    let frame =
        if let Some(Entity::Player(p_data)) = world.entities.get_mut(entity) {
            if p_data.attacking.0 || p_data.movement.is_moving {
                return Ok(());
            }

            p_data.attacking.0 = true;
            p_data.attack_timer = seconds + 0.5;
            p_data.attack_frame.frame = 0;
            p_data.attack_frame.timer = seconds + 0.16;

            p_data.dir * PLAYER_SPRITE_FRAME_X as u8
        } else {
            return Ok(());
        };

    set_player_frame(world, systems, entity, frame as usize + 3)
}

pub fn process_player_attack(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: GlobalKey,
    seconds: f32,
) -> Result<()> {
    let frame =
        if let Some(Entity::Player(p_data)) = world.entities.get_mut(entity) {
            if !p_data.attacking.0 {
                return Ok(());
            }

            if seconds < p_data.attack_timer {
                if seconds > p_data.attack_frame.timer {
                    p_data.attack_frame.frame += 1;
                    p_data.attack_frame.timer = seconds + 0.16;

                    let mut attackframe = p_data.attack_frame.frame;

                    if attackframe > 2 {
                        attackframe = 2;
                    }

                    let frame = p_data.dir * PLAYER_SPRITE_FRAME_X as u8;

                    frame as usize + 3 + attackframe
                } else {
                    return Ok(());
                }
            } else {
                p_data.attacking.0 = false;

                (p_data.dir * PLAYER_SPRITE_FRAME_X as u8) as usize
            }
        } else {
            return Ok(());
        };

    set_player_frame(world, systems, entity, frame)
}

pub fn process_player_movement(
    world: &mut World,
    systems: &mut SystemHolder,
    socket: &mut Poller,
    entity: GlobalKey,
    content: &mut GameContent,
    buffer: &mut BufferTask,
) -> Result<()> {
    if !world.entities.contains_key(entity) {
        return Ok(());
    }

    let movement =
        if let Some(Entity::Player(p_data)) = world.entities.get(entity) {
            p_data.movement
        } else {
            return Ok(());
        };

    if !movement.is_moving {
        return Ok(());
    };

    let add_offset = 4.0;

    if movement.move_offset + add_offset < TILE_SIZE as f32 {
        if let Some(Entity::Player(p_data)) = world.entities.get_mut(entity) {
            p_data.movement.move_offset += add_offset;

            let moveoffset = p_data.movement.move_offset;

            let offset = match movement.move_direction {
                Direction::Up => Vec2::new(0.0, moveoffset),
                Direction::Down => Vec2::new(0.0, -moveoffset),
                Direction::Left => Vec2::new(-moveoffset, 0.0),
                Direction::Right => Vec2::new(moveoffset, 0.0),
            };

            p_data.pos_offset = offset;
        }
    } else {
        if let Some(Entity::Player(p_data)) = world.entities.get_mut(entity) {
            p_data.pos_offset = Vec2::new(0.0, 0.0);
        }
        end_player_move(world, systems, content, socket, entity, buffer)?;
    }

    if let Some(myindex) = content.myentity {
        if myindex != entity {
            update_player_camera(world, systems, socket, entity, content)?;
        }
    }
    Ok(())
}

pub fn update_player_camera(
    world: &mut World,
    systems: &mut SystemHolder,
    socket: &mut Poller,
    entity: GlobalKey,
    content: &mut GameContent,
) -> Result<()> {
    if let Some(Entity::Player(p_data)) = world.entities.get_mut(entity) {
        update_player_position(
            systems,
            content,
            p_data.sprite_index.0,
            &p_data.pos,
            p_data.pos_offset,
            &p_data.hp_bar,
            &p_data.name_map,
            p_data.light,
        )?;

        let is_target = if let Some(target) = content.target.entity {
            target == entity
        } else {
            false
        };

        if is_target {
            if !p_data.hp_bar.visible {
                p_data.hp_bar.visible = true;
                systems.gfx.set_visible(&p_data.hp_bar.bar_index, true);
                systems.gfx.set_visible(&p_data.hp_bar.bg_index, true);
            }

            let pos = systems.gfx.get_pos(&p_data.sprite_index.0);

            content.target.set_target_pos(
                socket,
                systems,
                Vec2::new(pos.x, pos.y),
                &mut p_data.hp_bar,
            )?;
        } else if p_data.hp_bar.visible {
            p_data.hp_bar.visible = false;
            systems.gfx.set_visible(&p_data.hp_bar.bar_index, false);
            systems.gfx.set_visible(&p_data.hp_bar.bg_index, false);
        }
    }
    Ok(())
}

pub fn player_get_next_lvl_exp(
    world: &mut World,
    entity: GlobalKey,
) -> Result<u64> {
    if let Some(Entity::Player(p_data)) = world.entities.get(entity) {
        let exp_per_level = match p_data.level {
            1..=10 => 100,
            11..=20 => 250,
            21..=30 => 400,
            31..=40 => 550,
            41..=50 => 700,
            51..=60 => 850,
            61..=70 => 1000,
            71..=80 => 1150,
            81..=90 => 1300,
            91..=100 => 1450,
            101..=120 => 2000,
            121..=150 => 3000,
            151..=199 => 4000,
            _ => 0,
        };

        Ok(p_data.level as u64 * exp_per_level as u64)
    } else {
        Ok(0)
    }
}

pub fn create_player_light(
    world: &mut World,
    systems: &mut SystemHolder,
    game_light: &GfxType,
    entity: GlobalKey,
) {
    if let Some(Entity::Player(p_data)) = world.entities.get_mut(entity) {
        p_data.light = systems.gfx.add_area_light(game_light, AreaLight {
            pos: Vec2::new(0.0, 0.0),
            color: Color::rgba(100, 100, 100, 20),
            max_distance: 60.0,
            animate: true,
            anim_speed: 5.0,
            dither: 0.8,
            camera_type: CameraType::None,
        })
    }
}
