use graphics::*;
use cosmic_text::{Attrs, Metrics};
use enum_iterator::{all, Sequence};

use winit::{
    event::*,
    keyboard::*,
};

use crate::{
    gfx_order::*, is_within_area, widget::*, DrawSetting, GameContent, MouseInputType
};
use hecs::World;

mod inventory;
mod profile;
mod setting;
mod screen;
mod chatbox;

use inventory::*;
use profile::*;
use setting::*;
use screen::*;
use chatbox::*;

#[derive(PartialEq, Eq, Clone, Copy, Sequence, Debug)]
pub enum Window {
    Inventory,
    Profile,
    Setting,
    Chatbox,
}

pub enum SelectedTextbox {
    None,
    Chatbox,
}

pub struct Interface {
    menu_button: [Button; 3],
    did_button_click: bool,
    inventory: Inventory,
    profile: Profile,
    setting: Setting,
    chatbox: Chatbox,
    window_order: Vec<(Window, usize)>,
    drag_window: Option<Window>,
    selected_textbox: SelectedTextbox,
}

impl Interface {
    pub fn new(systems: &mut DrawSetting) -> Self {
        let menu_button = create_menu_button(systems);

        Interface {
            menu_button,
            did_button_click: false,
            inventory: Inventory::new(systems),
            profile: Profile::new(systems),
            setting: Setting::new(systems),
            chatbox: Chatbox::new(systems),
            window_order: 
                vec![
                    (Window::Inventory, 0),
                    (Window::Profile, 1),
                    (Window::Setting, 2),
                    (Window::Chatbox, 3),
                ],
            drag_window: None,
            selected_textbox: SelectedTextbox::None,
        }
    }

    pub fn unload(&mut self, systems: &mut DrawSetting) {
        self.menu_button.iter_mut().for_each(|button| {
            button.unload(systems);
        });
        self.inventory.unload(systems);
        self.profile.unload(systems);
        self.setting.unload(systems);
        self.chatbox.unload(systems);
    }

