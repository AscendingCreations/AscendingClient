use graphics::*;
use winit::{
    event::*,
    keyboard::*,
};
use hecs::World;

use crate::{content::*, Direction, DrawSetting};

pub fn handle_key_input(
    world: &mut World,
    systems: &mut DrawSetting,
    content: &mut Content,
    event: &KeyEvent,
) {
    match &mut content.holder {
        ContentHolder::Game(data) => {
            if event.state.is_pressed() {
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::ArrowUp) => {
                        data.move_player(world, &Direction::Up);
                    }
                    PhysicalKey::Code(KeyCode::ArrowDown) => {
                        data.move_player(world, &Direction::Down);
                    }
                    PhysicalKey::Code(KeyCode::ArrowLeft) => {
                        data.move_player(world, &Direction::Left);
                    }
                    PhysicalKey::Code(KeyCode::ArrowRight) => {
                        data.move_player(world, &Direction::Right);
                    }
                    
                    PhysicalKey::Code(KeyCode::KeyW) => {
                        data.move_other_player(world, &Direction::Up);
                    }
                    PhysicalKey::Code(KeyCode::KeyS) => {
                        data.move_other_player(world, &Direction::Down);
                    }
                    PhysicalKey::Code(KeyCode::KeyA) => {
                        data.move_other_player(world, &Direction::Left);
                    }
                    PhysicalKey::Code(KeyCode::KeyD) => {
                        data.move_other_player(world, &Direction::Right);
                    }

                    PhysicalKey::Code(KeyCode::F1) => {
                        content.switch_content(world, systems, ContentType::Menu);
                        return;
                    }
                    _ => {}
                }
            }
        }
        ContentHolder::Menu(_data) => {
            if event.state.is_pressed() {
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::F1) => {
                        content.switch_content(world, systems, ContentType::Game);
                    }
                    _ => {}
                }
            }
        }
    }
}