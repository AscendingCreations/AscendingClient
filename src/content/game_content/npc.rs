use graphics::*;
use hecs::World;

pub const NPC_SPRITE_FRAME_X: f32 = 6.0;

use crate::{
    game_content::entity::*, game_content::*, values::*, Direction,
    SystemHolder,
};

pub fn add_npc(
    world: &mut World,
    systems: &mut SystemHolder,
    pos: Position,
    cur_map: MapPosition,
    entity: Option<&Entity>,
) -> Entity {
    let start_pos = get_start_map_pos(cur_map, pos.map)
        .unwrap_or_else(|| Vec2::new(0.0, 0.0));
    let texture_pos = Vec2::new(pos.x as f32, pos.y as f32) * TILE_SIZE as f32;
    let mut image = Image::new(
        Some(systems.resource.players[1].allocation), // ToDo Change Sprite
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
    let sprite = systems.gfx.add_image(image, 0);
    systems.gfx.set_visible(sprite, false);

    let mut bg_image = Rect::new(&mut systems.renderer, 0);
    bg_image
        .set_size(Vec2::new(20.0, 6.0))
        .set_position(Vec3::new(0.0, 0.0, ORDER_HPBAR_BG))
        .set_color(Color::rgba(80, 80, 80, 255))
        .set_border_width(1.0)
        .set_border_color(Color::rgba(10, 10, 10, 255));
    let bg_index = systems.gfx.add_rect(bg_image, 0);
    systems.gfx.set_visible(bg_index, false);
    let mut bar_image = Rect::new(&mut systems.renderer, 0);
    bar_image
        .set_size(Vec2::new(18.0, 4.0))
        .set_position(Vec3::new(1.0, 1.0, ORDER_HPBAR))
        .set_color(Color::rgba(180, 30, 30, 255));
    let bar_index = systems.gfx.add_rect(bar_image, 0);
    systems.gfx.set_visible(bar_index, false);

    let hpbar = HPBar {
        visible: false,
        bg_index,
        bar_index,
    };

    let component1 = (
        pos,
        PositionOffset::default(),
        SpriteIndex(sprite),
        Movement::default(),
        EndMovement::default(),
        Dir::default(),
        LastMoveFrame::default(),
        Attacking::default(),
        AttackTimer::default(),
        AttackFrame::default(),
        EntityName::default(),
        MovementBuffer::default(),
        Vitals::default(),
        SpriteImage::default(),
        WorldEntityType::Npc,
    );
    let component2 = (
        Hidden::default(),
        DeathType::default(),
        NpcMode::default(),
        NpcIndex::default(),
        Physical::default(),
        Level::default(),
        hpbar,
        Finalized::default(),
    );

    if let Some(data) = entity {
        world.spawn_at(data.0, component1);
        let _ = world.insert(data.0, component2);
        let _ = world.insert_one(data.0, EntityType::Npc(Entity(data.0)));
        Entity(data.0)
    } else {
        let entity = world.spawn(component1);
        let _ = world.insert(entity, component2);
        let _ = world.insert_one(entity, EntityType::Npc(Entity(entity)));
        Entity(entity)
    }
}

pub fn npc_finalized(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: &Entity,
) {
    if !world.contains(entity.0) {
        return;
    }
    let npc_sprite = world.get_or_panic::<SpriteIndex>(entity).0;
    let hpbar = world.get_or_panic::<HPBar>(entity);
    npc_finalized_data(systems, npc_sprite, &hpbar);
}

pub fn npc_finalized_data(
    systems: &mut SystemHolder,
    sprite: usize,
    hpbar: &HPBar,
) {
    systems.gfx.set_visible(sprite, true);

    systems.gfx.set_visible(hpbar.bg_index, hpbar.visible);
    systems.gfx.set_visible(hpbar.bar_index, hpbar.visible);
}

pub fn unload_npc(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: &Entity,
) {
    let npc_sprite = world.get_or_panic::<SpriteIndex>(entity).0;
    systems.gfx.remove_gfx(npc_sprite);
    let hpbar = world.get_or_panic::<HPBar>(entity);
    systems.gfx.remove_gfx(hpbar.bar_index);
    systems.gfx.remove_gfx(hpbar.bg_index);
    let _ = world.despawn(entity.0);
}

pub fn move_npc(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: &Entity,
    move_type: MovementType,
) {
    if !world.contains(entity.0) {
        return;
    }

    let (dir, end) = match move_type {
        MovementType::MovementBuffer => {
            let mut movementbuffer = world
                .get::<&mut MovementBuffer>(entity.0)
                .expect("Could not find MovementBuffer");
            let movement = world.get_or_panic::<Movement>(entity);
            if movementbuffer.data.is_empty() || movement.is_moving {
                return;
            }
            let movement_data = movementbuffer.data.pop_front();
            if let Some(data) = movement_data {
                (dir_to_enum(data.dir), Some(data.end_pos))
            } else {
                return;
            }
        }
        MovementType::Manual(m_dir, m_end) => (dir_to_enum(m_dir), m_end),
    };

    if let Some(end_pos) = end {
        world
            .get::<&mut EndMovement>(entity.0)
            .expect("Could not find EndMovement")
            .0 = end_pos;
    } else {
        let mut pos = world.get_or_panic::<Position>(entity);
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

        world
            .get::<&mut EndMovement>(entity.0)
            .expect("Could not find EndMovement")
            .0 = pos;
    }

    if let Ok(mut movement) = world.get::<&mut Movement>(entity.0) {
        movement.is_moving = true;
        movement.move_direction = dir;
        movement.move_offset = 0.0;
        movement.move_timer = 0.0;
    }
    {
        world
            .get::<&mut Dir>(entity.0)
            .expect("Could not find Dir")
            .0 = enum_to_dir(dir);
    }
    let last_frame = if world.get_or_panic::<LastMoveFrame>(entity).0 == 1 {
        2
    } else {
        1
    };
    {
        world
            .get::<&mut LastMoveFrame>(entity.0)
            .expect("Could not find LastFrame")
            .0 = last_frame;
    }
    let frame = world.get_or_panic::<Dir>(entity).0 * NPC_SPRITE_FRAME_X as u8;
    set_npc_frame(world, systems, entity, frame as usize + last_frame);
}

pub fn end_npc_move(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: &Entity,
) {
    if !world.contains(entity.0) {
        return;
    }

    if let Ok(mut movement) = world.get::<&mut Movement>(entity.0) {
        if !movement.is_moving {
            return;
        }
        movement.is_moving = false;
        movement.move_offset = 0.0;
        movement.move_timer = 0.0;
    }

    let end_pos = world.get_or_panic::<EndMovement>(entity).0;
    {
        if let Ok(mut pos) = world.get::<&mut Position>(entity.0) {
            pos.x = end_pos.x;
            pos.y = end_pos.y;
            if pos.map != end_pos.map {
                pos.map = end_pos.map;
            }
        }
        world
            .get::<&mut PositionOffset>(entity.0)
            .expect("Could not find Position")
            .offset = Vec2::new(0.0, 0.0);
    }

    let frame = world.get_or_panic::<Dir>(entity).0 * NPC_SPRITE_FRAME_X as u8;
    set_npc_frame(world, systems, entity, frame as usize);
}

pub fn update_npc_position(
    systems: &mut SystemHolder,
    content: &mut GameContent,
    socket: &mut Socket,
    sprite: usize,
    pos: &Position,
    pos_offset: &PositionOffset,
    hpbar: &HPBar,
    is_target: bool,
) {
    let start_pos = get_start_map_pos(content.map.map_pos, pos.map)
        .unwrap_or_else(|| Vec2::new(0.0, 0.0));
    let cur_pos = systems.gfx.get_pos(sprite);
    let texture_pos = content.camera.pos
        + (Vec2::new(pos.x as f32, pos.y as f32) * TILE_SIZE as f32)
        + pos_offset.offset
        - Vec2::new(10.0, 4.0);

    let pos =
        Vec2::new(start_pos.x + texture_pos.x, start_pos.y + texture_pos.y);

    if is_target {
        content.target.set_target_pos(socket, systems, pos);
    }

    if pos == Vec2::new(cur_pos.x, cur_pos.y) {
        return;
    }

    systems
        .gfx
        .set_pos(sprite, Vec3::new(pos.x, pos.y, cur_pos.z));

    let sprite_size = systems.gfx.get_size(sprite);
    let bar_pos = pos + Vec2::new(((sprite_size.x - 20.0) * 0.5).floor(), 0.0);
    systems.gfx.set_pos(
        hpbar.bar_index,
        Vec3::new(bar_pos.x + 1.0, bar_pos.y + 1.0, ORDER_HPBAR),
    );
    systems.gfx.set_pos(
        hpbar.bg_index,
        Vec3::new(bar_pos.x, bar_pos.y, ORDER_HPBAR_BG),
    );
}

pub fn set_npc_frame(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: &Entity,
    frame_index: usize,
) {
    if !world.contains(entity.0) {
        return;
    }

    let sprite_index = world.get_or_panic::<SpriteIndex>(entity).0;
    let size = systems.gfx.get_size(sprite_index);
    let frame_pos = Vec2::new(
        frame_index as f32 % NPC_SPRITE_FRAME_X,
        (frame_index as f32 / NPC_SPRITE_FRAME_X).floor(),
    );
    systems.gfx.set_uv(
        sprite_index,
        Vec4::new(size.x * frame_pos.x, size.y * frame_pos.y, size.x, size.y),
    );
}

pub fn init_npc_attack(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: &Entity,
    seconds: f32,
) {
    if !world.contains(entity.0) {
        return;
    }

    {
        world
            .get::<&mut Attacking>(entity.0)
            .expect("Could not find attacking")
            .0 = true;
        world
            .get::<&mut AttackTimer>(entity.0)
            .expect("Could not find AttackTimer")
            .0 = seconds + 0.5;
        if let Ok(mut attackframe) = world.get::<&mut AttackFrame>(entity.0) {
            attackframe.frame = 0;
            attackframe.timer = seconds + 0.16;
        }
    }
    let frame = world.get_or_panic::<Dir>(entity).0 * NPC_SPRITE_FRAME_X as u8;
    set_npc_frame(world, systems, entity, frame as usize + 3);
}

pub fn process_npc_attack(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: &Entity,
    seconds: f32,
) {
    if !world.contains(entity.0) {
        return;
    }

    if !world.get_or_panic::<Attacking>(entity).0 {
        return;
    }

    if seconds < world.get_or_panic::<AttackTimer>(entity).0 {
        if seconds > world.get_or_panic::<AttackFrame>(entity).timer {
            {
                world
                    .get::<&mut AttackFrame>(entity.0)
                    .expect("Could not find AttackTimer")
                    .frame += 1;
                world
                    .get::<&mut AttackFrame>(entity.0)
                    .expect("Could not find AttackTimer")
                    .timer = seconds + 0.16;
            }

            let mut attackframe =
                world.get_or_panic::<AttackFrame>(entity).frame;
            if attackframe > 2 {
                attackframe = 2;
            }
            let frame =
                world.get_or_panic::<Dir>(entity).0 * NPC_SPRITE_FRAME_X as u8;
            set_npc_frame(
                world,
                systems,
                entity,
                frame as usize + 3 + attackframe,
            );
        }
    } else {
        {
            world
                .get::<&mut Attacking>(entity.0)
                .expect("Could not find attacking")
                .0 = false;
        }
        let frame =
            world.get_or_panic::<Dir>(entity).0 * NPC_SPRITE_FRAME_X as u8;
        set_npc_frame(world, systems, entity, frame as usize);
    }
}

pub fn process_npc_movement(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: &Entity,
) {
    if !world.contains(entity.0) {
        return;
    }

    let movement = world.get_or_panic::<Movement>(entity);
    if !movement.is_moving {
        return;
    };

    let add_offset = 2.0;

    if movement.move_offset + add_offset < TILE_SIZE as f32 {
        {
            world
                .get::<&mut Movement>(entity.0)
                .expect("Could not find movement")
                .move_offset += add_offset;
        }
        let moveoffset = world.get_or_panic::<Movement>(entity).move_offset;
        {
            let offset = match movement.move_direction {
                Direction::Up => Vec2::new(0.0, moveoffset),
                Direction::Down => Vec2::new(0.0, -moveoffset),
                Direction::Left => Vec2::new(-moveoffset, 0.0),
                Direction::Right => Vec2::new(moveoffset, 0.0),
            };
            world
                .get::<&mut PositionOffset>(entity.0)
                .expect("Could not find Position")
                .offset = offset;
        }
    } else {
        end_npc_move(world, systems, entity);
    }
}