    pub fn mouse_input(
        game_content: &mut GameContent,
        _world: &mut World,
        systems: &mut DrawSetting,
        input_type: MouseInputType,
        screen_pos: Vec2,
    ) {
        match input_type {
            MouseInputType::MouseMove => {
                Interface::hover_buttons(game_content, systems, screen_pos);
                
                if game_content.interface.setting.visible {
                    if game_content.interface.setting.sfx_scroll.in_scroll(screen_pos) {
                        game_content.interface.setting.sfx_scroll.set_hover(systems, true);
                    } else {
                        game_content.interface.setting.sfx_scroll.set_hover(systems, false);
                    }
                    if game_content.interface.setting.bgm_scroll.in_scroll(screen_pos) {
                        game_content.interface.setting.bgm_scroll.set_hover(systems, true);
                    } else {
                        game_content.interface.setting.bgm_scroll.set_hover(systems, false);
                    }
                }
                if game_content.interface.chatbox.scrollbar.in_scroll(screen_pos) {
                    game_content.interface.chatbox.scrollbar.set_hover(systems, true);
                } else {
                    game_content.interface.chatbox.scrollbar.set_hover(systems, false);
                }
                game_content.interface.chatbox.hover_buttons(systems, screen_pos);
            }
            MouseInputType::MouseLeftDown => {
                let button_index = Interface::click_buttons(game_content, systems, screen_pos);
                if let Some(index) = button_index {
                    game_content.interface.did_button_click = true;
                    trigger_button(game_content, systems, index);
                }

                if game_content.interface.drag_window.is_none() {
                    let window = find_window(game_content, screen_pos);
                    if let Some(result) = window {
                        hold_interface(game_content, systems, result, screen_pos);
                    }
                }
                
                if game_content.interface.setting.visible &&
                    game_content.interface.drag_window.is_none()
                {
                    if game_content.interface.setting.sfx_scroll.in_scroll(screen_pos) {
                        game_content.interface.setting.sfx_scroll.set_hold(systems, true, screen_pos);
                    }
                    if game_content.interface.setting.bgm_scroll.in_scroll(screen_pos) {
                        game_content.interface.setting.bgm_scroll.set_hold(systems, true, screen_pos);
                    }
                }
                if game_content.interface.chatbox.scrollbar.in_scroll(screen_pos) {
                    game_content.interface.chatbox.scrollbar.set_hold(systems, true, screen_pos);
                }

                let chatbox_button_index = game_content.interface.chatbox.click_buttons(systems, screen_pos);
                if let Some(_index) = chatbox_button_index {
                    game_content.interface.chatbox.did_button_click = true;
                }

                Interface::click_textbox(game_content, systems, screen_pos);
            }
            MouseInputType::MouseLeftDownMove => {
                if let Some(window) = &game_content.interface.drag_window {
                    match window {
                        Window::Inventory =>
                            game_content.interface.inventory.move_window(systems, screen_pos),
                        Window::Profile =>
                            game_content.interface.profile.move_window(systems, screen_pos),
                        Window::Setting =>
                            game_content.interface.setting.move_window(systems, screen_pos),
                        Window::Chatbox =>
                            game_content.interface.chatbox.move_window(systems, screen_pos),
                    }
                } else {
                    if game_content.interface.setting.visible {
                        game_content.interface.setting.sfx_scroll.set_move_scroll(systems, screen_pos);
                        game_content.interface.setting.bgm_scroll.set_move_scroll(systems, screen_pos);
                    }
                    game_content.interface.chatbox.scrollbar.set_move_scroll(systems, screen_pos);
                }
            }
            MouseInputType::MouseRelease => {
                Interface::reset_buttons(game_content, systems);
                if let Some(window) = &game_content.interface.drag_window {
                    match window {
                        Window::Inventory =>
                            game_content.interface.inventory.release_window(),
                        Window::Profile =>
                            game_content.interface.profile.release_window(),
                        Window::Setting =>
                            game_content.interface.setting.release_window(),
                        Window::Chatbox =>
                            game_content.interface.chatbox.release_window(),
                    }
                }
                game_content.interface.drag_window = None;

                if game_content.interface.setting.visible {
                    game_content.interface.setting.sfx_scroll.set_hold(systems, false, screen_pos);
                    game_content.interface.setting.bgm_scroll.set_hold(systems, false, screen_pos);
                }
                game_content.interface.chatbox.scrollbar.set_hold(systems, false, screen_pos);
                game_content.interface.chatbox.reset_buttons(systems);
            }
        }
    }

    pub fn key_input(
        game_content: &mut GameContent,
        _world: &mut World,
        systems: &mut DrawSetting,
        event: &KeyEvent,
    ) {
        match game_content.interface.selected_textbox {
            SelectedTextbox::Chatbox => {
                game_content.interface.chatbox.textbox.enter_text(systems, event);
            }
            _ => {}
        }
    }

    pub fn hover_buttons(
        game_content: &mut GameContent,
        systems: &mut DrawSetting,
        screen_pos: Vec2,
    ) {
        for button in game_content.interface.menu_button.iter_mut() {
            if is_within_area(screen_pos, 
                Vec2::new(button.base_pos.x + button.adjust_pos.x, 
                    button.base_pos.y + button.adjust_pos.y), button.size) {
                button.set_hover(systems, true);
            } else {
                button.set_hover(systems, false);
            }
        }
    }
    
    pub fn click_buttons(
        game_content: &mut GameContent,
        systems: &mut DrawSetting,
        screen_pos: Vec2,
    ) -> Option<usize> {
        let mut button_found = None;
        for (index, button) in game_content.interface.menu_button.iter_mut().enumerate() {
            if is_within_area(screen_pos, 
                Vec2::new(button.base_pos.x + button.adjust_pos.x, 
                    button.base_pos.y + button.adjust_pos.y), button.size) {
                button.set_click(systems, true);
                button_found = Some(index)
            }
        }
        button_found
    }
    
