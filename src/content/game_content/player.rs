use crate::{create_label, Result};
use bytey::{ByteBufferError, ByteBufferRead, ByteBufferWrite};
use graphics::*;
use hecs::World;

pub const PLAYER_SPRITE_FRAME_X: f32 = 6.0;

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Default, ByteBufferRead, ByteBufferWrite,
)]
pub struct PlayerPvP {
    pub pk: bool,
    pub pvpon: bool,
}

use crate::{
    data_types::*, fade::*, game_content::*, send_move, Direction, Socket,
    SystemHolder,
};

pub fn add_player(
    world: &mut World,
    systems: &mut SystemHolder,
    pos: Position,
    cur_map: MapPosition,
    entity: Option<&Entity>,
    sprite: usize,
) -> Result<Entity> {
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
    let sprite = systems.gfx.add_image(image, 0, "Player Sprite".into());
    systems.gfx.set_visible(sprite, false);

    let mut bg_image = Rect::new(&mut systems.renderer, 0);
    bg_image
        .set_size(Vec2::new(20.0, 6.0))
        .set_position(Vec3::new(0.0, 0.0, ORDER_HPBAR_BG))
        .set_color(Color::rgba(80, 80, 80, 255))
        .set_border_width(1.0)
        .set_border_color(Color::rgba(10, 10, 10, 255));
    let bg_index = systems.gfx.add_rect(bg_image, 0, "Player HP BG".into());
    systems.gfx.set_visible(bg_index, false);
    let mut bar_image = Rect::new(&mut systems.renderer, 0);
    bar_image
        .set_size(Vec2::new(18.0, 4.0))
        .set_position(Vec3::new(1.0, 1.0, ORDER_HPBAR))
        .set_color(Color::rgba(180, 30, 30, 255));
    let bar_index = systems.gfx.add_rect(bar_image, 0, "Player HP Bar".into());
    systems.gfx.set_visible(bar_index, false);

    let entity_name = create_label(
        systems,
        Vec3::new(0.0, 0.0, ORDER_ENTITY_NAME),
        Vec2::new(20.0, 20.0),
        Bounds::new(0.0, 0.0, systems.size.width, systems.size.height),
        Color::rgba(230, 230, 230, 255),
    );
    let name_index = systems.gfx.add_text(entity_name, 1, "Player Name".into());
    systems.gfx.set_visible(name_index, false);
    let entitynamemap = EntityNameMap(name_index);

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
        UserAccess::default(),
        Equipment::default(),
        Hidden::default(),
        WorldEntityType::Player,
    );
    let component2 = (
        Level::default(),
        DeathType::default(),
        Physical::default(),
        Vitals::default(),
        SpriteImage::default(),
        MovementBuffer::default(),
        hpbar,
        Finalized::default(),
        entitynamemap,
    );

    if let Some(data) = entity {
        world.spawn_at(data.0, component1);
        world.insert(data.0, component2)?;
        world.insert_one(data.0, EntityType::Player(Entity(data.0)))?;
        Ok(Entity(data.0))
    } else {
        let entity = world.spawn(component1);
        world.insert(entity, component2)?;
        world.insert_one(entity, EntityType::Player(Entity(entity)))?;
        Ok(Entity(entity))
    }
}

pub fn player_finalized(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: &Entity,
) -> Result<()> {
    if !world.contains(entity.0) {
        return Ok(());
    }
    let player_sprite = world.get_or_err::<SpriteIndex>(entity)?.0;
    let hpbar = world.get_or_err::<HPBar>(entity)?;
    let name = world.get_or_err::<EntityNameMap>(entity)?.0;
    player_finalized_data(systems, player_sprite, name, &hpbar);
    Ok(())
}

pub fn player_finalized_data(
    systems: &mut SystemHolder,
    sprite: usize,
    name: usize,
    hpbar: &HPBar,
) {
    systems.gfx.set_visible(sprite, true);
    systems.gfx.set_visible(name, true);

    systems.gfx.set_visible(hpbar.bg_index, hpbar.visible);
    systems.gfx.set_visible(hpbar.bar_index, hpbar.visible);
}

pub fn unload_player(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: &Entity,
) -> Result<()> {
    let player_sprite = world.get_or_err::<SpriteIndex>(entity)?.0;
    systems.gfx.remove_gfx(&mut systems.renderer, player_sprite);
    let hpbar = world.get_or_err::<HPBar>(entity)?;
    systems
        .gfx
        .remove_gfx(&mut systems.renderer, hpbar.bar_index);
    systems
        .gfx
        .remove_gfx(&mut systems.renderer, hpbar.bg_index);
    let entitynamemap = world.get_or_err::<EntityNameMap>(entity)?;
    systems
        .gfx
        .remove_gfx(&mut systems.renderer, entitynamemap.0);
    world.despawn(entity.0)?;
    Ok(())
}

