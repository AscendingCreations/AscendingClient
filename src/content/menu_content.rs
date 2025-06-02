use graphics::{cosmic_text::Attrs, *};

pub mod content_input;
pub mod login;
pub mod register;

pub use content_input::*;
pub use login::*;
pub use register::*;

use crate::{
    SystemHolder, content::*, data_types::*, is_within_area, widget::*,
};

pub enum WindowType {
    None,
    Login,
    Register,
}

pub struct MenuContent {
    bg: GfxType,
    cur_window: WindowType,

    login: Login,
    register: Register,

    selected_textbox: Option<usize>,

    pub content_data: usize,

    pub did_button_click: bool,
    pub did_checkbox_click: bool,
}

impl MenuContent {
    pub fn new(systems: &mut SystemHolder) -> Self {
        let bg_image = Image::new(
            Some(systems.resource.menu_bg.allocation),
            &mut systems.renderer,
            Vec3::new(0.0, 0.0, ORDER_MENU_BG),
            Vec2::new(800.0, 600.0),
            Vec4::new(0.0, 0.0, 800.0, 600.0),
            0,
        );

        let bg = systems.gfx.add_image(bg_image, 0, "Menu BG", true);

        MenuContent {
            bg,
            cur_window: WindowType::None,
            login: Login::new(systems),
            register: Register::new(systems),

            did_button_click: false,
            did_checkbox_click: false,
            selected_textbox: None,
            content_data: 0,
        }
    }

    pub fn show(&mut self, systems: &mut SystemHolder) {
        systems.gfx.set_visible(&self.bg, true);
        create_window(systems, self, WindowType::Login);
    }

    pub fn hide(&mut self, systems: &mut SystemHolder) {
        systems.gfx.set_visible(&self.bg, false);
        systems.caret.index = None;
        self.clear_window(systems)
    }

    pub fn clear_window(&mut self, systems: &mut SystemHolder) {
        self.login.set_visible(systems, false);
        self.register.set_visible(systems, false);
        self.selected_textbox = None;
        self.content_data = 0;
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
