use graphics::{cosmic_text::Attrs, *};

pub mod content_input;
pub mod login;
pub mod register;

pub use content_input::*;
pub use login::*;
pub use register::*;

use crate::{
    content::*, data_types::*, is_within_area, widget::*, SystemHolder,
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

    login: Login,
    register: Register,

    selected_textbox: Option<usize>,
    pub server_status: usize,

    pub content_data: usize,

    pub did_button_click: bool,
    pub did_checkbox_click: bool,
}

impl MenuContent {
    pub fn new(systems: &mut SystemHolder) -> Self {
        let mut bg_image = Image::new(
            Some(systems.resource.menu_bg.allocation),
            &mut systems.renderer,
            0,
        );
        bg_image.pos = Vec3::new(0.0, 0.0, ORDER_MENU_BG);
        bg_image.hw = Vec2::new(800.0, 600.0);
        bg_image.uv = Vec4::new(0.0, 0.0, 800.0, 600.0);
        let bg = systems.gfx.add_image(bg_image, 0, "Menu BG".into());

        let label_size = Vec2::new(
            200.0 * systems.scale as f32,
            20.0 * systems.scale as f32,
        )
        .floor();
        let text = create_label(
            systems,
            Vec3::new(10.0, 10.0, ORDER_SERVER_STATUS),
            label_size,
            Bounds::new(10.0, 10.0, label_size.x + 10.0, label_size.y + 10.0),
            Color::rgba(220, 220, 220, 255),
        );
        let server_status =
            systems.gfx.add_text(text, 1, "Menu Server Status".into());

        MenuContent {
            bg,
            cur_window: WindowType::None,
            login: Login::new(systems),
            register: Register::new(systems),

            did_button_click: false,
            did_checkbox_click: false,
            selected_textbox: None,
            content_data: 0,
            server_status,
        }
    }

    pub fn show(&mut self, systems: &mut SystemHolder) {
        systems.gfx.set_visible(self.bg, true);
        systems.gfx.set_visible(self.server_status, true);
        create_window(systems, self, WindowType::Login);
    }

    pub fn hide(&mut self, systems: &mut SystemHolder) {
        systems.gfx.set_visible(self.bg, false);
        systems.gfx.set_visible(self.server_status, false);
        systems.caret.index = None;
        self.clear_window(systems)
    }

    pub fn clear_window(&mut self, systems: &mut SystemHolder) {
        self.login.set_visible(systems, false);
        self.register.set_visible(systems, false);
        self.selected_textbox = None;
        self.content_data = 0;
    }

    pub fn set_status_online(&mut self, systems: &mut SystemHolder) {
        systems.gfx.set_rich_text(
            &mut systems.renderer,
            self.server_status,
            [
                (
                    "Server Status: ",
                    Attrs::new().color(Color::rgba(220, 220, 220, 255)),
                ),
                ("Online", Attrs::new().color(Color::rgba(10, 200, 20, 255))),
            ],
        );
    }

    pub fn set_status_offline(&mut self, systems: &mut SystemHolder) {
        systems.gfx.set_rich_text(
            &mut systems.renderer,
            self.server_status,
            [
                (
                    "Server Status: ",
                    Attrs::new().color(Color::rgba(220, 220, 220, 255)),
                ),
                ("Offline", Attrs::new().color(Color::rgba(200, 10, 20, 255))),
            ],
        );
    }
}

pub fn create_window(
    systems: &mut SystemHolder,
    content: &mut MenuContent,
    window_type: WindowType,
) {
    content.cur_window = window_type;
    content.clear_window(systems);

    match content.cur_window {
        WindowType::Login => {
            content.login.set_visible(systems, true);
        }
        WindowType::Register => {
            content.register.set_visible(systems, true);
        }
        _ => {}
    }
}
