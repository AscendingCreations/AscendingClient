use graphics::*;
use winit::{
    event::*,
    keyboard::*,
};
use hecs::World;

use crate::{ContentType, DrawSetting, content::*, MouseInputType};

impl GameContent {
    pub fn mouse_input(
        _content: &mut Content,
        _world: &mut World,
        _systems: &mut DrawSetting,
        _input_type: MouseInputType,
        _screen_pos: Vec2,
    ) {
        
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
                    content.switch_content(world, systems, ContentType::Menu);
                    return;
                }
                _ => {}
            }
        }
    }
}