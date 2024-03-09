use graphics::*;
use winit::{
    event::*,
    keyboard::*,
};
use hecs::World;

use crate::{content::*, socket::*, Alert, ContentType, DrawSetting, MouseInputType};

use super::{KEY_ATTACK, KEY_MOVEDOWN, KEY_MOVELEFT, KEY_MOVERIGHT, KEY_MOVEUP};

impl GameContent {
    pub fn mouse_input(
        content: &mut Content,
        world: &mut World,
        systems: &mut DrawSetting,
        _socket: &mut Socket,
        alert: &mut Alert,
        input_type: MouseInputType,
        screen_pos: Vec2,
    ) {
        if alert.visible {
            alert.alert_mouse_input(systems, input_type, screen_pos);
            return;
        }

        if let ContentHolder::Game(data) = &mut content.holder {
            Interface::mouse_input(&mut data.interface, world, systems, input_type, screen_pos);
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
        if let ContentHolder::Game(data) = &mut content.holder {
            Interface::key_input(data, world, systems, event);

            match event.physical_key {
                PhysicalKey::Code(KeyCode::ArrowUp) => {
                    data.keyinput[KEY_MOVEUP] = event.state.is_pressed();
                }
                PhysicalKey::Code(KeyCode::ArrowDown) => {
                    data.keyinput[KEY_MOVEDOWN] = event.state.is_pressed();
                }
                PhysicalKey::Code(KeyCode::ArrowLeft) => {
                    data.keyinput[KEY_MOVELEFT] = event.state.is_pressed();
                }
                PhysicalKey::Code(KeyCode::ArrowRight) => {
                    data.keyinput[KEY_MOVERIGHT] = event.state.is_pressed();
                }
                PhysicalKey::Code(KeyCode::Space) => {
                    data.keyinput[KEY_ATTACK] = event.state.is_pressed();
                }
                _ => {}
            }
        }

        if event.state.is_pressed() {
            match event.physical_key {
                PhysicalKey::Code(KeyCode::F1) => {
                    content.switch_content(world, systems, ContentType::Menu);
                    return;
                }
                PhysicalKey::Code(KeyCode::F5) => {
                    if let ContentHolder::Game(data) = &mut content.holder {
                        print_z_order(&mut data.interface);
                    }
                }
                _ => {}
            }
        }
    }
}