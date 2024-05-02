use graphics::*;
use hecs::World;
use input::Key;
use winit::keyboard::NamedKey;

use crate::{
    content::*,
    data_types::*,
    socket::{self, *},
    Alert, ContentType, MouseInputType, SystemHolder, Tooltip, COLOR_RED,
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
        elwt: &winit::event_loop::EventLoopWindowTarget<()>,
        alert: &mut Alert,
        tooltip: &mut Tooltip,
        input_type: MouseInputType,
        screen_pos: Vec2,
    ) -> Result<()> {
        if alert.visible {
            return alert.alert_mouse_input(
                systems,
                socket,
                elwt,
                input_type.clone(),
                tooltip,
                screen_pos,
            );
        }

        if Interface::mouse_input(
            &mut content.game_content.interface,
            world,
            systems,
            socket,
            alert,
            input_type.clone(),
            tooltip,
            screen_pos,
        )? {
            return Ok(());
        }

        if let MouseInputType::MouseLeftDown = input_type {
            let target_entity = find_entity(
                world,
                systems,
                &mut content.game_content,
                screen_pos,
            );

            if let Some(entity) = target_entity {
                if let Some(t_entity) = content.game_content.target.entity {
                    if let Ok(mut hpbar) = world.get::<&mut HPBar>(t_entity.0) {
                        content
                            .game_content
                            .target
                            .clear_target(socket, systems, &mut hpbar)?;
                    }
                    if t_entity == entity {
                        return Ok(());
                    }
                }

                content
                    .game_content
                    .target
                    .set_target(socket, systems, &entity)?;
                match world.get_or_err::<WorldEntityType>(&entity)? {
                    WorldEntityType::Player => {
                        update_player_camera(
                            world,
                            systems,
                            socket,
                            &entity,
                            &mut content.game_content,
                        )?;
                    }
                    WorldEntityType::Npc => {
                        update_npc_camera(
                            world,
                            systems,
                            &entity,
                            socket,
                            &mut content.game_content,
                        )?;
                    }
                    _ => {}
                }
            }
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
    ) -> Result<()> {
        if alert.visible {
            alert.alert_key_input(systems, key, pressed);
            return Ok(());
        }

        Interface::key_input(
            &mut content.game_content,
            world,
            systems,
            socket,
            key,
            pressed,
        )?;

        if content.game_content.interface.inventory.hold_slot.is_some()
            || content.game_content.interface.storage.hold_slot.is_some()
        {
            content.game_content.keyinput.iter_mut().for_each(|key| {
                *key = false;
            });
            return Ok(());
        }

        match key {
            Key::Named(NamedKey::ArrowUp) => {
                content.game_content.keyinput[KEY_MOVEUP] = pressed;
            }
            Key::Named(NamedKey::ArrowDown) => {
                content.game_content.keyinput[KEY_MOVEDOWN] = pressed;
            }
            Key::Named(NamedKey::ArrowLeft) => {
                content.game_content.keyinput[KEY_MOVELEFT] = pressed;
            }
            Key::Named(NamedKey::ArrowRight) => {
                content.game_content.keyinput[KEY_MOVERIGHT] = pressed;
            }
            Key::Named(NamedKey::Control) => {
                content.game_content.keyinput[KEY_ATTACK] = pressed;
            }
            Key::Named(NamedKey::Space) => {
                content.game_content.keyinput[KEY_PICKUP] = pressed;
            }
            _ => {}
        }
        Ok(())
    }
}
