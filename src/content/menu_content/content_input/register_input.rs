use graphics::*;
use hecs::World;
use input::Key;
use log::error;
use regex::Regex;
use winit::keyboard::NamedKey;

use crate::{
    button, content::*, logic::*, socket::*, Alert, AlertIndex, AlertType,
    ContentType, MouseInputType, SystemHolder, Tooltip, APP_MAJOR, APP_MINOR,
    APP_REV,
};

pub fn register_mouse_input(
    menu_content: &mut MenuContent,
    _world: &mut World,
    systems: &mut SystemHolder,
    socket: &mut Socket,
    alert: &mut Alert,
    tooltip: &mut Tooltip,
    input_type: MouseInputType,
    screen_pos: Vec2,
) {
    match input_type {
        MouseInputType::MouseMove => {
            hover_buttons(menu_content, systems, tooltip, screen_pos);
            hover_checkbox(menu_content, systems, tooltip, screen_pos);
            hover_textbox(menu_content, systems, tooltip, screen_pos);
        }
        MouseInputType::MouseLeftDown => {
            let button_index = click_buttons(menu_content, systems, screen_pos);
            if let Some(index) = button_index {
                menu_content.did_button_click = true;
                trigger_button(menu_content, systems, socket, alert, index);
            }

            let checkbox_index =
                click_checkbox(menu_content, systems, screen_pos);
            if let Some(index) = checkbox_index {
                menu_content.did_checkbox_click = true;
                trigger_checkbox(menu_content, systems, index);
            }

            click_textbox(menu_content, systems, screen_pos);
        }
        MouseInputType::MouseLeftDownMove => {
            hold_move_textbox(menu_content, systems, screen_pos);
        }
        MouseInputType::MouseRelease => {
            reset_buttons(menu_content, systems);
            reset_checkbox(menu_content, systems);
            release_textbox(menu_content);
        }
        _ => {}
    }
}

pub fn register_key_input(
    menu_content: &mut MenuContent,
    _world: &mut World,
    systems: &mut SystemHolder,
    key: &Key,
    pressed: bool,
) {
    if let Some(textbox_index) = menu_content.selected_textbox {
        menu_content.textbox[textbox_index]
            .enter_text(systems, key, pressed, false);
    }
}

fn trigger_button(
    menu_content: &mut MenuContent,
    systems: &mut SystemHolder,
    socket: &mut Socket,
    alert: &mut Alert,
    index: usize,
) {
    match index {
        0 => {
            // Register
            let email_regex = Regex::new(
                r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
            ).unwrap();

            if menu_content.textbox[0].text != menu_content.textbox[1].text {
                alert.show_alert(
                    systems,
                    AlertType::Inform,
                    "Email did not match".into(),
                    "Alert Message".into(),
                    250,
                    AlertIndex::None,
                    false,
                );
                return;
            }

            if menu_content.textbox[2].text != menu_content.textbox[3].text {
                alert.show_alert(
                    systems,
                    AlertType::Inform,
                    "Password did not match".into(),
                    "Alert Message".into(),
                    250,
                    AlertIndex::None,
                    false,
                );
                return;
            }

            let email = menu_content.textbox[0].text.clone();
            let password = menu_content.textbox[2].text.clone();
            let username = menu_content.textbox[4].text.clone();

            if !username.chars().all(is_name_acceptable)
                || !password.chars().all(is_password_acceptable)
            {
                alert.show_alert(
                    systems,
                    AlertType::Inform,
                    "Username or Password contains unaccepted Characters"
                        .into(),
                    "Alert Message".into(),
                    250,
                    AlertIndex::None,
                    false,
                );
                return;
            }

            if username.len() >= 64 {
                alert.show_alert(
                    systems,
                    AlertType::Inform,
                    "Username has too many Characters, 64 Characters Max"
                        .into(),
                    "Alert Message".into(),
                    250,
                    AlertIndex::None,
                    false,
                );
                return;
            }

            if password.len() >= 128 {
                alert.show_alert(
                    systems,
                    AlertType::Inform,
                    "Password has too many Characters, 128 Characters Max"
                        .into(),
                    "Alert Message".into(),
                    250,
                    AlertIndex::None,
                    false,
                );
                return;
            }

            if !email_regex.is_match(&email) {
                alert.show_alert(
                    systems,
                    AlertType::Inform,
                    "Email must be an actual email.".into(),
                    "Alert Message".into(),
                    250,
                    AlertIndex::None,
                    false,
                );
                return;
            }

            match send_register(
                socket,
                username,
                password,
                email,
                menu_content.content_data as u8,
                (APP_MAJOR, APP_MINOR, APP_REV),
            ) {
                Ok(_) => {}
                Err(e) => {
                    error!("send_register error: {:?}", e);
                    alert.show_alert(
                        systems,
                        AlertType::Inform,
                        "Server is offline".into(),
                        "Alert Message".into(),
                        250,
                        AlertIndex::None,
                        false,
                    );
                }
            }
        }
        1 => {
            // Sign In
            create_window(systems, menu_content, WindowType::Login);
        }
        2 => {
            // Sprite Left
            menu_content.content_data =
                menu_content.content_data.saturating_sub(1).max(0);
            systems.gfx.set_image(
                menu_content.image[0],
                systems.resource.players[menu_content.content_data].allocation,
            );
            systems.gfx.set_text(
                &mut systems.renderer,
                menu_content.unique_label[0],
                &format!("{}", menu_content.content_data),
            );
        }
        3 => {
            // Sprite Right
            menu_content.content_data =
                menu_content.content_data.saturating_add(1).min(2);
            systems.gfx.set_image(
                menu_content.image[0],
                systems.resource.players[menu_content.content_data].allocation,
            );
            systems.gfx.set_text(
                &mut systems.renderer,
                menu_content.unique_label[0],
                &format!("{}", menu_content.content_data),
            );
        }
        _ => {}
    }
}

fn trigger_checkbox(
    _menu_content: &mut MenuContent,
    _systems: &mut SystemHolder,
    _index: usize,
) {
    /*match index {
        _ => {}
    }*/
}
