use graphics::*;
use hecs::World;
use winit::{event::*, keyboard::*};

use crate::{
    content::{menu_content::content_input::*, *},
    socket::*,
    Alert, Direction, DrawSetting, Tooltip,
};

#[derive(Clone)]
#[allow(clippy::enum_variant_names)]
pub enum MouseInputType {
    MouseLeftDown,
    MouseLeftDownMove,
    MouseMove,
    MouseRelease,
}

#[allow(clippy::too_many_arguments)]
pub fn handle_mouse_input(
    world: &mut World,
    systems: &mut DrawSetting,
    socket: &mut Socket,
    input_type: MouseInputType,
    mouse_pos: &Vec2,
    content: &mut Content,
    alert: &mut Alert,
    tooltip: &mut Tooltip,
) {
    // We convert the mouse position to render position as the y pos increase upward
    let screen_pos = Vec2::new(mouse_pos.x, systems.size.height - mouse_pos.y);

    tooltip.check_tooltip(systems, screen_pos);

    match content.content_type {
        ContentType::Game => {
            GameContent::mouse_input(
                content, world, systems, socket, alert, tooltip, input_type,
                screen_pos,
            );
        }
        ContentType::Menu => {
            MenuContent::mouse_input(
                content, world, systems, socket, alert, tooltip, input_type,
                screen_pos,
            );
        }
    }
}

pub fn handle_key_input(
    world: &mut World,
    systems: &mut DrawSetting,
    socket: &mut Socket,
    content: &mut Content,
    alert: &mut Alert,
    event: &KeyEvent,
) {
    match content.content_type {
        ContentType::Game => {
            GameContent::key_input(
                content, world, systems, socket, alert, event,
            );
        }
        ContentType::Menu => {
            MenuContent::key_input(
                content, world, systems, socket, alert, event,
            );
        }
    }
}
