use graphics::*;

use input::Key;
use log::error;
use winit::keyboard::NamedKey;

use crate::{
    APP_MAJOR, APP_MINOR, APP_REV, Alert, AlertIndex, AlertType, ContentType,
    MouseInputType, SystemHolder, Tooltip, alert, button,
    content::*,
    fade::*,
    logic::FloatFix,
    socket::{self, *},
};

pub fn login_mouse_input(
    menu_content: &mut MenuContent,
    _world: &mut World,
    systems: &mut SystemHolder,
    socket: &mut Poller,
    alert: &mut Alert,
    tooltip: &mut Tooltip,
    input_type: MouseInputType,
    screen_pos: Vec2,
) {
    match input_type {
        MouseInputType::MouseMove => {
            menu_content
                .login
                .hover_buttons(systems, tooltip, screen_pos);
            menu_content
                .login
                .hover_checkbox(systems, tooltip, screen_pos);
        }
        MouseInputType::MouseLeftDown => {
            let button_index =
                menu_content.login.click_buttons(systems, screen_pos);

            if let Some(index) = button_index {
                menu_content.did_button_click = true;
                trigger_button(menu_content, systems, socket, alert, index);
            }

            if menu_content.login.click_checkbox(systems, screen_pos) {
                menu_content.login.checkbox.set_click(systems, true);
                menu_content.did_checkbox_click = true;
                trigger_checkbox(menu_content, systems, 0);
            }

            click_login_textbox(menu_content, systems, screen_pos);
        }
        MouseInputType::MouseLeftDownMove => {
            hold_move_login_textbox(menu_content, systems, screen_pos);
        }
        MouseInputType::MouseRelease => {
            reset_login_buttons(menu_content, systems);
            reset_login_checkbox(menu_content, systems);
            release_login_textbox(menu_content);
        }
        _ => {}
    }
}

pub fn login_key_input(
    menu_content: &mut MenuContent,
    _world: &mut World,
    systems: &mut SystemHolder,
    socket: &mut Poller,
    alert: &mut Alert,
    key: &Key,
    pressed: bool,
) {
    if pressed {
        match key {
            Key::Named(NamedKey::Tab) => match menu_content.selected_textbox {
                None => {
                    menu_content.login.textbox[0].set_select(systems, true);
                    menu_content.selected_textbox = Some(0);
                }
                Some(index) => {
                    menu_content.login.textbox[index]
                        .set_select(systems, false);

                    let mut next_index = index + 1;

                    if next_index >= menu_content.login.textbox.len() {
                        next_index = 0;
                    }

                    menu_content.login.textbox[next_index]
                        .set_select(systems, true);
                    menu_content.selected_textbox = Some(next_index);
                }
            },
            Key::Named(NamedKey::Enter) => {
                match menu_content.selected_textbox {
                    None => {
                        menu_content.login.textbox[0].set_select(systems, true);
                        menu_content.selected_textbox = Some(0);
                    }
                    Some(index) => {
                        menu_content.login.textbox[index]
                            .set_select(systems, false);

                        let next_index = index + 1;

                        if next_index >= menu_content.login.textbox.len() {
                            menu_content.selected_textbox = None;
                            trigger_button(
                                menu_content,
                                systems,
                                socket,
                                alert,
                                0,
                            );
                        } else {
                            menu_content.login.textbox[next_index]
                                .set_select(systems, true);
                            menu_content.selected_textbox = Some(next_index);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    if let Some(textbox_index) = menu_content.selected_textbox {
        menu_content.login.textbox[textbox_index]
            .enter_text(systems, key, pressed, false);
    }
}

fn trigger_button(
    menu_content: &mut MenuContent,
    systems: &mut SystemHolder,
    socket: &mut Poller,
    alert: &mut Alert,
    index: usize,
) {
    match index {
        0 => {
            // Login
            let username = menu_content.login.textbox[0].text.clone();
            let password = menu_content.login.textbox[1].text.clone();

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
            systems.config.save_password = menu_content.login.checkbox.value;
            menu_content
                .login
                .checkbox
                .set_value(systems, menu_content.login.checkbox.value);
        }
        _ => {}
    }
}
