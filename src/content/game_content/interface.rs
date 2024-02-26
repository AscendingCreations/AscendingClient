use graphics::*;
use cosmic_text::{Attrs, Metrics};

use winit::{
    event::*,
    keyboard::*,
};

use crate::{
    interface::*,
    DrawSetting, 
    GameContent,
    MouseInputType,
    gfx_order::*,
};
use hecs::World;

pub struct Interface {
    menu_button: [Button; 3],
    did_button_click: bool,
}

impl Interface {
    pub fn new(systems: &mut DrawSetting) -> Self {
        let menu_button = create_menu_button(systems);

        Interface {
            menu_button,
            did_button_click: false,
        }
    }

    pub fn unload(&mut self, systems: &mut DrawSetting) {
        self.menu_button.iter_mut().for_each(|button| {
            button.unload(systems);
        });
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
                    println!("Button Index {index}");
                    //trigger_button(game_content, systems, index);
                }
            }
            MouseInputType::MouseRelease => {
                Interface::reset_buttons(game_content, systems);
            }
            _ => {}
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
    
    pub fn click_buttons(
        game_content: &mut GameContent,
        systems: &mut DrawSetting,
        screen_pos: Vec2,
    ) -> Option<usize> {
        let mut button_found = None;
        for (index, button) in game_content.interface.menu_button.iter_mut().enumerate() {
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
        pos: Vec3::new(4.0, 4.0, ORDER_INTERFACE_BUTTON_DETAIL),
        uv: Vec2::new(0.0, 0.0),
        size: Vec2::new(32.0, 32.0),
        hover_change: ButtonChangeType::None,
        click_change: ButtonChangeType::None,
    };

    let character_button = Button::new(systems,
        ButtonType::Rect(button_properties.clone()),
        ButtonContentType::Image(image_properties.clone()),
        Vec3::new(systems.size.width - 140.0, 10.0, ORDER_INTERFACE_BUTTON),
        Vec2::new(40.0, 40.0),
        0,
        true,
    );
    image_properties.uv.x = 32.0;
    let inventory_button = Button::new(systems,
        ButtonType::Rect(button_properties.clone()),
        ButtonContentType::Image(image_properties.clone()),
        Vec3::new(systems.size.width - 95.0, 10.0, ORDER_INTERFACE_BUTTON),
        Vec2::new(40.0, 40.0),
        0,
        true,
    );
    image_properties.uv.x = 64.0;
    let setting_button = Button::new(systems,
        ButtonType::Rect(button_properties.clone()),
        ButtonContentType::Image(image_properties.clone()),
        Vec3::new(systems.size.width - 50.0, 10.0, ORDER_INTERFACE_BUTTON),
        Vec2::new(40.0, 40.0),
        0,
        true,
    );
    
    [character_button, inventory_button, setting_button]
}