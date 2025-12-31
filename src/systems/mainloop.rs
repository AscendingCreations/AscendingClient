use camera::controls::FlatControls;
use graphics::MapRenderer;
use log::info;
use time::Instant;

use crate::{
    BufferTask, Entity, EntityKind, Position, Result, SystemHolder, World,
    content::{game_content::map, *},
    dir_to_enum, send_gameping,
    systems::State,
};

use super::Poller;

#[derive(Copy, Clone, Debug, Default)]
pub struct LoopTimer {
    entity_tmr: f32,
    input_tmr: f32,
    maprefresh_tmr: f32,
    ping_tmr: f32,
}

pub fn game_loop(
    socket: &mut Poller,
    world: &mut World,
    systems: &mut SystemHolder,
    graphics: &mut State<FlatControls>,
    content: &mut Content,
    buffer: &mut BufferTask,
    seconds: f32,
    delta: f32,
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
                    graphics,
                    seconds,
                    delta,
                )?;
                update_npc(
                    world,
                    systems,
                    socket,
                    &mut content.game_content,
                    seconds,
                    delta,
                )?;
                float_text_loop(systems, &mut content.game_content, seconds)?;

                loop_timer.entity_tmr = seconds + 0.025;
            }

            if seconds > loop_timer.input_tmr {
                content
                    .game_content
                    .handle_key_input(world, systems, socket, seconds)?;
                loop_timer.input_tmr = seconds + 0.032;
            }

            if seconds > loop_timer.ping_tmr
                && systems.config.show_ping
                && !systems.fade.show
            {
                send_gameping(socket)?;
                content.ping_start = Instant::recent();
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

    let myentity = if let Some(myentity) = content.myentity {
        myentity
    } else {
        return Ok(());
    };

    content.refresh_map = false;

    let mut entity_to_remove = Vec::with_capacity(1000);

    for (global_key, _) in world.entities.iter().filter(|(key, data)| {
        if *key != myentity {
            match data {
                Entity::Player(e_data) => {
                    content.map.map_pos.checkdistance(e_data.pos.map) > 1
                }
                Entity::Npc(e_data) => {
                    content.map.map_pos.checkdistance(e_data.pos.map) > 1
                }
                Entity::MapItem(e_data) => {
                    content.map.map_pos.checkdistance(e_data.pos.map) > 1
                }
                Entity::None => false,
            }
        } else {
            false
        }
    }) {
        entity_to_remove.push(global_key);
    }

    for entity in entity_to_remove {
        let entity_kind = world.get_kind(entity)?;
        match entity_kind {
            EntityKind::Player => {
                unload_player(world, systems, content, entity)?;
                content.players.borrow_mut().swap_remove(&entity);
            }
            EntityKind::Npc => {
                unload_npc(world, systems, content, entity)?;
                content.npcs.borrow_mut().swap_remove(&entity);
            }
            EntityKind::MapItem => {
                unload_mapitems(world, systems, content, entity)?;
                content.mapitems.borrow_mut().swap_remove(&entity);
            }
            EntityKind::None => {}
        }
    }

    Ok(())
}
