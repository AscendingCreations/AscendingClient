use hecs::World;

use crate::{
    content::*, dir_to_enum, BufferTask, Entity, Position, Result, Socket,
    SystemHolder, WorldEntityType,
};

#[derive(Copy, Clone, Debug, Default)]
pub struct LoopTimer {
    player_tmr: f32,
    npc_tmr: f32,
    input_tmr: f32,
    maprefresh_tmr: f32,
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
            update_camera(world, &mut content.game_content, systems, socket)?;
            float_text_loop(systems, &mut content.game_content, seconds);

            if seconds > loop_timer.player_tmr {
                update_player(
                    world,
                    systems,
                    socket,
                    &mut content.game_content,
                    buffer,
                    seconds,
                )?;
                loop_timer.player_tmr = seconds + 0.01;
            }

            if seconds > loop_timer.npc_tmr {
                update_npc(world, systems, &mut content.game_content, seconds)?;
                loop_timer.npc_tmr = seconds + 0.01;
            }

            if seconds > loop_timer.maprefresh_tmr {
                update_map_refresh(world, systems, content)?;
                loop_timer.maprefresh_tmr = seconds + 0.5;
            }

            if seconds > loop_timer.input_tmr {
                content
                    .game_content
                    .handle_key_input(world, systems, socket, seconds)?;
                loop_timer.input_tmr = seconds + 0.032;
            }
        }
        ContentType::Menu => {}
    }
    Ok(())
}

pub fn update_map_refresh(
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
) -> Result<()> {
    let mut entity_to_remove = Vec::new();

    for (entity, (_, worldentitytype)) in world
        .query::<(&Position, &WorldEntityType)>()
        .iter()
        .filter(|(_, (pos, _))| {
            !is_map_connected(content.game_content.map.map_pos, pos.map)
        })
    {
        entity_to_remove.push((entity, *worldentitytype));
    }

    for (entity, worldtype) in entity_to_remove {
        match worldtype {
            WorldEntityType::Player => {
                unload_player(world, systems, &Entity(entity))?;
                content
                    .game_content
                    .players
                    .borrow_mut()
                    .swap_remove(&Entity(entity));
            }
            WorldEntityType::Npc => {
                unload_npc(world, systems, &Entity(entity))?;
                content.game_content.npcs.swap_remove(&Entity(entity));
            }
            WorldEntityType::MapItem => {
                unload_mapitems(world, systems, &Entity(entity))?;
                content.game_content.mapitems.swap_remove(&Entity(entity));
            }
            _ => {}
        }
    }

    Ok(())
}
