use graphics::*;
use cosmic_text::{Attrs, Metrics};

use winit::{
    event::*,
    keyboard::*,
};

use crate::{
    is_within_area, widget::*, DrawSetting, GameContent, MouseInputType, interface::chatbox::*,
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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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
                    (Window::Chatbox, 0),
                    (Window::Inventory, 1),
                    (Window::Profile, 2),
                    (Window::Setting, 3),
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
        interface: &mut Interface,
        _world: &mut World,
        systems: &mut DrawSetting,
        input_type: MouseInputType,
        screen_pos: Vec2,
    ) {
        match input_type {
            MouseInputType::MouseMove => {
                Interface::hover_buttons(interface, systems, screen_pos);
                
                if interface.setting.visible {
                    if interface.setting.sfx_scroll.in_scroll(screen_pos) {
                        interface.setting.sfx_scroll.set_hover(systems, true);
                    } else {
                        interface.setting.sfx_scroll.set_hover(systems, false);
                    }
                    if interface.setting.bgm_scroll.in_scroll(screen_pos) {
                        interface.setting.bgm_scroll.set_hover(systems, true);
                    } else {
                        interface.setting.bgm_scroll.set_hover(systems, false);
                    }
                }
                if interface.chatbox.scrollbar.in_scroll(screen_pos) {
                    interface.chatbox.scrollbar.set_hover(systems, true);
                } else {
                    interface.chatbox.scrollbar.set_hover(systems, false);
                }
                interface.chatbox.hover_buttons(systems, screen_pos);
            }
            MouseInputType::MouseLeftDown => {
                let button_index = Interface::click_buttons(interface, systems, screen_pos);
                if let Some(index) = button_index {
                    interface.did_button_click = true;
                    trigger_button(interface, systems, index);
                }

                if interface.drag_window.is_none() {
                    let window = find_window(interface, screen_pos);
                    if let Some(result) = window {
                        hold_interface(interface, systems, result, screen_pos);
                    }
                }
                
                if interface.setting.visible &&
                    interface.drag_window.is_none()
                {
                    if interface.setting.sfx_scroll.in_scroll(screen_pos) {
                        interface.setting.sfx_scroll.set_hold(systems, true, screen_pos);
                    }
                    if interface.setting.bgm_scroll.in_scroll(screen_pos) {
                        interface.setting.bgm_scroll.set_hold(systems, true, screen_pos);
                    }
                }
                if interface.chatbox.scrollbar.in_scroll(screen_pos) {
                    interface.chatbox.scrollbar.set_hold(systems, true, screen_pos);
                }

                let chatbox_button_index = interface.chatbox.click_buttons(systems, screen_pos);
                if let Some(index) = chatbox_button_index {
                    interface.chatbox.did_button_click = true;
                    trigger_chatbox_button(interface, systems, index);
                }

                interface.click_textbox(systems, screen_pos);
            }
            MouseInputType::MouseLeftDownMove => {
                if let Some(window) = &interface.drag_window {
                    match window {
                        Window::Inventory =>
                            interface.inventory.move_window(systems, screen_pos),
                        Window::Profile =>
                            interface.profile.move_window(systems, screen_pos),
                        Window::Setting =>
                            interface.setting.move_window(systems, screen_pos),
                        Window::Chatbox =>
                            interface.chatbox.move_window(systems, screen_pos),
                    }
                } else {
                    if interface.setting.visible {
                        interface.setting.sfx_scroll.set_move_scroll(systems, screen_pos);
                        interface.setting.bgm_scroll.set_move_scroll(systems, screen_pos);
                    }
                    interface.chatbox.scrollbar.set_move_scroll(systems, screen_pos);
                    interface.chatbox.set_chat_scrollbar(systems, false);
                }
            }
            MouseInputType::MouseRelease => {
                interface.reset_buttons(systems);

                if let Some(window) = &interface.drag_window {
                    match window {
                        Window::Inventory =>
                            interface.inventory.release_window(),
                        Window::Profile =>
                            interface.profile.release_window(),
                        Window::Setting =>
                            interface.setting.release_window(),
                        Window::Chatbox =>
                            interface.chatbox.release_window(),
                    }
                }
                interface.drag_window = None;

                if interface.setting.visible {
                    interface.setting.sfx_scroll.set_hold(systems, false, screen_pos);
                    interface.setting.bgm_scroll.set_hold(systems, false, screen_pos);
                }
                interface.chatbox.scrollbar.set_hold(systems, false, screen_pos);
                interface.chatbox.reset_buttons(systems);
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
        interface: &mut Interface,
        systems: &mut DrawSetting,
        screen_pos: Vec2,
    ) {
        for button in interface.menu_button.iter_mut() {
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
        interface: &mut Interface,
        systems: &mut DrawSetting,
        screen_pos: Vec2,
    ) -> Option<usize> {
        let mut button_found = None;
        for (index, button) in interface.menu_button.iter_mut().enumerate() {
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
        &mut self,
        systems: &mut DrawSetting,
    ) {
        if !self.did_button_click {
            return;
        }
        self.did_button_click = false;
    
        self.menu_button.iter_mut().for_each(|button| {
            button.set_click(systems, false);
        });
    }

    pub fn click_textbox(
        &mut self,
        systems: &mut DrawSetting,
        screen_pos: Vec2,
    ) {
        if is_within_area(screen_pos, 
            Vec2::new(self.chatbox.textbox.pos.x, 
                self.chatbox.textbox.pos.y), 
                self.chatbox.textbox.size) {
            self.chatbox.textbox.set_select(systems, true);
            self.selected_textbox = SelectedTextbox::Chatbox;
            return;
        }
        
        match self.selected_textbox {
            SelectedTextbox::Chatbox => self.chatbox.textbox.set_select(systems, false),
            _ => {}
        }
        self.selected_textbox = SelectedTextbox::None;
    }
}

fn trigger_button(
    interface: &mut Interface,
    systems: &mut DrawSetting,
    index: usize,
) {
    match index {
        0 => {
            if interface.profile.visible {
                close_interface(interface, systems, Window::Profile);
            } else {
                open_interface(interface, systems, Window::Profile);
            }
        }
        1 => {
            if interface.inventory.visible {
                close_interface(interface, systems, Window::Inventory);
            } else {
                open_interface(interface, systems, Window::Inventory);
            }
        }
        2 => {
            if interface.setting.visible {
                close_interface(interface, systems, Window::Setting);
            } else {
                open_interface(interface, systems, Window::Setting);
            }
        }
        _ => {}
    }
}

fn trigger_chatbox_button(
    interface: &mut Interface,
    systems: &mut DrawSetting,
    index: usize,
) {
    match index {
        0 => { // Scroll Up
            if interface.chatbox.scrollbar.max_value <= 0 {
                return;
            }
            let scrollbar_value = 
                interface.chatbox.scrollbar.value
                    .saturating_add(1)
                    .min(interface.chatbox.scrollbar.max_value);
            interface.chatbox.scrollbar.set_value(systems, scrollbar_value);
            interface.chatbox.set_chat_scrollbar(systems, true);
        }
        1 => { // Scroll Down
            if interface.chatbox.scrollbar.max_value <= 0 {
                return;
            }
            let scrollbar_value = 
                interface.chatbox.scrollbar.value
                    .saturating_sub(1);
            interface.chatbox.scrollbar.set_value(systems, scrollbar_value);
            interface.chatbox.set_chat_scrollbar(systems, true);
        }
        2 => { // Send
            let msg = interface.chatbox.textbox.text.clone();
            interface.chatbox.add_chat(systems, (msg, COLOR_WHITE), Some(("[Sherwin]: ".to_string(), COLOR_RED)));
            interface.chatbox.textbox.set_text(systems, String::new());
        }
        _ => {}
    }
}

fn find_window(interface: &mut Interface, screen_pos: Vec2) -> Option<Window> {
    let mut max_z_order: f32 = 0.0;
    let mut selected_window = None;

    if interface.inventory.in_window(screen_pos) {
        max_z_order = interface.inventory.z_order;
        selected_window = Some(Window::Inventory);
    }
    if interface.profile.in_window(screen_pos) {
        let z_order = interface.profile.z_order;
        if z_order > max_z_order {
            max_z_order = z_order;
            selected_window = Some(Window::Profile);
        }
    }
    if interface.setting.in_window(screen_pos) {
        let z_order = interface.setting.z_order;
        if z_order > max_z_order {
            max_z_order = z_order;
            selected_window = Some(Window::Setting);
        }
    }
    if interface.chatbox.in_window(screen_pos) {
        let z_order = interface.chatbox.z_order;
        if z_order > max_z_order {
            //max_z_order = z_order;
            selected_window = Some(Window::Chatbox);
        }
    }
    selected_window
}

fn open_interface(
    interface: &mut Interface,
    systems: &mut DrawSetting,
    window: Window,
) {
    match window {
        Window::Inventory => {
            if interface.inventory.visible { return; }
            interface.inventory.set_visible(systems, true);
        }
        Window::Profile => {
            if interface.profile.visible { return; }
            interface.profile.set_visible(systems, true);
        }
        Window::Setting => {
            if interface.setting.visible { return; }
            interface.setting.set_visible(systems, true);
        }
        _ => {}
    }
    interface_set_to_first(interface, systems, window);
}

fn close_interface(
    interface: &mut Interface,
    systems: &mut DrawSetting,
    window: Window,
) {
    match window {
        Window::Inventory => {
            if !interface.inventory.visible { return; }
            interface.inventory.set_visible(systems, false);
        }
        Window::Profile => {
            if !interface.profile.visible { return; }
            interface.profile.set_visible(systems, false);
        }
        Window::Setting => {
            if !interface.setting.visible { return; }
            interface.setting.set_visible(systems, false);
        }
        _ => {}
    }
    interface_set_to_last(interface, systems, window);
}

fn hold_interface(
    interface: &mut Interface,
    systems: &mut DrawSetting,
    window: Window,
    screen_pos: Vec2,
) {
    interface_set_to_first(interface, systems, window);
    match window {
        Window::Inventory => {
            if !interface.inventory.can_hold(screen_pos) {
                return;
            }
            interface.inventory.hold_window(screen_pos);
        }
        Window::Profile => {
            if !interface.profile.can_hold(screen_pos) {
                return;
            }
            interface.profile.hold_window(screen_pos);
        }
        Window::Setting => {
            if !interface.setting.can_hold(screen_pos) {
                return;
            }
            interface.setting.hold_window(screen_pos);
        }
        Window::Chatbox => {
            if !interface.chatbox.can_hold(screen_pos) {
                return;
            }
            interface.chatbox.hold_window(screen_pos);
        }
    }
    interface.drag_window = Some(window);
}

fn interface_set_to_first(
    interface: &mut Interface,
    systems: &mut DrawSetting,
    window: Window,
) {
    if let Some(index) = 
        interface.window_order
            .iter()
            .position(|&wndw| wndw.0 == window)
    {
        if interface.window_order[index].1 == 0 {
            return;
        }
        for i in 0..index {
            interface.window_order[i].1 = i.saturating_add(1);
        }
        interface.window_order[index].1 = 0;
    }
    interface.window_order
        .sort_by(|a, b| a.1.cmp(&b.1));
    adjust_window_zorder(interface, systems);
}

fn interface_set_to_last(
    interface: &mut Interface,
    systems: &mut DrawSetting,
    window: Window,
) {
    let last_index = interface.window_order.len() - 1;
    if let Some(index) = 
        interface.window_order
            .iter()
            .position(|&wndw| wndw.0 == window)
    {
        if interface.window_order[index].1 == last_index {
            return;
        }
        for i in index..(last_index + 1) {
            interface.window_order[i].1 = i.saturating_sub(1);
        }
        interface.window_order[index].1 = last_index;
    }
    interface.window_order
        .sort_by(|a, b| a.1.cmp(&b.1));
    adjust_window_zorder(interface, systems);
}

fn adjust_window_zorder(
    interface: &mut Interface,
    systems: &mut DrawSetting,
) {
    let mut order = 0.99;
    for wndw in interface.window_order.iter() {
        match wndw.0 {
            Window::Inventory => interface.inventory.set_z_order(systems, order),
            Window::Profile => interface.profile.set_z_order(systems, order),
            Window::Setting => interface.setting.set_z_order(systems, order),
            Window::Chatbox => interface.chatbox.set_z_order(systems, order),
        }
        order -= 0.01;
    }
}