use graphics::*;
use winit::{
    event::*,
    keyboard::*,
};
use hecs::World;

use crate::{button, content::*, ContentType, DrawSetting, MouseInputType, socket::*};

pub fn register_mouse_input(
    menu_content: &mut MenuContent,
    _world: &mut World,
    systems: &mut DrawSetting,
    socket: &mut Socket,
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
                trigger_button(menu_content, systems, socket, index);
            }

            let checkbox_index = click_checkbox(menu_content, systems, screen_pos);
            if let Some(index) = checkbox_index {
                menu_content.did_checkbox_click = true;
                trigger_checkbox(menu_content, systems, index);
            }

            click_textbox(menu_content, systems, screen_pos);
        }
        MouseInputType::MouseRelease => {
            reset_buttons(menu_content, systems);
            reset_checkbox(menu_content, systems);
        }
        _ => {}
    }
}

pub fn register_key_input(
    menu_content: &mut MenuContent,
    _world: &mut World,
    systems: &mut DrawSetting,
    event: &KeyEvent,
) {
    if let Some(textbox_index) = menu_content.selected_textbox {
        menu_content.textbox[textbox_index].enter_text(systems, event);
    }
}

fn trigger_button(
    menu_content: &mut MenuContent,
    systems: &mut DrawSetting,
    socket: &mut Socket,
    index: usize,
) {
    match index {
        0 => { // Register
            
            send_register(
                socket,
                menu_content.textbox[4].text.clone(),
                menu_content.textbox[2].text.clone(),
                menu_content.textbox[0].text.clone(),
                1).expect("Failed to send register");
        }
        1 => { // Sign In
            println!("Sign In");
            println!("Old Collection {:?}", systems.gfx.count_collection());
            create_window(systems, menu_content, WindowType::Login);
            println!("New Collection {:?}", systems.gfx.count_collection());
        }
        2 => { // Sprite Left
            println!("Sprite Left");
            let x: i32 = menu_content.textbox[0].text.clone().parse().unwrap();
            let y: i32 = menu_content.textbox[1].text.clone().parse().unwrap();
            let g: i32 = menu_content.textbox[2].text.clone().parse().unwrap();
            let result = get_start_map_pos(MapPosition::new(0, 0, 0), MapPosition::new(x, y, g));
            println!("Result {:?} x: {x} y: {y} g: {g}", result);
        }
        3 => { // Sprite Right
            println!("Sprite Right");
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
        _ => {}
    }
}