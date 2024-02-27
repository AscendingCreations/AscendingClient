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

use inventory::*;
use profile::*;
use setting::*;

#[derive(PartialEq, Eq, Clone, Copy, Sequence)]
pub enum Window {
    Inventory,
    Profile,
    Setting,
}

pub struct Interface {
    menu_button: [Button; 3],
    did_button_click: bool,
    inventory: Inventory,
    profile: Profile,
    setting: Setting,
    window_order: Vec<Window>,
    drag_window: Option<Window>,
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
            window_order: Vec::new(),
            drag_window: None,
        }
    }

    pub fn unload(&mut self, systems: &mut DrawSetting) {
        self.menu_button.iter_mut().for_each(|button| {
            button.unload(systems);
        });
        self.inventory.unload(systems);
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
                        push_interface(game_content, systems, result, screen_pos);
                    }
                }
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
                    }
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
                    }
                }
                game_content.interface.drag_window = None;
            }
        }
    }

    pub fn key_input(
        _game_content: &mut GameContent,
        _world: &mut World,
        _systems: &mut DrawSetting,
        _event: &KeyEvent,
    ) {

    }

    pub fn hover_buttons(
        game_content: &mut GameContent,
        systems: &mut DrawSetting,
        screen_pos: Vec2,
    ) {
        for button in game_content.interface.menu_button.iter_mut() {
            if is_within_area(screen_pos, Vec2::new(button.pos.x, button.pos.y), button.size) {
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
            if is_within_area(screen_pos, Vec2::new(button.pos.x, button.pos.y), button.size) {
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

    if game_content.interface.inventory.visible && is_within_area(screen_pos,
        game_content.interface.inventory.pos,
        game_content.interface.inventory.size) {
        max_z_order = game_content.interface.inventory.z_order;
        selected_window = Some(Window::Inventory);
    }
    if game_content.interface.profile.visible && is_within_area(screen_pos,
        game_content.interface.profile.pos,
        game_content.interface.profile.size) {
        let z_order = game_content.interface.profile.z_order;
        if z_order > max_z_order {
            max_z_order = z_order;
            selected_window = Some(Window::Profile);
        }
    }
    if game_content.interface.setting.visible && is_within_area(screen_pos,
        game_content.interface.setting.pos,
        game_content.interface.setting.size) {
        let z_order = game_content.interface.setting.z_order;
        if z_order > max_z_order {
            //max_z_order = z_order;
            selected_window = Some(Window::Setting);
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
    }
    game_content.interface.window_order.insert(0, window);
    adjust_window_zorder(game_content, systems);
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
    }
    if let Some(index) = game_content.interface
        .window_order.iter().position(|&wndw| wndw == window) {
        let _ = game_content.interface.window_order.remove(index);
    }
}

fn push_interface(
    game_content: &mut GameContent,
    systems: &mut DrawSetting,
    window: Window,
    screen_pos: Vec2,
) {
    game_content.interface.drag_window = Some(window);
    match window {
        Window::Inventory => game_content.interface.inventory.hold_window(screen_pos),
        Window::Profile => game_content.interface.profile.hold_window(screen_pos),
        Window::Setting => game_content.interface.setting.hold_window(screen_pos),
    }

    if game_content.interface.window_order[0] == window {
        return;
    }
    if let Some(index) = game_content.interface
        .window_order.iter().position(|&wndw| wndw == window) {
        let wndw = game_content.interface.window_order.remove(index);
        game_content.interface.window_order.insert(0, wndw);
    }
    adjust_window_zorder(game_content, systems);
}

fn adjust_window_zorder(
    game_content: &mut GameContent,
    systems: &mut DrawSetting,
) {
    let mut order = 99.0;
    for wndw in game_content.interface.window_order.iter() {
        match wndw {
            Window::Inventory => game_content.interface.inventory.set_z_order(systems, order),
            Window::Profile => game_content.interface.profile.set_z_order(systems, order),
            Window::Setting => game_content.interface.setting.set_z_order(systems, order),
        }
        order -= 1.0;
    }

    print_z_order(game_content);
}

pub fn create_menu_button(systems: &mut DrawSetting) -> [Button; 3] {
    let button_properties = ButtonRect {
        rect_color: Color::rgba(80, 80, 80, 255),
        got_border: true,
        border_color: Color::rgba(40, 40, 40, 255),
        border_radius: 8.0,
        hover_change: ButtonChangeType::ColorChange(Color::rgba(135, 135, 135, 255)),
        click_change: ButtonChangeType::ColorChange(Color::rgba(200, 200, 200, 255)),
    };
    let mut image_properties = ButtonContentImg {
        res: systems.resource.button_icon.allocation,
        pos: Vec3::new(4.0, 4.0, ORDER_GUI_BUTTON_DETAIL),
        uv: Vec2::new(0.0, 0.0),
        size: Vec2::new(32.0, 32.0),
        hover_change: ButtonChangeType::None,
        click_change: ButtonChangeType::None,
    };

    let character_button = Button::new(systems,
        ButtonType::Rect(button_properties.clone()),
        ButtonContentType::Image(image_properties.clone()),
        Vec3::new(systems.size.width - 140.0, 10.0, ORDER_GUI_BUTTON),
        Vec2::new(40.0, 40.0),
        0,
        true,
    );
    image_properties.uv.x = 32.0;
    let inventory_button = Button::new(systems,
        ButtonType::Rect(button_properties.clone()),
        ButtonContentType::Image(image_properties.clone()),
        Vec3::new(systems.size.width - 95.0, 10.0, ORDER_GUI_BUTTON),
        Vec2::new(40.0, 40.0),
        0,
        true,
    );
    image_properties.uv.x = 64.0;
    let setting_button = Button::new(systems,
        ButtonType::Rect(button_properties.clone()),
        ButtonContentType::Image(image_properties.clone()),
        Vec3::new(systems.size.width - 50.0, 10.0, ORDER_GUI_BUTTON),
        Vec2::new(40.0, 40.0),
        0,
        true,
    );
    
    [character_button, inventory_button, setting_button]
}



// TEST //
pub fn print_z_order(game_content: &mut GameContent) {
    for wndw in all::<Window>().collect::<Vec<_>>() {
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
    }
}