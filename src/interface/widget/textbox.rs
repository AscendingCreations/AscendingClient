use graphics::*;
use cosmic_text::{Attrs, Metrics};
use arboard::Clipboard;

const KEY_CTRL: usize = 0;
const MAX_KEY: usize = 1;

use winit::{
    event::*,
    keyboard::*,
};

use crate::{
    interface::*,
    DrawSetting,
};

pub struct Textbox {
    pub text: String,
    text_index: usize,
    bg: usize,
    limit: usize,
    pub size: Vec2,
    pub pos: Vec3,
    adjust_x: f32,
    is_selected: bool,

    special_key_hold: [bool; MAX_KEY],
    hide_content: bool,
}

impl Textbox {
    pub fn new(
        systems: &mut DrawSetting,
        pos: Vec3,
        size: Vec2,
        text_color: Color,
        render_layer: usize,
        limit: usize,
        selected_bg_color: Color,
        hide_content: bool,
    ) -> Self {
        let mut rect = Rect::new(&mut systems.renderer, 0);
        rect.set_color(selected_bg_color)
            .set_position(pos)
            .set_size(size);
        let bg = systems.gfx.add_rect(rect, 0);
        systems.gfx.set_visible(bg, false);

        let text_data = create_label(systems, 
            pos, 
            size, 
            Bounds::new(pos.x, pos.y, pos.x + size.x, pos.y + size.y),
            text_color);
        let text_index = systems.gfx.add_text(text_data, render_layer);

        Textbox {
            text: String::new(),
            text_index,
            bg,
            limit,
            size,
            pos,
            adjust_x: 0.0,
            is_selected: false,
            special_key_hold: [false; MAX_KEY],
            hide_content,
        }
    }

    pub fn set_select(&mut self, systems: &mut DrawSetting, is_select: bool) {
        if self.is_selected == is_select {
            return;
        }
        self.is_selected = is_select;
        systems.gfx.set_visible(self.bg, self.is_selected);
    }

    pub fn unload(&mut self, systems: &mut DrawSetting) {
        systems.gfx.remove_gfx(self.bg);
        systems.gfx.remove_gfx(self.text_index);
    }

    pub fn enter_text(
        &mut self,
        systems: &mut DrawSetting,
        event: &KeyEvent,
    ) {
        match event.physical_key {
            PhysicalKey::Code(KeyCode::ControlLeft) | PhysicalKey::Code(KeyCode::ControlRight) => {
                self.special_key_hold[KEY_CTRL] = event.state.is_pressed();
            }
            _ => {}
        }

        if !event.state.is_pressed() || !self.is_selected {
            return;
        }

        let mut did_edit = false;
        if self.special_key_hold[KEY_CTRL] {
            match event.physical_key {
                PhysicalKey::Code(KeyCode::KeyC) => {
                    set_clipboard_text(self.text.clone());
                }
                PhysicalKey::Code(KeyCode::KeyV) => {
                    self.text.push_str(&get_clipboard_text());
                    did_edit = true;
                }
                _ => {}
            }
        } else {
            match event.physical_key {
                PhysicalKey::Code(KeyCode::Backspace) => { self.text.pop(); did_edit = true; }
                PhysicalKey::Code(KeyCode::Delete) => { self.text.clear(); did_edit = true; }
                _ => {
                    if self.text.len() >= self.limit {
                        return;
                    }
                    if is_text(event) {
                        if let Some(char) = event.logical_key.to_text() {
                            self.text.push_str(char);
                        }
                        did_edit = true;
                    }
                }
            };
        }
        
        if did_edit {
            let msg = if self.hide_content {
                self.text.chars().map(|_| '*').collect()
            } else {
                self.text.clone()
            };
            systems.gfx.set_text(&mut systems.renderer, self.text_index, &msg);
            self.adjust_text(systems);
        }
    }

    pub fn adjust_text(&mut self, systems: &mut DrawSetting) {
        let adjust_x = (systems.gfx.get_measure(self.text_index).x - self.size.x).max(0.0);
        if self.adjust_x == adjust_x {
            return;
        }
        self.adjust_x = adjust_x;
        systems.gfx.set_pos(self.text_index, 
            Vec3::new(self.pos.x - self.adjust_x, self.pos.y, self.pos.z));
    }
}

pub fn is_text(event: &KeyEvent) -> bool {
    match event.physical_key {
        PhysicalKey::Code(
            KeyCode::KeyA | KeyCode::KeyB | KeyCode::KeyC | KeyCode::KeyD
            | KeyCode::KeyE | KeyCode::KeyF | KeyCode::KeyG | KeyCode::KeyH
            | KeyCode::KeyI | KeyCode::KeyJ | KeyCode::KeyK | KeyCode::KeyL
            | KeyCode::KeyM | KeyCode::KeyN | KeyCode::KeyO | KeyCode::KeyP
            | KeyCode::KeyQ | KeyCode::KeyR | KeyCode::KeyS | KeyCode::KeyT
            | KeyCode::KeyU | KeyCode::KeyV | KeyCode::KeyW | KeyCode::KeyX
            | KeyCode::KeyY | KeyCode::KeyZ | KeyCode::Digit1 | KeyCode::Digit2
            | KeyCode::Digit3 | KeyCode::Digit4 | KeyCode::Digit5 | KeyCode::Digit6
            | KeyCode::Digit7 | KeyCode::Digit8 | KeyCode::Digit9 | KeyCode::Digit0
            | KeyCode::Comma | KeyCode::Period | KeyCode::BracketLeft | KeyCode::BracketRight
            | KeyCode::Backquote | KeyCode::Minus | KeyCode::Equal | KeyCode::Quote
            | KeyCode::Backslash | KeyCode::Semicolon | KeyCode::Slash | KeyCode::Space
        ) => true,
        _ => false,
    }
}

pub fn get_clipboard_text() -> String {
    let mut clipboard = Clipboard::new().unwrap();
    match clipboard.get_text() {
        Ok(data) => data,
        Err(_) => String::new(),
    }
}

pub fn set_clipboard_text(message: String) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(message).unwrap();
}