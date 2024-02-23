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
            hover_checkbox(menu_content, systems, screen_pos);
        }
        MouseInputType::MouseLeftDown => {
            let button_index = click_buttons(menu_content, systems, screen_pos);
            if let Some(index) = button_index {
                menu_content.did_button_click = true;
                trigger_button(menu_content, systems, index);
            }

            let checkbox_index = click_checkbox(menu_content, systems, screen_pos);
            if let Some(index) = checkbox_index {
                menu_content.did_checkbox_click = true;
                trigger_checkbox(menu_content, systems, index);
            }
        }
        MouseInputType::MouseRelease => {
            reset_buttons(menu_content, systems);
            reset_checkbox(menu_content, systems);
        }
        _ => {}
    }
}

fn trigger_button(
    menu_content: &mut MenuContent,
    systems: &mut DrawSetting,
    index: usize,
) {
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

fn trigger_checkbox(
    _menu_content: &mut MenuContent,
    _systems: &mut DrawSetting,
    index: usize,
) {
    match index {
        0 => { // Remember Account
            println!("Remember Account");
        }
        _ => {}
    }
}