    pub fn reset_buttons(
        game_content: &mut GameContent,
        systems: &mut DrawSetting,
    ) {
        if !game_content.interface.did_button_click {
            return;
        }
        game_content.interface.did_button_click = false;
    
        game_content.interface.menu_button.iter_mut().for_each(|button| {
            button.set_click(systems, false);
        });
    }

    pub fn click_textbox(
        game_content: &mut GameContent,
        systems: &mut DrawSetting,
        screen_pos: Vec2,
    ) {
        if is_within_area(screen_pos, 
            Vec2::new(game_content.interface.chatbox.textbox.pos.x, 
                game_content.interface.chatbox.textbox.pos.y), 
                game_content.interface.chatbox.textbox.size) {
            game_content.interface.chatbox.textbox.set_select(systems, true);
            game_content.interface.selected_textbox = SelectedTextbox::Chatbox;
            return;
        }
        
        match game_content.interface.selected_textbox {
            SelectedTextbox::Chatbox => game_content.interface.chatbox.textbox.set_select(systems, false),
            _ => {}
        }
        game_content.interface.selected_textbox = SelectedTextbox::None;
    }
}

fn trigger_button(
    game_content: &mut GameContent,
    systems: &mut DrawSetting,
    index: usize,
) {
    match index {
        0 => {
            if game_content.interface.profile.visible {
                close_interface(game_content, systems, Window::Profile);
            } else {
                open_interface(game_content, systems, Window::Profile);
            }
        }
        1 => {
            if game_content.interface.inventory.visible {
                close_interface(game_content, systems, Window::Inventory);
            } else {
                open_interface(game_content, systems, Window::Inventory);
            }
        }
        2 => {
            if game_content.interface.setting.visible {
                close_interface(game_content, systems, Window::Setting);
            } else {
                open_interface(game_content, systems, Window::Setting);
            }
        }
        _ => {}
    }
}

fn find_window(game_content: &mut GameContent, screen_pos: Vec2) -> Option<Window> {
    let mut max_z_order: f32 = 0.0;
    let mut selected_window = None;

    if game_content.interface.inventory.in_window(screen_pos) {
        max_z_order = game_content.interface.inventory.z_order;
        selected_window = Some(Window::Inventory);
    }
    if game_content.interface.profile.in_window(screen_pos) {
        let z_order = game_content.interface.profile.z_order;
        if z_order > max_z_order {
            max_z_order = z_order;
            selected_window = Some(Window::Profile);
        }
    }
    if game_content.interface.setting.in_window(screen_pos) {
        let z_order = game_content.interface.setting.z_order;
        if z_order > max_z_order {
            max_z_order = z_order;
            selected_window = Some(Window::Setting);
        }
    }
    if game_content.interface.chatbox.in_window(screen_pos) {
        let z_order = game_content.interface.chatbox.z_order;
        if z_order > max_z_order {
            //max_z_order = z_order;
            selected_window = Some(Window::Chatbox);
        }
    }
    selected_window
}

fn open_interface(
    game_content: &mut GameContent,
    systems: &mut DrawSetting,
    window: Window,
) {
    match window {
        Window::Inventory => {
            if game_content.interface.inventory.visible { return; }
            game_content.interface.inventory.set_visible(systems, true);
        }
        Window::Profile => {
            if game_content.interface.profile.visible { return; }
            game_content.interface.profile.set_visible(systems, true);
        }
        Window::Setting => {
            if game_content.interface.setting.visible { return; }
            game_content.interface.setting.set_visible(systems, true);
        }
        _ => {}
    }
    interface_set_to_first(game_content, systems, window);
}

fn close_interface(
    game_content: &mut GameContent,
    systems: &mut DrawSetting,
    window: Window,
) {
    match window {
        Window::Inventory => {
            if !game_content.interface.inventory.visible { return; }
            game_content.interface.inventory.set_visible(systems, false);
        }
        Window::Profile => {
            if !game_content.interface.profile.visible { return; }
            game_content.interface.profile.set_visible(systems, false);
        }
        Window::Setting => {
            if !game_content.interface.setting.visible { return; }
            game_content.interface.setting.set_visible(systems, false);
        }
        _ => {}
    }
    interface_set_to_last(game_content, systems, window);
}

