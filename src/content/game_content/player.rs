use graphics::*;
use hecs::World;

const SPRITE_FRAME_X: f32 = 3.0;

use crate::{
    gfx_order::*,
    Direction,
    DrawSetting,
    game_content::entity::*,
    values::*,
    game_content::*,
};

pub fn add_player(
    world: &mut World,
    systems: &mut DrawSetting,
    tile_pos: Vec2,
) -> Entity {
    let texture_pos = tile_pos * TILE_SIZE as f32;
    let mut image = Image::new(Some(systems.resource.player.allocation),
            &mut systems.renderer, 0);
    image.pos = Vec3::new(texture_pos.x, texture_pos.y, ORDER_PLAYER);
    image.hw = Vec2::new(40.0, 40.0);
    image.uv = Vec4::new(0.0, 0.0, 40.0, 40.0);
    let sprite = systems.gfx.add_image(image, 0);
    
    let entity = world.spawn((
        Position {
            pos: tile_pos,
            offset: Vec2::new(0.0, 0.0),
        },
        Sprite(sprite),
        Movement::default(),
        Dir::default(),
        LastMoveFrame::default(),
    ));
    let _ = world.insert_one(entity, EntityType::Player(Entity(entity)));
    Entity(entity)
}

pub fn unload_player(
    world: &mut World,
    systems: &mut DrawSetting,
    entity: &Entity,
) {
    let player_sprite = world.get_or_panic::<Sprite>(entity).0;
    systems.gfx.remove_gfx(player_sprite);
    let _ = world.despawn(entity.0);
}

pub fn move_player(
    world: &mut World,
    systems: &mut DrawSetting,
    entity: &Entity,
    dir: &Direction,
) {
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
        world.get::<&mut Dir>(entity.0).expect("Could not find Dir").0 = match dir {
            Direction::Up => 2,
            Direction::Down => 0,
            Direction::Left => 3,
            Direction::Right => 1,
        };
    }
    let last_frame = if world.get_or_panic::<LastMoveFrame>(entity).0 == 1 { 2 } else { 1 };
    {
        world.get::<&mut LastMoveFrame>(entity.0).expect("Could not find LastFrame").0 = last_frame;
    }
    let frame = world.get_or_panic::<Dir>(entity).0 * 3;
    set_player_frame(world, systems, entity, frame as usize + last_frame);
}

pub fn set_player_frame(
    world: &mut World,
    systems: &mut DrawSetting,
    entity: &Entity,
    frame_index: usize,
) {
    let sprite_index = world.get_or_panic::<Sprite>(entity).0;
    let size = systems.gfx.get_size(sprite_index);
    let frame_pos = Vec2::new(frame_index as f32 % SPRITE_FRAME_X,
        (frame_index  as f32 / SPRITE_FRAME_X).floor());
    systems.gfx.set_uv(sprite_index,
        Vec4::new(size.x * frame_pos.x, size.y * frame_pos.y, size.x, size.y));
}

pub fn end_player_move(
    world: &mut World,
    systems: &mut DrawSetting,
    entity: &Entity,
) {
    if let Ok(mut movement) = world.get::<&mut Movement>(entity.0) {
        if !movement.is_moving {
            return;
        }
        movement.is_moving = false;
        movement.move_direction = Direction::default();
        movement.move_offset = 0.0;
        movement.move_timer = 0.0;
    }
    let frame = world.get_or_panic::<Dir>(entity).0 * 3;
    set_player_frame(world, systems, entity, frame as usize);
}

pub fn update_player_position(
    world: &mut World,
    systems: &mut DrawSetting,
    camera: &Camera,
    entity: &Entity,
) {
    let player_sprite = world.get_or_panic::<Sprite>(entity).0;
    let cur_tile_pos = world.get_or_panic::<Position>(entity).pos;
    let cur_pos = systems.gfx.get_pos(player_sprite);
    let offset = world.get_or_panic::<Position>(entity).offset;
    let texture_pos = camera.pos + (cur_tile_pos * TILE_SIZE as f32) + offset - Vec2::new(10.0, 4.0);
    if texture_pos == Vec2::new(cur_pos.x, cur_pos.y) {
        return;
    }
    systems.gfx.set_pos(player_sprite,
        Vec3::new(texture_pos.x, 
                texture_pos.y,
                cur_pos.z));
}

pub fn process_player_movement(
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
            world.get::<&mut Position>(entity.0).expect("Could not find Position").offset = offset;
        }
    } else {
        let cur_tile_pos = world.get_or_panic::<Position>(entity).pos;
        let new_tile_pos = match movement.move_direction {
            Direction::Up => cur_tile_pos + Vec2::new(0.0, 1.0),
            Direction::Down => cur_tile_pos + Vec2::new(0.0, -1.0),
            Direction::Left => cur_tile_pos + Vec2::new(-1.0, 0.0),
            Direction::Right => cur_tile_pos + Vec2::new(1.0, 0.0),
        };
        {
            world.get::<&mut Position>(entity.0).expect("Could not find Position").pos = new_tile_pos;
            world.get::<&mut Position>(entity.0).expect("Could not find Position").offset = Vec2::new(0.0, 0.0);
        }
        end_player_move(world, systems, entity);
    }
}