use graphics::*;

pub const NPC_SPRITE_FRAME_X: f32 = 6.0;

use crate::{
    Direction, Result, SystemHolder, create_label, data_types::*,
    game_content::*,
};

pub fn add_npc(
    world: &mut World,
    systems: &mut SystemHolder,
    pos: Position,
    cur_map: MapPosition,
    entity: Option<GlobalKey>,
    npcnum: usize,
) -> Result<GlobalKey> {
    let npc_data = &systems.base.npc[npcnum];
    let start_pos = get_start_map_pos(cur_map, pos.map)
        .unwrap_or_else(|| Vec2::new(0.0, 0.0));
    let texture_pos = Vec2::new(pos.x as f32, pos.y as f32) * TILE_SIZE as f32;
    let mut image = Image::new(
        Some(systems.resource.npcs[npc_data.sprite as usize].allocation),
        &mut systems.renderer,
        0,
    );

    image.pos = Vec3::new(
        start_pos.x + texture_pos.x,
        start_pos.y + texture_pos.y,
        ORDER_NPC,
    );
    image.hw = Vec2::new(40.0, 40.0);
    image.uv = Vec4::new(0.0, 0.0, 40.0, 40.0);

    let sprite = systems.gfx.add_image(image, 0, "Npc Sprite", false);
    let mut bg_image = Rect::new(&mut systems.renderer, 0);

    bg_image
        .set_size(Vec2::new(20.0, 6.0))
        .set_position(Vec3::new(0.0, 0.0, ORDER_HPBAR_BG))
        .set_color(Color::rgba(80, 80, 80, 255))
        .set_border_width(1.0)
        .set_border_color(Color::rgba(10, 10, 10, 255));

    let bg_index = systems.gfx.add_rect(bg_image, 0, "Npc HP BG", false);
    let mut bar_image = Rect::new(&mut systems.renderer, 0);

    bar_image
        .set_size(Vec2::new(18.0, 4.0))
        .set_position(Vec3::new(1.0, 1.0, ORDER_HPBAR))
        .set_color(Color::rgba(180, 30, 30, 255));

    let bar_index = systems.gfx.add_rect(bar_image, 0, "Npc HP Bar", false);
    let entity_name = create_label(
        systems,
        Vec3::new(0.0, 0.0, ORDER_ENTITY_NAME),
        Vec2::new(20.0, 20.0),
        Bounds::new(0.0, 0.0, systems.size.width, systems.size.height),
        Color::rgba(200, 40, 40, 255),
    );
    let name_index = systems.gfx.add_text(entity_name, 2, "Npc Name", false);
    let entitynamemap = EntityNameMap(name_index);
    let hpbar = HPBar {
        visible: false,
        bg_index,
        bar_index,
    };
}

pub fn npc_finalized(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: GlobalKey,
) -> Result<()> {
    if !world.contains(entity) {
        return Ok(());
    }

    let npc_sprite = world.get_or_err::<SpriteIndex>(entity)?.0;
    let hpbar = world.get_or_err::<HPBar>(entity)?;
    let name = world.get_or_err::<EntityNameMap>(entity)?.0;

    npc_finalized_data(systems, npc_sprite, name, &hpbar);
    Ok(())
}

