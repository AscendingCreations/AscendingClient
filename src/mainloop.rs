use hecs::World;

use crate::{
    content::*, BufferTask, DrawSetting
};

#[derive(Copy, Clone, Debug, Default)]
pub struct LoopTimer {
    player_tmr: f32,
    npc_tmr: f32,
    input_tmr: f32,
}

pub fn game_loop(
    world: &mut World,
    systems: &mut DrawSetting,
    content: &mut Content,
    buffer: &mut BufferTask,
    seconds: f32,
    loop_timer: &mut LoopTimer
) {
    match &mut content.holder {
        ContentHolder::Game(data) => {
            update_camera(world, data, systems);
    
            if seconds > loop_timer.player_tmr {
                update_player(world, systems, data, buffer, seconds);
                loop_timer.player_tmr = seconds + 0.01;
            }

            if seconds > loop_timer.npc_tmr {
                update_npc(world, systems, data, seconds);
                loop_timer.npc_tmr = seconds + 0.01;
            }

            if seconds > loop_timer.input_tmr {
                data.handle_key_input(world, systems, seconds);
                loop_timer.input_tmr = seconds + 0.032;
            }
        }
        ContentHolder::Menu(_content) => {}
    }
}