use graphics::*;
use hecs::World;

pub const NPC_SPRITE_FRAME_X: f32 = 6.0;

use crate::{
    Direction,
    DrawSetting,
    game_content::entity::*,
    values::*,
    game_content::*,
};

pub fn add_npc(
    world: &mut World,
    systems: &mut DrawSetting,
    pos: Position,
    cur_map: MapPosition,
) -> Entity {
    let start_pos = get_start_map_pos(cur_map, pos.map).unwrap_or_else(|| Vec2::new(0.0, 0.0));
    let texture_pos = Vec2::new(pos.x as f32, pos.y as f32) * TILE_SIZE as f32;
    let mut image = Image::new(Some(systems.resource.players[0].allocation), // ToDo Change Sprite
            &mut systems.renderer, 0);
    image.pos = Vec3::new(start_pos.x + texture_pos.x, start_pos.y + texture_pos.y, ORDER_NPC);
    image.hw = Vec2::new(40.0, 40.0);
    image.uv = Vec4::new(0.0, 0.0, 40.0, 40.0);
    let sprite = systems.gfx.add_image(image, 0);
    
    let entity = world.spawn((
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
        WorldEntityType::Npc,
    ));
    let _ = world.insert_one(entity, EntityType::Npc(Entity(entity)));
    Entity(entity)
}

pub fn unload_npc(
    world: &mut World,
    systems: &mut DrawSetting,
    entity: &Entity,
) {
    let npc_sprite = world.get_or_panic::<SpriteIndex>(entity).0;
    systems.gfx.remove_gfx(npc_sprite);
    let _ = world.despawn(entity.0);
}

pub fn move_npc(
    world: &mut World,
    systems: &mut DrawSetting,
    entity: &Entity,
    dir: &Direction,
    end: Option<Position>,
) {
    if world.get_or_panic::<Attacking>(entity).0 {
        return;
    }

    if let Some(end_pos) = end {
        world.get::<&mut EndMovement>(entity.0).expect("Could not find EndMovement").0 = end_pos.clone();
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

        world.get::<&mut EndMovement>(entity.0).expect("Could not find EndMovement").0 = pos.clone();
    }
    
    if let Ok(mut movement) = world.get::<&mut Movement>(entity.0) {
        if movement.is_moving {
            return;
        }
        movement.is_moving = true;
        movement.move_direction = dir.clone();
        movement.move_offset = 0.0;
        movement.move_timer = 0.0;
    }
    {
        world.get::<&mut Dir>(entity.0).expect("Could not find Dir").0 = enum_to_dir(*dir);
    }
    let last_frame = if world.get_or_panic::<LastMoveFrame>(entity).0 == 1 { 2 } else { 1 };
    {
        world.get::<&mut LastMoveFrame>(entity.0).expect("Could not find LastFrame").0 = last_frame;
    }
    let frame = world.get_or_panic::<Dir>(entity).0 * NPC_SPRITE_FRAME_X as u8;
    set_npc_frame(world, systems, entity, frame as usize + last_frame);
}

pub fn end_npc_move(
    world: &mut World,
    systems: &mut DrawSetting,
    entity: &Entity,
) {
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
            pos.x = end_pos.x as i32;
            pos.y = end_pos.y as i32;
            if pos.map != end_pos.map {
                pos.map = end_pos.map;
            }
        }
        world.get::<&mut PositionOffset>(entity.0).expect("Could not find Position").offset = Vec2::new(0.0, 0.0);
    }

    let frame = world.get_or_panic::<Dir>(entity).0 * NPC_SPRITE_FRAME_X as u8;
    set_npc_frame(world, systems, entity, frame as usize);
}

