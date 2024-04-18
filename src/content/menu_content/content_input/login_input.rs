use graphics::*;
use hecs::World;
use input::Key;
use log::error;
use winit::keyboard::NamedKey;

use crate::{
    button, content::*, fade::*, logic::FloatFix, socket::*, Alert, AlertIndex,
    AlertType, ContentType, MouseInputType, SystemHolder, Tooltip, APP_MAJOR,
    APP_MINOR, APP_REV,
};

pub fn login_mouse_input(
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

pub fn login_key_input(
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
            // Login
            let username = menu_content.textbox[0].text.clone();
            let password = menu_content.textbox[1].text.clone();

            systems.config.username.clone_from(&username);
            systems.config.password.clone_from(&password);
            systems.config.save_config("settings.toml");

            match send_login(
                socket,
                username,
                password,
                (APP_MAJOR, APP_MINOR, APP_REV),
                &systems.config.reconnect_code,
            ) {
                Ok(_) => {}
                Err(e) => {
                    error!("send_login error: {:?}", e);
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
            // Register
            create_window(systems, menu_content, WindowType::Register);
        }
        _ => {}
    }
}

fn trigger_checkbox(
    menu_content: &mut MenuContent,
    systems: &mut SystemHolder,
    index: usize,
) {
    #[allow(clippy::single_match)]
    match index {
        0 => {
            // Remember Account
            systems.config.save_password = menu_content.checkbox[index].value;
        }
        _ => {}
    }
}
