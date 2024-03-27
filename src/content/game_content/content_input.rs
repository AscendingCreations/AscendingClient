use graphics::*;
use hecs::World;
use winit::{event::*, keyboard::*};

use crate::{
    content::*, socket::*, Alert, ContentType, MouseInputType, SystemHolder,
    Tooltip,
};

use super::{
    KEY_ATTACK, KEY_MOVEDOWN, KEY_MOVELEFT, KEY_MOVERIGHT, KEY_MOVEUP,
    KEY_PICKUP,
};

impl GameContent {
    pub fn mouse_input(
        content: &mut Content,
        world: &mut World,
        systems: &mut SystemHolder,
        socket: &mut Socket,
        alert: &mut Alert,
        tooltip: &mut Tooltip,
        input_type: MouseInputType,
        screen_pos: Vec2,
    ) {
        if alert.visible {
            alert.alert_mouse_input(
                systems,
                socket,
                input_type.clone(),
                tooltip,
                screen_pos,
            );
            return;
        }

        if Interface::mouse_input(
            &mut content.game_content.interface,
            world,
            systems,
            socket,
            alert,
            input_type.clone(),
            screen_pos,
        ) {
            return;
        }

        if let MouseInputType::MouseLeftDown = input_type {
            let target_entity = find_entity(
                world,
                systems,
                &mut content.game_content,
                screen_pos,
            );
            if let Some(entity) = target_entity {
                content
                    .game_content
                    .target
                    .set_target(socket, systems, &entity);
            } else {
                content.game_content.target.clear_target(socket, systems);
            }
        }
    }

    pub fn key_input(
        content: &mut Content,
        world: &mut World,
        systems: &mut SystemHolder,
        socket: &mut Socket,
        alert: &mut Alert,
        event: &KeyEvent,
    ) {
        if alert.visible {
            alert.alert_key_input(systems, event);
            return;
        }

        Interface::key_input(&mut content.game_content, world, systems, event);

        match event.physical_key {
            PhysicalKey::Code(KeyCode::ArrowUp) => {
                content.game_content.keyinput[KEY_MOVEUP] =
                    event.state.is_pressed();
            }
            PhysicalKey::Code(KeyCode::ArrowDown) => {
                content.game_content.keyinput[KEY_MOVEDOWN] =
                    event.state.is_pressed();
            }
            PhysicalKey::Code(KeyCode::ArrowLeft) => {
                content.game_content.keyinput[KEY_MOVELEFT] =
                    event.state.is_pressed();
            }
            PhysicalKey::Code(KeyCode::ArrowRight) => {
                content.game_content.keyinput[KEY_MOVERIGHT] =
                    event.state.is_pressed();
            }
            PhysicalKey::Code(KeyCode::ControlLeft) => {
                content.game_content.keyinput[KEY_ATTACK] =
                    event.state.is_pressed();
            }
            PhysicalKey::Code(KeyCode::Space) => {
                content.game_content.keyinput[KEY_PICKUP] =
                    event.state.is_pressed();
            }
            _ => {}
        }

        if event.state.is_pressed() {
            #[allow(clippy::single_match)]
            match event.physical_key {
                PhysicalKey::Code(KeyCode::F1) => {
                    if let Some(entity) = content.game_content.myentity {
                        let pos = world.get_or_panic::<Position>(&entity);
                        let _ = send_admincommand(
                            socket,
                            AdminCommand::SpawnNpc(0, pos),
                        );
                    }
                }
                _ => {}
            }
        }
    }
}
