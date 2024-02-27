use hecs::World;

use crate::{
    content::*,
    DrawSetting,
};

#[derive(Copy, Clone, Debug, Default)]
pub struct LoopTimer {
    player_tmr: f32,
    input_tmr: f32,
}

pub fn game_loop(
    world: &mut World,
    systems: &mut DrawSetting,
    content: &mut Content,
    seconds: f32,
    loop_timer: &mut LoopTimer
) {
    match &mut content.holder {
        ContentHolder::Game(data) => {
            update_camera(world, data, systems);
    
            if seconds > loop_timer.player_tmr {
                update_player(world, systems, data);
                loop_timer.player_tmr = seconds + 0.01;
            }

            if seconds > loop_timer.input_tmr {
                data.handle_key_input(world, systems);
                loop_timer.input_tmr = seconds + 0.032;
            }
        }
        ContentHolder::Menu(_content) => {}
    }
}