pub fn move_player(
    world: &mut World,
    systems: &mut SystemHolder,
    socket: &mut Socket,
    entity: &Entity,
    content: &mut GameContent,
    move_type: MovementType,
) -> Result<()> {
    if !world.contains(entity.0) {
        return Ok(());
    }

    let (dir, end) = match move_type {
        MovementType::MovementBuffer => {
            let mut movementbuffer =
                world.get::<&mut MovementBuffer>(entity.0)?;
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

    if let Some(p) = content.myentity {
        if &p == entity {
            if world.get_or_err::<Attacking>(entity)?.0 {
                return Ok(());
            }

            if world.get_or_err::<Movement>(entity)?.is_moving {
                return Ok(());
            }

            if !can_move(world, systems, entity, content, &dir)? {
                return Ok(());
            }
        }
    }

    if let Some(end_pos) = end {
        world.get::<&mut EndMovement>(entity.0)?.0 = end_pos;
    } else {
        let pos = world.get_or_err::<Position>(entity)?;

        let adj = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        let dir_index = enum_to_dir(dir) as usize;
        let mut end_move = Position {
            x: pos.x + adj[dir_index].0,
            y: pos.y + adj[dir_index].1,
            map: pos.map,
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

        world.get::<&mut EndMovement>(entity.0)?.0 = end_move;
    }

    let dir_u8 = enum_to_dir(dir);

    if let Some(p) = content.myentity {
        if &p == entity {
            let pos = world.get_or_err::<Position>(entity)?;
            send_move(socket, dir_u8, pos)?;
        }
    }

    if let Ok(mut movement) = world.get::<&mut Movement>(entity.0) {
        movement.is_moving = true;
        movement.move_direction = dir;
        movement.move_offset = 0.0;
        movement.move_timer = 0.0;
    }

    {
        world.get::<&mut Dir>(entity.0)?.0 = dir_u8;
    }
    let last_frame = if world.get_or_err::<LastMoveFrame>(entity)?.0 == 1 {
        2
    } else {
        1
    };
    {
        world.get::<&mut LastMoveFrame>(entity.0)?.0 = last_frame;
    }

    let frame =
        world.get_or_err::<Dir>(entity)?.0 * PLAYER_SPRITE_FRAME_X as u8;
    set_player_frame(world, systems, entity, frame as usize + last_frame)
}

pub fn end_player_move(
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut GameContent,
    socket: &mut Socket,
    entity: &Entity,
    buffer: &mut BufferTask,
) -> Result<()> {
    if !world.contains(entity.0) {
        return Ok(());
    }

    if let Ok(mut movement) = world.get::<&mut Movement>(entity.0) {
        if !movement.is_moving {
            return Ok(());
        }
        movement.is_moving = false;
        movement.move_offset = 0.0;
        movement.move_timer = 0.0;
    }

    let mut move_map: bool = false;
    let end_pos = world.get_or_err::<EndMovement>(entity)?.0;
    {
        if let Ok(mut pos) = world.get::<&mut Position>(entity.0) {
            pos.x = end_pos.x;
            pos.y = end_pos.y;
            if pos.map != end_pos.map {
                pos.map = end_pos.map;
                move_map = true;
            }
        }
        world.get::<&mut PositionOffset>(entity.0)?.offset =
            Vec2::new(0.0, 0.0);
    }

    if let Some(p) = &content.myentity {
        if p == entity && move_map {
            let dir = world.get_or_err::<Movement>(entity)?.move_direction;
            content.move_map(world, systems, socket, dir, buffer)?;
            finalize_entity(world, systems);
        }
    }

    let frame =
        world.get_or_err::<Dir>(entity)?.0 * PLAYER_SPRITE_FRAME_X as u8;
    set_player_frame(world, systems, entity, frame as usize)
}

pub fn update_player_position(
    systems: &mut SystemHolder,
    content: &mut GameContent,
    socket: &mut Socket,
    sprite: usize,
    pos: &Position,
    pos_offset: &PositionOffset,
    hpbar: &HPBar,
    entitynamemap: &EntityNameMap,
    is_target: bool,
) -> Result<()> {
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
        content.target.set_target_pos(socket, systems, pos)?;
    }

    if pos == Vec2::new(cur_pos.x, cur_pos.y) {
        return Ok(());
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

    let textsize = systems.gfx.get_measure(entitynamemap.0).floor();
    let name_pos =
        pos + Vec2::new(((sprite_size.x - textsize.x) * 0.5).floor(), 40.0);
    systems.gfx.set_pos(
        entitynamemap.0,
        Vec3::new(name_pos.x, name_pos.y, ORDER_ENTITY_NAME),
    );

    Ok(())
}

pub fn set_player_frame(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: &Entity,
    frame_index: usize,
) -> Result<()> {
    if !world.contains(entity.0) {
        return Ok(());
    }

    let sprite_index = world.get_or_err::<SpriteIndex>(entity)?.0;
    let size = systems.gfx.get_size(sprite_index);
    let frame_pos = Vec2::new(
        frame_index as f32 % PLAYER_SPRITE_FRAME_X,
        (frame_index as f32 / PLAYER_SPRITE_FRAME_X).floor(),
    );
    systems.gfx.set_uv(
        sprite_index,
        Vec4::new(size.x * frame_pos.x, size.y * frame_pos.y, size.x, size.y),
    );
    Ok(())
}

pub fn init_player_attack(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: &Entity,
    seconds: f32,
) -> Result<()> {
    if !world.contains(entity.0) {
        return Ok(());
    }

    if world.get_or_err::<Attacking>(entity)?.0
        || world.get_or_err::<Movement>(entity)?.is_moving
    {
        return Ok(());
    }

    {
        world.get::<&mut Attacking>(entity.0)?.0 = true;
        world.get::<&mut AttackTimer>(entity.0)?.0 = seconds + 0.5;
        if let Ok(mut attackframe) = world.get::<&mut AttackFrame>(entity.0) {
            attackframe.frame = 0;
            attackframe.timer = seconds + 0.16;
        }
    }
    let frame =
        world.get_or_err::<Dir>(entity)?.0 * PLAYER_SPRITE_FRAME_X as u8;
    set_player_frame(world, systems, entity, frame as usize + 3)
}

pub fn process_player_attack(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: &Entity,
    seconds: f32,
) -> Result<()> {
    if !world.contains(entity.0) {
        return Ok(());
    }

    if !world.get_or_err::<Attacking>(entity)?.0 {
        return Ok(());
    }

    if seconds < world.get_or_err::<AttackTimer>(entity)?.0 {
        if seconds > world.get_or_err::<AttackFrame>(entity)?.timer {
            {
                world.get::<&mut AttackFrame>(entity.0)?.frame += 1;
                world.get::<&mut AttackFrame>(entity.0)?.timer = seconds + 0.16;
            }

            let mut attackframe =
                world.get_or_err::<AttackFrame>(entity)?.frame;
            if attackframe > 2 {
                attackframe = 2;
            }
            let frame = world.get_or_err::<Dir>(entity)?.0
                * PLAYER_SPRITE_FRAME_X as u8;
            set_player_frame(
                world,
                systems,
                entity,
                frame as usize + 3 + attackframe,
            )?;
        }
    } else {
        {
            world.get::<&mut Attacking>(entity.0)?.0 = false;
        }
        let frame =
            world.get_or_err::<Dir>(entity)?.0 * PLAYER_SPRITE_FRAME_X as u8;
        set_player_frame(world, systems, entity, frame as usize)?;
    }
    Ok(())
}

pub fn process_player_movement(
    world: &mut World,
    systems: &mut SystemHolder,
    socket: &mut Socket,
    entity: &Entity,
    content: &mut GameContent,
    buffer: &mut BufferTask,
) -> Result<()> {
    if !world.contains(entity.0) {
        return Ok(());
    }

    let movement = world.get_or_err::<Movement>(entity)?;
    if !movement.is_moving {
        return Ok(());
    };

    let add_offset = 2.0;

    if movement.move_offset + add_offset < TILE_SIZE as f32 {
        {
            world.get::<&mut Movement>(entity.0)?.move_offset += add_offset;
        }
        let moveoffset = world.get_or_err::<Movement>(entity)?.move_offset;
        {
            let offset = match movement.move_direction {
                Direction::Up => Vec2::new(0.0, moveoffset),
                Direction::Down => Vec2::new(0.0, -moveoffset),
                Direction::Left => Vec2::new(-moveoffset, 0.0),
                Direction::Right => Vec2::new(moveoffset, 0.0),
            };
            world.get::<&mut PositionOffset>(entity.0)?.offset = offset;
        }
    } else {
        end_player_move(world, systems, content, socket, entity, buffer)?;
    }
    Ok(())
}

pub fn player_get_next_lvl_exp(
    world: &mut World,
    entity: &Entity,
) -> Result<u64> {
    let mut query = world.query_one::<&Level>(entity.0)?;

    if let Some(player_level) = query.get() {
        let exp_per_level = match player_level.0 {
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

        Ok(player_level.0 as u64 * exp_per_level as u64)
    } else {
        Ok(0)
    }
}
