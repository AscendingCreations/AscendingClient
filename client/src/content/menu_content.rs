use graphics::*;
use winit::window::Window;

use crate::{
    content::*,
    gfx_order::*,
    DrawSetting,
    SCREEN_HEIGHT, SCREEN_WIDTH,
    interface::*,
};
use hecs::World;

pub enum WindowType {
    Login,
    Register,
}

pub struct MenuContent {
    bg: usize,
    window: Vec<usize>,
    button: Vec<Button>,
}

impl MenuContent {
    pub fn new(_world: &mut World, systems: &mut DrawSetting) -> Self {
        let mut bg_image = Image::new(Some(systems.resource.menu_bg.allocation),
            &mut systems.renderer, 0);
        bg_image.pos = Vec3::new(0.0, 0.0, MENU_BG);
        bg_image.hw = Vec2::new(800.0, 600.0);
        bg_image.uv = Vec4::new(0.0, 0.0, 800.0, 600.0);
        let bg = systems.gfx.add_image(bg_image, 0);

        let mut content = MenuContent {
            bg,
            window: Vec::new(),
            button: Vec::new(),
        };

        create_window(systems, &mut content, WindowType::Register);

        content
    }

    pub fn unload(&mut self, _world: &mut World, systems: &mut DrawSetting) {
        systems.gfx.remove_gfx(self.bg);
        self.window.iter().for_each(|gfx_index| {
            systems.gfx.remove_gfx(*gfx_index);
        });
        self.button.iter_mut().for_each(|button| {
            button.unload(systems);
        });
    }
}

pub fn create_window(systems: &mut DrawSetting, content: &mut MenuContent, window_type: WindowType) {
    content.window.iter().for_each(|gfx_index| {
        systems.gfx.remove_gfx(*gfx_index);
    });
    content.button.iter_mut().for_each(|button| {
        button.unload(systems);
    });
    content.window = Vec::new();
    content.button = Vec::new();

    let screen_size = Vec2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

    match window_type {
        WindowType::Login => {

        }
        WindowType::Register => {
            let size = Vec2::new(348.0, 375.0);
            let pos = Vec2::new((screen_size.x - size.x) * 0.5, 50.0);

            let mut menu_rect = Rect::new(&mut systems.renderer, 0);
            menu_rect.set_position(Vec3::new(pos.x - 1.0, pos.y - 1.0, MENU_WINDOW))
                .set_size(size + 2.0)
                .set_color(Color::rgba(160, 160, 160, 255))
                .set_border_color(Color::rgba(10, 10, 10, 255))
                .set_border_width(1.0);
            content.window.push(systems.gfx.add_rect(menu_rect, 0));
            
            let mut header_rect = Rect::new(&mut systems.renderer, 0);
            header_rect.set_position(Vec3::new(pos.x, pos.y + 345.0, MENU_WINDOW_CONTENT))
                .set_size(Vec2::new(size.x, 30.0))
                .set_color(Color::rgba(120, 120, 120, 255));
            content.window.push(systems.gfx.add_rect(header_rect, 0));

            for index in 0..5 {
                let mut labelbox = Rect::new(&mut systems.renderer, 0);
                let mut textbox = Rect::new(&mut systems.renderer, 0);
                let addy = match index {
                    1 => 278.0,
                    2 => 247.0,
                    3 => 222.0,
                    4 => 191.0,
                    _ => 303.0,
                };
                labelbox.set_position(Vec3::new(pos.x + 24.0, pos.y + addy, MENU_WINDOW_CONTENT))
                    .set_size(Vec2::new(116.0, 24.0))
                    .set_color(Color::rgba(208, 208, 208, 255));
                textbox.set_position(Vec3::new(pos.x + 140.0, pos.y + addy, MENU_WINDOW_CONTENT))
                    .set_size(Vec2::new(184.0, 24.0))
                    .set_color(Color::rgba(90, 90, 90, 255));
                content.window.push(systems.gfx.add_rect(labelbox, 0));
                content.window.push(systems.gfx.add_rect(textbox, 0));
            }

            let mut sprite_bg = Rect::new(&mut systems.renderer, 0);
            sprite_bg.set_position(Vec3::new(pos.x + 34.0, pos.y + 98.0, MENU_WINDOW_CONTENT))
                .set_size(Vec2::new(80.0, 80.0))
                .set_color(Color::rgba(120, 120, 120, 255));
            content.window.push(systems.gfx.add_rect(sprite_bg, 0));

            let button = Button::new(systems,
                ButtonType::Rect(Color::rgba(100, 100, 100, 255),
                    true,
                    Color::rgba(70, 70, 70, 255),
                    ButtonChangeType::ColorChange(Color::rgba(255, 255, 255, 255)),
                    ButtonChangeType::ColorChange(Color::rgba(255, 255, 255, 255)),
                ),
                ButtonContentType::Text("Register".to_string(),
                        Vec3::new(0.0, 7.0, MENU_WINDOW_CONTENT),
                        Color::rgba(200, 200, 200, 255),
                        1,
                        ButtonChangeType::ColorChange(Color::rgba(255, 255, 255, 255)),
                        ButtonChangeType::ColorChange(Color::rgba(170, 170, 170, 255))),
                Vec3::new(pos.x + 104.0, pos.y + 45.0, MENU_WINDOW_CONTENT),
                Vec2::new(140.0, 34.0),
                0);
            content.button.push(button);
        }
    }
}
