use hecs::World;

use crate::{
    content::*,
    DrawSetting,
};

#[derive(Copy, Clone, Debug, Default)]
pub struct LoopTimer {
    player_tmr: f32,
    tmr100: f32,
    tmr500: f32,
    tmr1000: f32,
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
        }
        ContentHolder::Menu(_content) => {}
    }
}