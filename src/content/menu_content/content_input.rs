use graphics::*;
use hecs::World;
use winit::{event::*, keyboard::*};

use crate::{
    content::*, socket::*, Alert, ContentType, MouseInputType, SystemHolder,
    Tooltip,
};

mod login_input;
mod register_input;

use login_input::*;
use register_input::*;

impl MenuContent {
    pub fn mouse_input(
        content: &mut Content,
        world: &mut World,
        systems: &mut SystemHolder,
        socket: &mut Socket,
        alert: &mut Alert,
        tooltip: &mut Tooltip,
        input_type: MouseInputType,
        screen_pos: Vec2,
    ) -> Result<()> {
        if alert.visible {
            return alert.alert_mouse_input(
                systems, socket, input_type, tooltip, screen_pos,
            );
        }

        match content.menu_content.cur_window {
            WindowType::Register => {
                register_mouse_input(
                    &mut content.menu_content,
                    world,
                    systems,
                    socket,
                    alert,
                    tooltip,
                    input_type,
                    screen_pos,
                );
            }
            WindowType::Login => {
                login_mouse_input(
                    &mut content.menu_content,
                    world,
                    systems,
                    socket,
                    alert,
                    tooltip,
                    input_type,
                    screen_pos,
                );
            }
            _ => {}
        }

        Ok(())
    }

    pub fn key_input(
        content: &mut Content,
        world: &mut World,
        systems: &mut SystemHolder,
        _socket: &mut Socket,
        alert: &mut Alert,
        event: &KeyEvent,
    ) {
        if alert.visible {
            alert.alert_key_input(systems, event);
            return;
        }

        match content.menu_content.cur_window {
            WindowType::Register => {
                register_key_input(
                    &mut content.menu_content,
                    world,
                    systems,
                    event,
                );
            }
            WindowType::Login => {
                login_key_input(
                    &mut content.menu_content,
                    world,
                    systems,
                    event,
                );
            }
            _ => {}
        }
    }
}
