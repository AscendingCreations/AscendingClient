use graphics::*;

use input::Key;
use winit::{event_loop::ActiveEventLoop, keyboard::NamedKey};

use crate::{
    content::*,
    socket::{self, *},
    Alert, ContentType, MouseInputType, SystemHolder, Tooltip,
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
        elwt: &ActiveEventLoop,
        alert: &mut Alert,
        tooltip: &mut Tooltip,
        input_type: MouseInputType,
        screen_pos: Vec2,
    ) -> Result<()> {
        if alert.visible {
            return alert.alert_mouse_input(
                systems, socket, elwt, input_type, tooltip, screen_pos,
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
        socket: &mut Socket,
        alert: &mut Alert,
        key: &Key,
        pressed: bool,
    ) {
        if alert.visible {
            alert.alert_key_input(systems, key, pressed);
            return;
        }

        match content.menu_content.cur_window {
            WindowType::Register => {
                register_key_input(
                    &mut content.menu_content,
                    world,
                    systems,
                    socket,
                    alert,
                    key,
                    pressed,
                );
            }
            WindowType::Login => {
                login_key_input(
                    &mut content.menu_content,
                    world,
                    systems,
                    socket,
                    alert,
                    key,
                    pressed,
                );
            }
            _ => {}
        }
    }
}