pub fn npc_finalized_data(
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

pub fn unload_npc(
    world: &mut World,
    systems: &mut SystemHolder,
    content: &GameContent,
    entity: GlobalKey,
) -> Result<()> {
    let npc_sprite = world.get_or_err::<SpriteIndex>(entity)?.0;

    systems.gfx.remove_gfx(&mut systems.renderer, &npc_sprite);

    let hpbar = world.get_or_err::<HPBar>(entity)?;

    systems
        .gfx
        .remove_gfx(&mut systems.renderer, &hpbar.bar_index);
    systems
        .gfx
        .remove_gfx(&mut systems.renderer, &hpbar.bg_index);

    let entitynamemap = world.get_or_err::<EntityNameMap>(entity)?;

    systems
        .gfx
        .remove_gfx(&mut systems.renderer, &entitynamemap.0);

    if let Some(entitylight) = world.get_or_err::<EntityLight>(entity)?.0 {
        systems
            .gfx
            .remove_area_light(&content.game_lights, entitylight);
    }

    world.despawn(entity)?;
    Ok(())
}

pub fn move_npc(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: GlobalKey,
    move_type: MovementType,
) -> Result<()> {
    if !world.contains(entity) {
        return Ok(());
    }

    let (dir, end) = match move_type {
        MovementType::MovementBuffer => {
            let mut movementbuffer =
                world.get::<&mut MovementBuffer>(entity)?;
            let movement = world.get_or_err::<Movement>(entity)?;

            if movementbuffer.data.is_empty() || movement.is_moving {
                return Ok(());
            }

            let movement_data = movementbuffer.data.pop_front();

            if let Some(data) = movement_data {
                (dir_to_enum(data.dir), Some(data.end_pos))
            } else {
                return Ok(());
            }
        }
        MovementType::Manual(m_dir, m_end) => (dir_to_enum(m_dir), m_end),
    };

    if let Some(end_pos) = end {
        world.get::<&mut EndMovement>(entity)?.0 = end_pos;
    } else {
        let mut pos = world.get_or_err::<Position>(entity)?;

        pos.x += match dir {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0,
        };
        pos.y += match dir {
            Direction::Up => 1,
            Direction::Down => -1,
            _ => 0,
        };

        if pos.x < 0 {
            pos.x = 31;
            pos.map.x -= 1;
        } else if pos.x >= 32 {
            pos.x = 0;
            pos.map.x += 1;
        }
        if pos.y < 0 {
            pos.y = 31;
            pos.map.y -= 1;
        } else if pos.y >= 32 {
            pos.y = 0;
            pos.map.y += 1;
        }

        world.get::<&mut EndMovement>(entity)?.0 = pos;
    }

    if let Ok(mut movement) = world.get::<&mut Movement>(entity) {
        movement.is_moving = true;
        movement.move_direction = dir;
        movement.move_offset = 0.0;
        movement.move_timer = 0.0;
    }
    {
        world.get::<&mut Dir>(entity)?.0 = enum_to_dir(dir);
    }

    let last_frame = if world.get_or_err::<LastMoveFrame>(entity)?.0 == 1 {
        2
    } else {
        1
    };

    {
        world.get::<&mut LastMoveFrame>(entity)?.0 = last_frame;
    }

    let frame = world.get_or_err::<Dir>(entity)?.0 * NPC_SPRITE_FRAME_X as u8;

    set_npc_frame(world, systems, entity, frame as usize + last_frame)
}

pub fn end_npc_move(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: GlobalKey,
) -> Result<()> {
    if !world.contains(entity) {
        return Ok(());
    }

    if let Ok(mut movement) = world.get::<&mut Movement>(entity) {
        if !movement.is_moving {
            return Ok(());
        }
        movement.is_moving = false;
        movement.move_offset = 0.0;
        movement.move_timer = 0.0;
    }

    let end_pos = world.get_or_err::<EndMovement>(entity)?.0;

    {
        if let Ok(mut pos) = world.get::<&mut Position>(entity) {
            pos.x = end_pos.x;
            pos.y = end_pos.y;
            if pos.map != end_pos.map {
                pos.map = end_pos.map;
            }
        }
        world.get::<&mut PositionOffset>(entity)?.offset = Vec2::new(0.0, 0.0);
    }

    let frame = world.get_or_err::<Dir>(entity)?.0 * NPC_SPRITE_FRAME_X as u8;

    set_npc_frame(world, systems, entity, frame as usize)
}

pub fn update_npc_position(
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
        + pos_offset.offset
        - Vec2::new(10.0, 4.0);
    let pos =
        Vec2::new(start_pos.x + texture_pos.x, start_pos.y + texture_pos.y);

    if pos == Vec2::new(cur_pos.x, cur_pos.y) {
        return Ok(());
    }

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

    let sprite_size = systems.gfx.get_size(&sprite);
    let bar_pos = pos + Vec2::new(((sprite_size.x - 20.0) * 0.5).floor(), 0.0);

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
        pos + Vec2::new(((sprite_size.x - textsize.x) * 0.5).floor(), 40.0);

    systems.gfx.set_pos(
        &entitynamemap.0,
        Vec3::new(name_pos.x, name_pos.y, ORDER_ENTITY_NAME),
    );

    Ok(())
}

pub fn set_npc_frame(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: GlobalKey,
    frame_index: usize,
) -> Result<()> {
    if !world.contains(entity) {
        return Ok(());
    }

    let sprite_index = world.get_or_err::<SpriteIndex>(entity)?.0;
    let size = systems.gfx.get_size(&sprite_index);
    let frame_pos = Vec2::new(
        frame_index as f32 % NPC_SPRITE_FRAME_X,
        (frame_index as f32 / NPC_SPRITE_FRAME_X).floor(),
    );

    systems.gfx.set_uv(
        &sprite_index,
        Vec4::new(size.x * frame_pos.x, size.y * frame_pos.y, size.x, size.y),
    );
    Ok(())
}

pub fn init_npc_attack(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: GlobalKey,
    seconds: f32,
) -> Result<()> {
    if !world.contains(entity) {
        return Ok(());
    }

    {
        world.get::<&mut Attacking>(entity)?.0 = true;
        world.get::<&mut AttackTimer>(entity)?.0 = seconds + 0.5;
        if let Ok(mut attackframe) = world.get::<&mut AttackFrame>(entity) {
            attackframe.frame = 0;
            attackframe.timer = seconds + 0.16;
        }
    }

    let frame = world.get_or_err::<Dir>(entity)?.0 * NPC_SPRITE_FRAME_X as u8;

    set_npc_frame(world, systems, entity, frame as usize + 3)
}

pub fn process_npc_attack(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: GlobalKey,
    seconds: f32,
) -> Result<()> {
    if !world.contains(entity) {
        return Ok(());
    }

    if !world.get_or_err::<Attacking>(entity)?.0 {
        return Ok(());
    }

    if seconds < world.get_or_err::<AttackTimer>(entity)?.0 {
        if seconds > world.get_or_err::<AttackFrame>(entity)?.timer {
            {
                world.get::<&mut AttackFrame>(entity)?.frame += 1;
                world.get::<&mut AttackFrame>(entity)?.timer = seconds + 0.16;
            }

            let mut attackframe =
                world.get_or_err::<AttackFrame>(entity)?.frame;

            if attackframe > 2 {
                attackframe = 2;
            }

            let frame =
                world.get_or_err::<Dir>(entity)?.0 * NPC_SPRITE_FRAME_X as u8;

            set_npc_frame(
                world,
                systems,
                entity,
                frame as usize + 3 + attackframe,
            )?;
        }
    } else {
        {
            world.get::<&mut Attacking>(entity)?.0 = false;
        }

        let frame =
            world.get_or_err::<Dir>(entity)?.0 * NPC_SPRITE_FRAME_X as u8;

        set_npc_frame(world, systems, entity, frame as usize)?;
    }
    Ok(())
}

pub fn process_npc_movement(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: GlobalKey,
    socket: &mut Socket,
    content: &mut GameContent,
) -> Result<()> {
    if !world.contains(entity) {
        return Ok(());
    }

    let movement = world.get_or_err::<Movement>(entity)?;

    if !movement.is_moving {
        return Ok(());
    };

    let add_offset = 2.0;

    if movement.move_offset + add_offset < TILE_SIZE as f32 {
        {
            world.get::<&mut Movement>(entity)?.move_offset += add_offset;
        }

        let moveoffset = world.get_or_err::<Movement>(entity)?.move_offset;

        {
            let offset = match movement.move_direction {
                Direction::Up => Vec2::new(0.0, moveoffset),
                Direction::Down => Vec2::new(0.0, -moveoffset),
                Direction::Left => Vec2::new(-moveoffset, 0.0),
                Direction::Right => Vec2::new(moveoffset, 0.0),
            };

            world.get::<&mut PositionOffset>(entity)?.offset = offset;
        }
    } else {
        world.get::<&mut PositionOffset>(entity)?.offset = Vec2::new(0.0, 0.0);
        end_npc_move(world, systems, entity)?;
    }

    update_npc_camera(world, systems, entity, socket, content)
}

pub fn update_npc_camera(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: GlobalKey,
    socket: &mut Socket,
    content: &mut GameContent,
) -> Result<()> {
    let mut query = world.query_one::<(
        &mut HPBar,
        &SpriteIndex,
        &Position,
        &PositionOffset,
        &EntityNameMap,
        &EntityLight,
    )>(entity)?;

    if let Some((
        hpbar,
        spriteindex,
        position,
        positionoffset,
        entitymapname,
        entitylight,
    )) = query.get()
    {
        let is_target = if let Some(target) = content.target.entity {
            target.0 == entity
        } else {
            false
        };

        if is_target {
            if !hpbar.visible {
                hpbar.visible = true;
                systems.gfx.set_visible(&hpbar.bar_index, true);
                systems.gfx.set_visible(&hpbar.bg_index, true);
            }
            let pos = systems.gfx.get_pos(&spriteindex.0);
            content.target.set_target_pos(
                socket,
                systems,
                Vec2::new(pos.x, pos.y),
                hpbar,
            )?;
        } else if hpbar.visible {
            hpbar.visible = false;
            systems.gfx.set_visible(&hpbar.bar_index, false);
            systems.gfx.set_visible(&hpbar.bg_index, false);
        }

        update_npc_position(
            systems,
            content,
            spriteindex.0,
            position,
            positionoffset,
            hpbar,
            entitymapname,
            entitylight.0,
        )?;
    }
    Ok(())
}

pub fn create_npc_light(
    world: &mut World,
    systems: &mut SystemHolder,
    game_light: &GfxType,
    entity: GlobalKey,
) {
    if let Ok(mut entitylight) = world.get::<&mut EntityLight>(entity) {
        entitylight.0 = systems.gfx.add_area_light(game_light, AreaLight {
            pos: Vec2::new(0.0, 0.0),
            color: Color::rgba(100, 100, 100, 20),
            max_distance: 20.0,
            animate: true,
            anim_speed: 5.0,
            dither: 0.8,
            camera_type: CameraType::None,
        })
    }
}
