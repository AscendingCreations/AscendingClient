use hecs::World;
use log::info;

use crate::{
    content::*, dir_to_enum, send_gameping, BufferTask, Entity, MyInstant,
    Position, Result, Socket, SystemHolder, WorldEntityType,
};

#[derive(Copy, Clone, Debug, Default)]
pub struct LoopTimer {
    entity_tmr: f32,
    input_tmr: f32,
    maprefresh_tmr: f32,
    ping_tmr: f32,
}

pub fn game_loop(
    socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    buffer: &mut BufferTask,
    seconds: f32,
    loop_timer: &mut LoopTimer,
) -> Result<()> {
    match content.content_type {
        ContentType::Game => {
            if seconds > loop_timer.maprefresh_tmr {
                update_map_refresh(world, systems, &mut content.game_content)?;
                loop_timer.maprefresh_tmr = seconds + 0.5;
            }

            if seconds > loop_timer.entity_tmr {
                update_player(
                    world,
                    systems,
                    socket,
                    &mut content.game_content,
                    buffer,
                    seconds,
                )?;
                update_npc(
                    world,
                    systems,
                    socket,
                    &mut content.game_content,
                    seconds,
                )?;
                float_text_loop(systems, &mut content.game_content, seconds);

                loop_timer.entity_tmr = seconds + 0.025;
            }

            update_camera(world, &mut content.game_content, systems, socket)?;

            if seconds > loop_timer.input_tmr {
                content
                    .game_content
                    .handle_key_input(world, systems, socket, seconds)?;
                loop_timer.input_tmr = seconds + 0.032;
            }

            if seconds > loop_timer.ping_tmr && systems.config.show_ping {
                send_gameping(socket)?;
                content.ping_start = MyInstant::now();
                loop_timer.ping_tmr = seconds + 1.0;
            }
        }
        ContentType::Menu => {}
    }
    Ok(())
}

pub fn update_map_refresh(
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut GameContent,
) -> Result<()> {
    if !content.refresh_map {
        return Ok(());
    }
    content.refresh_map = false;

    let mut entity_to_remove = Vec::with_capacity(1000);

    for (entity, (_, worldentitytype)) in world
        .query::<(&Position, &WorldEntityType)>()
        .iter()
        .filter(|(_, (pos, _))| content.map.map_pos.checkdistance(pos.map) > 1)
    {
        entity_to_remove.push((entity, *worldentitytype));
    }

    for (entity, worldtype) in entity_to_remove {
        match worldtype {
            WorldEntityType::Player => {
                unload_player(world, systems, content, &Entity(entity))?;
                content.players.borrow_mut().swap_remove(&Entity(entity));
            }
            WorldEntityType::Npc => {
                unload_npc(world, systems, content, &Entity(entity))?;
                content.npcs.borrow_mut().swap_remove(&Entity(entity));
            }
            WorldEntityType::MapItem => {
                unload_mapitems(world, systems, content, &Entity(entity))?;
                content.mapitems.borrow_mut().swap_remove(&Entity(entity));
            }
            _ => {}
        }
    }

    Ok(())
}
