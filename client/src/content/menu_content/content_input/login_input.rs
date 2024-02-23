use graphics::*;
use winit::{
    event::*,
    keyboard::*,
};
use hecs::World;

use crate::{button, content::*, ContentType, DrawSetting, MouseInputType};

pub fn login_mouse_input(
    menu_content: &mut MenuContent,
    _world: &mut World,
    systems: &mut DrawSetting,
    input_type: MouseInputType,
    screen_pos: Vec2,
) {
    match input_type {
        MouseInputType::MouseMove => {
            hover_buttons(menu_content, systems, screen_pos);
        }
        MouseInputType::MouseLeftDown => {
            let button_index = click_buttons(menu_content, systems, screen_pos);
            if let Some(index) = button_index {
                menu_content.did_button_click = true;
                match index {
                    0 => { // Login
                        println!("Login");
                    }
                    1 => { // Register
                        println!("Register");
                        println!("Old Collection {:?}", systems.gfx.count_collection());
                        create_window(systems, menu_content, WindowType::Register);
                        println!("New Collection {:?}", systems.gfx.count_collection());
                    }
                    _ => {}
                }
            }
        }
        MouseInputType::MouseRelease => {
            reset_buttons(menu_content, systems);
        }
        _ => {}
    }
}

fn hover_buttons(
    menu_content: &mut MenuContent,
    systems: &mut DrawSetting,
    screen_pos: Vec2,
) {
    for button in menu_content.button.iter_mut() {
        if screen_pos.x >= button.pos.x &&
            screen_pos.x <= button.pos.x + button.size.x &&
            screen_pos.y >= button.pos.y &&
            screen_pos.y <= button.pos.y + button.size.y {
            button.set_hover(systems, true);
        } else {
            button.set_hover(systems, false);
        }
    }
}

fn click_buttons(
    menu_content: &mut MenuContent,
    systems: &mut DrawSetting,
    screen_pos: Vec2,
) -> Option<usize> {
    let mut button_found = None;
    for (index, button) in menu_content.button.iter_mut().enumerate() {
        if screen_pos.x >= button.pos.x &&
            screen_pos.x <= button.pos.x + button.size.x &&
            screen_pos.y >= button.pos.y &&
            screen_pos.y <= button.pos.y + button.size.y {
            button.set_click(systems, true);
            button_found = Some(index)
        }
    }
    button_found
}

fn reset_buttons(
    menu_content: &mut MenuContent,
    systems: &mut DrawSetting,
) {
    if !menu_content.did_button_click {
        return;
    }
    menu_content.did_button_click = false;

    menu_content.button.iter_mut().for_each(|button| {
        button.set_click(systems, false);
    });
}