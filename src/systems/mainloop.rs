use hecs::World;

use crate::{
    content::*, dir_to_enum, BufferTask, Result, Socket, SystemHolder,
};

#[derive(Copy, Clone, Debug, Default)]
pub struct LoopTimer {
    player_tmr: f32,
    npc_tmr: f32,
    input_tmr: f32,
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