use graphics::*;
use winit::{
    event::*,
    keyboard::*,
};
use hecs::World;

use crate::{content::{*, menu_content::content_input::*}, Direction, DrawSetting, socket::*};

pub enum MouseInputType {
    MouseLeftDown,
    MouseLeftDownMove,
    MouseMove,
    MouseRelease,
}

pub fn handle_mouse_input(
    world: &mut World,
    systems: &mut DrawSetting,
    socket: &mut Socket,
    input_type: MouseInputType,
    mouse_pos: &Vec2,
    content: &mut Content,
) {
    // We convert the mouse position to render position as the y pos increase upward
    let screen_pos = Vec2::new(
        mouse_pos.x,
        systems.size.height - mouse_pos.y,
    );

    let content_type = content.content_type.clone();
    match content_type {
        ContentType::Game => {
            GameContent::mouse_input(content, world, systems, socket, input_type, screen_pos);
        }
        ContentType::Menu => {
            MenuContent::mouse_input(content, world, systems, socket, input_type, screen_pos);
        }
    }
}

pub fn handle_key_input(
    world: &mut World,
    systems: &mut DrawSetting,
    socket: &mut Socket,
    content: &mut Content,
    event: &KeyEvent,
) {
    let content_type = content.content_type.clone();
    match content_type {
        ContentType::Game => {
            GameContent::key_input(content, world, systems, socket, event);
        }
        ContentType::Menu => {
            MenuContent::key_input(content, world, systems, socket, event);
        }
    }
}