use graphics::*;

pub mod content_input;

pub use content_input::*;

use crate::{
    content::*,
    gfx_order::*,
    DrawSetting,
    SCREEN_HEIGHT, SCREEN_WIDTH,
    interface::*,
};
use hecs::World;

pub enum WindowType {
    None,
    Login,
    Register,
}

pub struct MenuContent {
    bg: usize,
    cur_window: WindowType,
    window: Vec<usize>,
    label: Vec<usize>,
    button: Vec<Button>,
    checkbox: Vec<Checkbox>,
    pub did_button_click: bool,
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
            cur_window: WindowType::None,
            window: Vec::new(),
            label: Vec::new(),
            button: Vec::new(),
            checkbox: Vec::new(),
            did_button_click: false,
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
        self.checkbox.iter_mut().for_each(|checkbox| {
            checkbox.unload(systems);
        });
        self.window = Vec::new();
        self.button = Vec::new();
        self.label = Vec::new();
        self.checkbox = Vec::new();
    }
}

pub fn create_window(systems: &mut DrawSetting, content: &mut MenuContent, window_type: WindowType) {
    content.cur_window = window_type;
    content.window.iter().for_each(|gfx_index| {
        systems.gfx.remove_gfx(*gfx_index);
    });
    content.button.iter_mut().for_each(|button| {
        button.unload(systems);
    });
    content.label.iter().for_each(|gfx_index| {
        systems.gfx.remove_gfx(*gfx_index);
    });
    content.checkbox.iter_mut().for_each(|checkbox| {
        checkbox.unload(systems);
    });
    content.window = Vec::new();
    content.button = Vec::new();
    content.label = Vec::new();
    content.checkbox = Vec::new();

    let screen_size = Vec2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

    match content.cur_window {
        WindowType::Login => {
            let size = Vec2::new(348.0, 226.0);
            let pos = Vec2::new((screen_size.x - size.x) * 0.5, 80.0);

            let mut menu_rect = Rect::new(&mut systems.renderer, 0);
            menu_rect.set_position(Vec3::new(pos.x - 1.0, pos.y - 1.0, MENU_WINDOW))
                .set_size(size + 2.0)
                .set_color(Color::rgba(160, 160, 160, 255))
                .set_border_color(Color::rgba(10, 10, 10, 255))
                .set_border_width(1.0);
            content.window.push(systems.gfx.add_rect(menu_rect, 0));

            let mut header_rect = Rect::new(&mut systems.renderer, 0);
            header_rect.set_position(Vec3::new(pos.x, pos.y + 196.0, MENU_WINDOW_CONTENT))
                .set_size(Vec2::new(size.x, 30.0))
                .set_color(Color::rgba(120, 120, 120, 255));
            content.window.push(systems.gfx.add_rect(header_rect, 0));

            for index in 0..2 {
                let mut labelbox = Rect::new(&mut systems.renderer, 0);
                let mut textbox = Rect::new(&mut systems.renderer, 0);
                let addy = match index {
                    1 => 123.0,
                    _ => 154.0,
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

            let button = Button::new(systems,
                ButtonType::Rect(
                    ButtonRect {
                        rect_color: Color::rgba(100, 100, 100, 255),
                        got_border: true,
                        border_color: Color::rgba(70, 70, 70, 255),
                        hover_change: ButtonChangeType::ColorChange(Color::rgba(180, 180, 180, 255)),
                        click_change: ButtonChangeType::ColorChange(Color::rgba(40, 40, 40, 255)),
                    }),
                ButtonContentType::Text(
                    ButtonContentText {
                        text: "Login".to_string(),
                        pos: Vec3::new(0.0, 7.0, MENU_WINDOW_CONTENT),
                        color: Color::rgba(200, 200, 200, 255),
                        render_layer: 1,
                        hover_change: ButtonChangeType::ColorChange(Color::rgba(80, 80, 80, 255)),
                        click_change: ButtonChangeType::ColorChange(Color::rgba(170, 170, 170, 255))
                    }),
                Vec3::new(pos.x + 104.0, pos.y + 45.0, MENU_WINDOW_CONTENT),
                Vec2::new(140.0, 34.0),
                0);
            content.button.push(button);

            let button = Button::new(systems,
                ButtonType::None,
                ButtonContentType::Text(
                    ButtonContentText {
                        text: "Register".to_string(),
                        pos: Vec3::new(0.0, 0.0, MENU_WINDOW_CONTENT),
                        color: Color::rgba(80, 80, 80, 255),
                        render_layer: 1,
                        hover_change: ButtonChangeType::ColorChange(Color::rgba(220, 220, 220, 255)),
                        click_change: ButtonChangeType::ColorChange(Color::rgba(80, 80, 80, 255)),
                    }),
                Vec3::new(pos.x + 104.0, pos.y + 19.0, MENU_WINDOW_CONTENT),
                Vec2::new(140.0, 20.0),
                0);
            content.button.push(button);

            let checkbox = Checkbox::new(systems,
                CheckboxType::Rect(
                    Color::rgba(100, 100, 100, 255),
                    true,
                    Color::rgba(50, 50, 50, 255),
                    CheckType::RectColor(
                        Color::rgba(200, 200, 200, 255)
                    )
                ),
                Vec3::new(pos.x + 116.0, pos.y + 92.0, MENU_WINDOW_CONTENT),
                Vec2::new(24.0, 24.0),
                0,
                None);
            content.checkbox.push(checkbox);
        }
        WindowType::Register => {
            let size = Vec2::new(348.0, 375.0);
            let pos = Vec2::new((screen_size.x - size.x) * 0.5, 20.0);

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
                ButtonType::Rect(
                    ButtonRect {
                        rect_color: Color::rgba(100, 100, 100, 255),
                        got_border: true,
                        border_color: Color::rgba(70, 70, 70, 255),
                        hover_change: ButtonChangeType::ColorChange(Color::rgba(180, 180, 180, 255)),
                        click_change: ButtonChangeType::ColorChange(Color::rgba(40, 40, 40, 255)),
                    }),
                ButtonContentType::Text(
                    ButtonContentText {
                        text: "Register".to_string(),
                        pos: Vec3::new(0.0, 7.0, MENU_WINDOW_CONTENT),
                        color: Color::rgba(200, 200, 200, 255),
                        render_layer: 1,
                        hover_change: ButtonChangeType::ColorChange(Color::rgba(80, 80, 80, 255)),
                        click_change: ButtonChangeType::ColorChange(Color::rgba(170, 170, 170, 255))
                    }),
                Vec3::new(pos.x + 104.0, pos.y + 45.0, MENU_WINDOW_CONTENT),
                Vec2::new(140.0, 34.0),
                0);
            content.button.push(button);

            let button = Button::new(systems,
                ButtonType::None,
                ButtonContentType::Text(
                    ButtonContentText {
                        text: "Sign In".to_string(),
                        pos: Vec3::new(0.0, 0.0, MENU_WINDOW_CONTENT),
                        color: Color::rgba(80, 80, 80, 255),
                        render_layer: 1,
                        hover_change: ButtonChangeType::ColorChange(Color::rgba(220, 220, 220, 255)),
                        click_change: ButtonChangeType::ColorChange(Color::rgba(80, 80, 80, 255)),
                    }),
                Vec3::new(pos.x + 104.0, pos.y + 19.0, MENU_WINDOW_CONTENT),
                Vec2::new(140.0, 20.0),
                0);
            content.button.push(button);

            let button = Button::new(systems,
                ButtonType::Rect(
                    ButtonRect {
                        rect_color: Color::rgba(100, 100, 100, 255),
                        got_border: true,
                        border_color: Color::rgba(70, 70, 70, 255),
                        hover_change: ButtonChangeType::ColorChange(Color::rgba(180, 180, 180, 255)),
                        click_change: ButtonChangeType::ColorChange(Color::rgba(40, 40, 40, 255)),
                    }),
                ButtonContentType::Image(
                    ButtonContentImg {
                        res: systems.resource.selection_arrow.allocation,
                        pos: Vec3::new(0.0, 0.0, MENU_WINDOW_CONTENT_DETAIL),
                        uv: Vec2::new(0.0, 0.0),
                        size: Vec2::new(24.0, 24.0),
                        hover_change: ButtonChangeType::None,
                        click_change: ButtonChangeType::None
                    }),
                Vec3::new(pos.x + 142.0, pos.y + 118.0, MENU_WINDOW_CONTENT),
                Vec2::new(24.0, 24.0),
                0);
            content.button.push(button);

            let button = Button::new(systems,
                ButtonType::Rect(
                    ButtonRect {
                        rect_color: Color::rgba(100, 100, 100, 255),
                        got_border: true,
                        border_color: Color::rgba(70, 70, 70, 255),
                        hover_change: ButtonChangeType::ColorChange(Color::rgba(180, 180, 180, 255)),
                        click_change: ButtonChangeType::ColorChange(Color::rgba(40, 40, 40, 255)),
                    }),
                ButtonContentType::Image(
                    ButtonContentImg {
                        res: systems.resource.selection_arrow.allocation,
                        pos: Vec3::new(0.0, 0.0, MENU_WINDOW_CONTENT_DETAIL),
                        uv: Vec2::new(24.0, 0.0),
                        size: Vec2::new(24.0, 24.0),
                        hover_change: ButtonChangeType::None,
                        click_change: ButtonChangeType::None
                    }),
                Vec3::new(pos.x + 282.0, pos.y + 118.0, MENU_WINDOW_CONTENT),
                Vec2::new(24.0, 24.0),
                0);
            content.button.push(button);
        }
        _ => {}
    }
}
