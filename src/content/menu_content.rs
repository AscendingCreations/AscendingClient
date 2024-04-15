use graphics::{cosmic_text::Attrs, *};

pub mod content_input;
pub mod login;
pub mod register;

pub use content_input::*;
pub use login::*;
pub use register::*;

use crate::{content::*, is_within_area, values::*, widget::*, SystemHolder};
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
    unique_label: Vec<usize>,
    button: Vec<Button>,
    checkbox: Vec<Checkbox>,
    textbox: Vec<Textbox>,
    selected_textbox: Option<usize>,
    image: Vec<usize>,
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
        let bg = systems.gfx.add_image(bg_image, 0);

        let text = create_label(
            systems,
            Vec3::new(10.0, 10.0, ORDER_SERVER_STATUS),
            Vec2::new(200.0, 20.0),
            Bounds::new(10.0, 10.0, 210.0, 30.0),
            Color::rgba(220, 220, 220, 255),
        );
        let server_status = systems.gfx.add_text(text, 1);

        MenuContent {
            bg,
            cur_window: WindowType::None,
            window: Vec::new(),
            label: Vec::new(),
            unique_label: Vec::new(),
            button: Vec::new(),
            checkbox: Vec::new(),
            textbox: Vec::new(),
            image: Vec::new(),
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
        self.window.iter().for_each(|gfx_index| {
            systems.gfx.remove_gfx(&mut systems.renderer, *gfx_index);
        });
        self.button.iter_mut().for_each(|button| {
            button.unload(systems);
        });
        self.label.iter().for_each(|gfx_index| {
            systems.gfx.remove_gfx(&mut systems.renderer, *gfx_index);
        });
        self.unique_label.iter().for_each(|gfx_index| {
            systems.gfx.remove_gfx(&mut systems.renderer, *gfx_index);
        });
        self.checkbox.iter_mut().for_each(|checkbox| {
            checkbox.unload(systems);
        });
        self.textbox.iter_mut().for_each(|textbox| {
            textbox.unload(systems);
        });
        self.image.iter_mut().for_each(|gfx_index| {
            systems.gfx.remove_gfx(&mut systems.renderer, *gfx_index);
        });
        self.window.clear();
        self.button.clear();
        self.label.clear();
        self.unique_label.clear();
        self.checkbox.clear();
        self.textbox.clear();
        self.image.clear();
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

pub fn hover_buttons(
    menu_content: &mut MenuContent,
    systems: &mut SystemHolder,
    tooltip: &mut Tooltip,
    screen_pos: Vec2,
) {
    for button in menu_content.button.iter_mut() {
        if is_within_area(
            screen_pos,
            Vec2::new(
                button.base_pos.x + button.adjust_pos.x,
                button.base_pos.y + button.adjust_pos.y,
            ),
            button.size,
        ) {
            button.set_hover(systems, true);

            if let Some(msg) = &button.tooltip {
                tooltip.init_tooltip(systems, screen_pos, msg.clone());
            }
        } else {
            button.set_hover(systems, false);
        }
    }
}

pub fn click_buttons(
    menu_content: &mut MenuContent,
    systems: &mut SystemHolder,
    screen_pos: Vec2,
) -> Option<usize> {
    let mut button_found = None;
    for (index, button) in menu_content.button.iter_mut().enumerate() {
        if is_within_area(
            screen_pos,
            Vec2::new(
                button.base_pos.x + button.adjust_pos.x,
                button.base_pos.y + button.adjust_pos.y,
            ),
            button.size,
        ) {
            button.set_click(systems, true);
            button_found = Some(index)
        }
    }
    button_found
}

pub fn reset_buttons(
    menu_content: &mut MenuContent,
    systems: &mut SystemHolder,
) {
    if !menu_content.did_button_click {
        return;
    }
    menu_content.did_button_click = false;

    menu_content.button.iter_mut().for_each(|button| {
        button.set_click(systems, false);
    });
}

pub fn hover_checkbox(
    menu_content: &mut MenuContent,
    systems: &mut SystemHolder,
    tooltip: &mut Tooltip,
    screen_pos: Vec2,
) {
    for checkbox in menu_content.checkbox.iter_mut() {
        if is_within_area(
            screen_pos,
            Vec2::new(
                checkbox.base_pos.x + checkbox.adjust_pos.x,
                checkbox.base_pos.y + checkbox.adjust_pos.y,
            ),
            Vec2::new(
                checkbox.box_size.x + checkbox.adjust_x,
                checkbox.box_size.y,
            ),
        ) {
            checkbox.set_hover(systems, true);

            if let Some(msg) = &checkbox.tooltip {
                tooltip.init_tooltip(systems, screen_pos, msg.clone());
            }
        } else {
            checkbox.set_hover(systems, false);
        }
    }
}

pub fn click_checkbox(
    menu_content: &mut MenuContent,
    systems: &mut SystemHolder,
    screen_pos: Vec2,
) -> Option<usize> {
    let mut checkbox_found = None;
    for (index, checkbox) in menu_content.checkbox.iter_mut().enumerate() {
        if is_within_area(
            screen_pos,
            Vec2::new(
                checkbox.base_pos.x + checkbox.adjust_pos.x,
                checkbox.base_pos.y + checkbox.adjust_pos.y,
            ),
            Vec2::new(
                checkbox.box_size.x + checkbox.adjust_x,
                checkbox.box_size.y,
            ),
        ) {
            checkbox.set_click(systems, true);
            checkbox_found = Some(index)
        }
    }
    checkbox_found
}

pub fn reset_checkbox(
    menu_content: &mut MenuContent,
    systems: &mut SystemHolder,
) {
    if !menu_content.did_checkbox_click {
        return;
    }
    menu_content.did_checkbox_click = false;

    menu_content.checkbox.iter_mut().for_each(|checkbox| {
        checkbox.set_click(systems, false);
    });
}

pub fn click_textbox(
    menu_content: &mut MenuContent,
    systems: &mut SystemHolder,
    screen_pos: Vec2,
) {
    let mut textbox_found = None;
    for (index, textbox) in menu_content.textbox.iter_mut().enumerate() {
        if is_within_area(
            screen_pos,
            Vec2::new(textbox.pos.x, textbox.pos.y),
            textbox.size,
        ) {
            textbox_found = Some(index)
        }
    }
    if let Some(index) = menu_content.selected_textbox {
        menu_content.textbox[index].set_select(systems, false);
    }
    if let Some(index) = textbox_found {
        menu_content.textbox[index].set_select(systems, true);
        menu_content.textbox[index].set_hold(true);
        menu_content.textbox[index].select_text(systems, screen_pos);
    }
    menu_content.selected_textbox = textbox_found;
}

pub fn release_textbox(menu_content: &mut MenuContent) {
    if let Some(index) = menu_content.selected_textbox {
        menu_content.textbox[index].set_hold(false);
    }
}

pub fn hold_move_textbox(
    menu_content: &mut MenuContent,
    systems: &mut SystemHolder,
    screen_pos: Vec2,
) {
    if let Some(index) = menu_content.selected_textbox {
        menu_content.textbox[index].hold_move(systems, screen_pos);
    }
}

pub fn hover_textbox(
    menu_content: &mut MenuContent,
    systems: &mut SystemHolder,
    tooltip: &mut Tooltip,
    screen_pos: Vec2,
) {
    for textbox in menu_content.textbox.iter_mut() {
        if is_within_area(
            screen_pos,
            Vec2::new(textbox.pos.x, textbox.pos.y),
            Vec2::new(textbox.size.x, textbox.size.y),
        ) {
            if let Some(msg) = &textbox.tooltip {
                tooltip.init_tooltip(systems, screen_pos, msg.clone());
            }
        }
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
            create_login(systems, content);
        }
        WindowType::Register => {
            create_register(systems, content);
        }
        _ => {}
    }
}
