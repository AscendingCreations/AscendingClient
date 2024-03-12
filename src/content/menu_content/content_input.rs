use graphics::*;
use winit::{
    event::*,
    keyboard::*,
};
use hecs::World;

use crate::{content::*, socket::*, Alert, ContentType, DrawSetting, MouseInputType, Tooltip};

mod register_input;
mod login_input;

use register_input::*;
use login_input::*;

impl MenuContent {
    pub fn mouse_input(
        content: &mut Content,
        world: &mut World,
        systems: &mut DrawSetting,
        socket: &mut Socket,
        alert: &mut Alert,
        tooltip: &mut Tooltip,
        input_type: MouseInputType,
        screen_pos: Vec2,
    ) {
        if alert.visible {
            alert.alert_mouse_input(systems, input_type, screen_pos);
            return;
        }
        
        match content.menu_content.cur_window {
            WindowType::Register => {
                register_mouse_input(&mut content.menu_content, world, systems, socket, alert, tooltip, input_type, screen_pos);
            }
            WindowType::Login => {
                login_mouse_input(&mut content.menu_content, world, systems, socket, alert, tooltip, input_type, screen_pos);
            }
            _ => {}
        }
    }

    pub fn key_input(
        content: &mut Content,
        world: &mut World,
        systems: &mut DrawSetting,
        _socket: &mut Socket,
        _alert: &mut Alert,
        event: &KeyEvent,
    ) {
        match content.menu_content.cur_window {
            WindowType::Register => {
                register_key_input(&mut content.menu_content, world, systems, event);
            }
            WindowType::Login => {
                login_key_input(&mut content.menu_content, world, systems, event);
            }
            _ => {}
        }
    }
}