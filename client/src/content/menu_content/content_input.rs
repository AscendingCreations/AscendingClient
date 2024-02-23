use graphics::*;
use winit::{
    event::*,
    keyboard::*,
};
use hecs::World;

use crate::{ContentType, DrawSetting, content::*, MouseInputType};

mod register_input;

use register_input::*;

impl MenuContent {
    pub fn mouse_input(
        content: &mut Content,
        world: &mut World,
        systems: &mut DrawSetting,
        input_type: MouseInputType,
        screen_pos: Vec2,
    ) {
        match &mut content.holder {
            ContentHolder::Menu(data) => {
                match data.cur_window {
                    WindowType::Register => {
                        register_mouse_input(data, world, systems, input_type, screen_pos);
                    }
                    WindowType::Login => {}
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn key_input(
        content: &mut Content,
        world: &mut World,
        systems: &mut DrawSetting,
        event: &KeyEvent,
    ) {
        if event.state.is_pressed() {
            match event.physical_key {
                PhysicalKey::Code(KeyCode::F1) => {
                    content.switch_content(world, systems, ContentType::Game);
                    return;
                }
                _ => {}
            }
        }
    }
}