fn hold_interface(
    game_content: &mut GameContent,
    systems: &mut DrawSetting,
    window: Window,
    screen_pos: Vec2,
) {
    interface_set_to_first(game_content, systems, window);
    match window {
        Window::Inventory => {
            if !game_content.interface.inventory.can_hold(screen_pos) {
                return;
            }
            game_content.interface.inventory.hold_window(screen_pos);
        }
        Window::Profile => {
            if !game_content.interface.profile.can_hold(screen_pos) {
                return;
            }
            game_content.interface.profile.hold_window(screen_pos);
        }
        Window::Setting => {
            if !game_content.interface.setting.can_hold(screen_pos) {
                return;
            }
            game_content.interface.setting.hold_window(screen_pos);
        }
        Window::Chatbox => {
            if !game_content.interface.chatbox.can_hold(screen_pos) {
                return;
            }
            game_content.interface.chatbox.hold_window(screen_pos);
        }
    }
    game_content.interface.drag_window = Some(window);
}

fn interface_set_to_first(
    game_content: &mut GameContent,
    systems: &mut DrawSetting,
    window: Window,
) {
    if let Some(index) = 
        game_content.interface.window_order
            .iter()
            .position(|&wndw| wndw.0 == window)
    {
        if game_content.interface.window_order[index].1 == 0 {
            return;
        }
        for i in 0..index {
            game_content.interface.window_order[i].1 = i.saturating_add(1);
        }
        game_content.interface.window_order[index].1 = 0;
    }
    game_content.interface.window_order
        .sort_by(|a, b| a.1.cmp(&b.1));
    adjust_window_zorder(game_content, systems);
}

fn interface_set_to_last(
    game_content: &mut GameContent,
    systems: &mut DrawSetting,
    window: Window,
) {
    let last_index = game_content.interface.window_order.len() - 1;
    if let Some(index) = 
        game_content.interface.window_order
            .iter()
            .position(|&wndw| wndw.0 == window)
    {
        if game_content.interface.window_order[index].1 == last_index {
            return;
        }
        for i in index..(last_index + 1) {
            game_content.interface.window_order[i].1 = i.saturating_sub(1);
        }
        game_content.interface.window_order[index].1 = last_index;
    }
    game_content.interface.window_order
        .sort_by(|a, b| a.1.cmp(&b.1));
    adjust_window_zorder(game_content, systems);
}

fn adjust_window_zorder(
    game_content: &mut GameContent,
    systems: &mut DrawSetting,
) {
    let mut order = 99.0;
    for wndw in game_content.interface.window_order.iter() {
        match wndw.0 {
            Window::Inventory => game_content.interface.inventory.set_z_order(systems, order),
            Window::Profile => game_content.interface.profile.set_z_order(systems, order),
            Window::Setting => game_content.interface.setting.set_z_order(systems, order),
            Window::Chatbox => game_content.interface.chatbox.set_z_order(systems, order),
        }
        order -= 1.0;
    }

    print_z_order(game_content);
}





// TEST //
pub fn print_z_order(game_content: &mut GameContent) {
    println!("============");
    for data in game_content.interface.window_order.iter() {
        println!("Order: {:?}", data);
    }
    /*for wndw in all::<Window>().collect::<Vec<_>>() {
        let z_order = match wndw {
            Window::Inventory => game_content.interface.inventory.z_order,
            Window::Profile => game_content.interface.profile.z_order,
            Window::Setting => game_content.interface.setting.z_order,
        };
        match wndw {
            Window::Inventory => println!("Inventory {z_order}"),
            Window::Profile => println!("Profile {z_order}"),
            Window::Setting => println!("Setting {z_order}"),
        }
    }*/
}