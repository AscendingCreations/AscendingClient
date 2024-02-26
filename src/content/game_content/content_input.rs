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
        if let ContentHolder::Game(data) = &mut content.holder {
            Interface::mouse_input(data, world, systems, input_type, screen_pos);
        }
    }

    pub fn key_input(
        content: &mut Content,
        world: &mut World,
        systems: &mut DrawSetting,
        event: &KeyEvent,
    ) {
        if let ContentHolder::Game(data) = &mut content.holder {
            Interface::key_input(data, world, systems, event);
        }

        if event.state.is_pressed() {
            match event.physical_key {
                PhysicalKey::Code(KeyCode::ArrowUp) => {
                    if let ContentHolder::Game(data) = &mut content.holder {
                        data.move_player(world, systems, &crate::Direction::Up);
                    }
                }
                PhysicalKey::Code(KeyCode::ArrowDown) => {
                    if let ContentHolder::Game(data) = &mut content.holder {
                        data.move_player(world, systems, &crate::Direction::Down);
                    }
                }
                PhysicalKey::Code(KeyCode::ArrowLeft) => {
                    if let ContentHolder::Game(data) = &mut content.holder {
                        data.move_player(world, systems, &crate::Direction::Left);
                    }
                }
                PhysicalKey::Code(KeyCode::ArrowRight) => {
                    if let ContentHolder::Game(data) = &mut content.holder {
                        data.move_player(world, systems, &crate::Direction::Right);
                    }
                }
                PhysicalKey::Code(KeyCode::F1) => {
                    content.switch_content(world, systems, ContentType::Menu);
                    return;
                }
                _ => {}
            }
        }
    }
}