pub fn update_npc_position(
    systems: &mut DrawSetting,
    content: &GameContent,
    sprite: usize,
    pos: &Position,
    pos_offset: &PositionOffset,
) {
    let start_pos = get_start_map_pos(content.map.map_pos, pos.map).unwrap_or_else(|| Vec2::new(0.0, 0.0));
    let cur_pos = systems.gfx.get_pos(sprite);
    let texture_pos = content.camera.pos + 
        (Vec2::new(pos.x as f32, pos.y as f32) * TILE_SIZE as f32) + pos_offset.offset - Vec2::new(10.0, 4.0);
    if start_pos + texture_pos == Vec2::new(cur_pos.x, cur_pos.y) {
        return;
    }
    systems.gfx.set_pos(sprite,
        Vec3::new(start_pos.x + texture_pos.x, 
                start_pos.y + texture_pos.y,
                cur_pos.z));
}

pub fn set_npc_frame(
    world: &mut World,
    systems: &mut DrawSetting,
    entity: &Entity,
    frame_index: usize,
) {
    let sprite_index = world.get_or_panic::<SpriteIndex>(entity).0;
    let size = systems.gfx.get_size(sprite_index);
    let frame_pos = Vec2::new(frame_index as f32 % NPC_SPRITE_FRAME_X,
        (frame_index  as f32 / NPC_SPRITE_FRAME_X).floor());
    systems.gfx.set_uv(sprite_index,
        Vec4::new(size.x * frame_pos.x, size.y * frame_pos.y, size.x, size.y));
}

pub fn init_npc_attack(
    world: &mut World,
    systems: &mut DrawSetting,
    entity: &Entity,
    seconds: f32,
) {
    if world.get_or_panic::<Attacking>(entity).0 || world.get_or_panic::<Movement>(entity).is_moving {
        return;
    }

    {
        world.get::<&mut Attacking>(entity.0).expect("Could not find attacking").0 = true;
        world.get::<&mut AttackTimer>(entity.0).expect("Could not find AttackTimer").0 = seconds + 0.5;
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
    systems: &mut DrawSetting,
    entity: &Entity,
    seconds: f32,
) {
    if !world.get_or_panic::<Attacking>(entity).0 {
        return;
    }
    
    if seconds < world.get_or_panic::<AttackTimer>(entity).0 {
        if seconds > world.get_or_panic::<AttackFrame>(entity).timer {
            {
                world.get::<&mut AttackFrame>(entity.0).expect("Could not find AttackTimer").frame += 1;
                world.get::<&mut AttackFrame>(entity.0).expect("Could not find AttackTimer").timer = seconds + 0.16;
            }

            let mut attackframe = world.get_or_panic::<AttackFrame>(entity).frame;
            if attackframe > 2 { attackframe = 2; }
            let frame = world.get_or_panic::<Dir>(entity).0 * NPC_SPRITE_FRAME_X as u8;
            set_npc_frame(world, systems, entity, frame as usize + 3 + attackframe);
        }
    } else {
        {
            world.get::<&mut Attacking>(entity.0).expect("Could not find attacking").0 = false;
        }
        let frame = world.get_or_panic::<Dir>(entity).0 * NPC_SPRITE_FRAME_X as u8;
        set_npc_frame(world, systems, entity, frame as usize);
    }
}

pub fn process_npc_movement(
    world: &mut World,
    systems: &mut DrawSetting,
    entity: &Entity,
) {
    let movement = world.get_or_panic::<Movement>(entity);
    if !movement.is_moving { return };
    
    let add_offset = 2.0;

    if movement.move_offset + add_offset < TILE_SIZE as f32 {
        {
            world.get::<&mut Movement>(entity.0).expect("Could not find movement").move_offset += add_offset;
        }
        let moveoffset = world.get_or_panic::<Movement>(entity).move_offset;
        {
            let offset = match movement.move_direction {
                Direction::Up => Vec2::new(0.0, moveoffset),
                Direction::Down => Vec2::new(0.0, -moveoffset),
                Direction::Left => Vec2::new(-moveoffset, 0.0),
                Direction::Right => Vec2::new(moveoffset, 0.0),
            };
            world.get::<&mut PositionOffset>(entity.0).expect("Could not find Position").offset = offset;
        }
    } else {
        end_npc_move(world, systems, entity);
    }
}

