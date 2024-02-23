use graphics::*;
use winit::{
    event::*,
    keyboard::*,
};
use hecs::World;

use crate::{ContentType, DrawSetting, content::*, MouseInputType};

impl GameContent {
    pub fn mouse_input(
        content: &mut Content,
        world: &mut World,
        systems: &mut DrawSetting,
        input_type: MouseInputType,
        screen_pos: Vec2,
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