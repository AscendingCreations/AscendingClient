use arboard::Clipboard;
use cosmic_text::{Attrs, Metrics};
use graphics::{cosmic_text::rustybuzz::ttf_parser::name::Name, *};
use log::warn;
use std::cmp;

const KEY_CTRL: usize = 0;
const MAX_KEY: usize = 1;

use input::Key;
use winit::keyboard::NamedKey;

use crate::{logic::*, widget::*, SystemHolder};

pub enum TextDisable {
    Selection,
    Copy,
    Paste,
}

pub struct Textbox {
    visible: bool,
    pub text: String,
    data_text: String,
    char_size: Vec<f32>,
    char_pos: Vec<f32>,
    text_index: usize,
    selection: usize,
    bg: usize,
    limit: usize,
    pub size: Vec2,
    pub pos: Vec3,
    adjust_x: f32,
    is_selected: bool,
    caret: usize,
    caret_left: f32,
    caret_pos: usize,

    special_key_hold: [bool; MAX_KEY],
    hide_content: bool,
    z_step: (f32, i32),
    pub tooltip: Option<String>,
    in_hold: bool,
    hold_initial_index: usize,
    hold_final_index: usize,
    selection_pos: f32,

    disable_selection: bool,
    disable_copy: bool,
    disable_paste: bool,
}

impl Textbox {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        systems: &mut SystemHolder,
        pos: Vec3,
        z_step: (f32, i32),
        size: Vec2,
        text_color: Color,
        render_layer: usize,
        limit: usize,
        bg_color: Color,
        selection_bg_color: Color,
        hide_content: bool,
        visible: bool,
        tooltip: Option<String>,
        disable_option: Vec<TextDisable>,
    ) -> Self {
        let detail_1 = pos.z.sub_f32(z_step.0, z_step.1);
        let detail_2 = detail_1.sub_f32(z_step.0, z_step.1);

        let mut rect = Rect::new(&mut systems.renderer, 0);
        rect.set_color(bg_color).set_position(pos).set_size(size);
        let bg = systems.gfx.add_rect(rect, 0, "Textbox BG".into());
        systems.gfx.set_visible(bg, false);

        let mut select_rect = Rect::new(&mut systems.renderer, 0);
        select_rect
            .set_color(selection_bg_color)
            .set_position(Vec3::new(pos.x, pos.y, detail_1))
            .set_size(Vec2::new(0.0, size.y));
        let selection =
            systems
                .gfx
                .add_rect(select_rect, 0, "Textbox Select".into());
        systems.gfx.set_visible(selection, visible);

        let mut text_data = create_label(
            systems,
            Vec3::new(pos.x, pos.y, detail_2),
            size,
            Bounds::new(pos.x, pos.y, pos.x + size.x, pos.y + size.y),
            text_color,
        );
        text_data.set_wrap(&mut systems.renderer, cosmic_text::Wrap::None);
        let text_index = systems.gfx.add_text(
            text_data,
            render_layer,
            "Textbox Text".into(),
        );
        systems.gfx.set_visible(text_index, visible);

        let mut caret_rect = Rect::new(&mut systems.renderer, 0);
        caret_rect
            .set_size(Vec2::new(2.0, size.y))
            .set_position(Vec3::new(pos.x, pos.y, detail_2))
            .set_color(text_color);
        let caret = systems.gfx.add_rect(caret_rect, 0, "Textbox Caret".into());
        systems.gfx.set_visible(caret, false);

        let mut disable_selection = false;
        let mut disable_copy = false;
        let mut disable_paste = false;
        for data in disable_option.iter() {
            match data {
                TextDisable::Selection => disable_selection = true,
                TextDisable::Copy => disable_copy = true,
                TextDisable::Paste => disable_paste = true,
            }
        }

        Textbox {
            visible,
            text: String::new(),
            data_text: String::new(),
            char_size: Vec::new(),
            char_pos: Vec::new(),
            text_index,
            bg,
            selection,
            limit,
            size,
            pos,
            z_step,
            adjust_x: 0.0,
            is_selected: false,
            special_key_hold: [false; MAX_KEY],
            hide_content,
            tooltip,
            caret,
            caret_left: 0.0,
            caret_pos: 0,
            in_hold: false,
            hold_initial_index: 0,
            hold_final_index: 0,
            selection_pos: 0.0,
            disable_selection,
            disable_copy,
            disable_paste,
        }
    }

    pub fn set_select(&mut self, systems: &mut SystemHolder, is_select: bool) {
        if self.is_selected == is_select || !self.visible {
            return;
        }
        self.is_selected = is_select;
        systems.gfx.set_visible(self.bg, self.is_selected);
        if self.is_selected {
            systems.caret.index = Some(self.caret);
        } else {
            systems.gfx.set_visible(self.caret, false);
            if let Some(index) = systems.caret.index {
                if index == self.caret {
                    systems.caret.index = None;
                }
            }
            self.hold_initial_index = self.caret_pos;
            self.hold_final_index = self.caret_pos;
            self.update_selection(systems);
        }
    }

    pub fn unload(&mut self, systems: &mut SystemHolder) {
        systems.gfx.remove_gfx(&mut systems.renderer, self.bg);
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, self.text_index);
        systems.gfx.remove_gfx(&mut systems.renderer, self.caret);
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, self.selection);
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        if self.visible == visible {
            return;
        }
        self.visible = visible;
        systems
            .gfx
            .set_visible(self.bg, self.is_selected && visible);
        systems.gfx.set_visible(self.text_index, visible);
        systems.gfx.set_visible(self.caret, false);
        systems.gfx.set_visible(self.selection, visible);
    }

    pub fn set_z_order(&mut self, systems: &mut SystemHolder, z_order: f32) {
        self.pos.z = z_order;

        let detail_1 = self.pos.z.sub_f32(self.z_step.0, self.z_step.1);
        let detail_2 = detail_1.sub_f32(self.z_step.0, self.z_step.1);

        systems.gfx.set_pos(self.bg, self.pos);
        systems.gfx.set_pos(
            self.text_index,
            Vec3::new(self.pos.x + self.adjust_x, self.pos.y, detail_2),
        );
        systems.gfx.set_pos(
            self.caret,
            Vec3::new(self.pos.x + self.caret_left, self.pos.y, detail_2),
        );
        systems.gfx.set_pos(
            self.selection,
            Vec3::new(self.pos.x + self.selection_pos, self.pos.y, detail_1),
        );
    }

    pub fn set_pos(&mut self, systems: &mut SystemHolder, new_pos: Vec2) {
        let detail_1 = self.pos.z.sub_f32(self.z_step.0, self.z_step.1);
        let detail_2 = detail_1.sub_f32(self.z_step.0, self.z_step.1);

        self.pos.x = new_pos.x;
        self.pos.y = new_pos.y;
        systems.gfx.set_pos(self.bg, self.pos);
        systems.gfx.set_pos(
            self.text_index,
            Vec3::new(self.pos.x + self.adjust_x, self.pos.y, detail_2),
        );
        systems.gfx.set_bound(
            self.text_index,
            Bounds::new(
                self.pos.x + self.adjust_x,
                self.pos.y,
                self.pos.x + self.size.x,
                self.pos.y + self.size.y,
            ),
        );
        systems.gfx.set_pos(
            self.caret,
            Vec3::new(self.pos.x + self.caret_left, self.pos.y, detail_2),
        );
        systems.gfx.set_pos(
            self.selection,
            Vec3::new(self.pos.x + self.selection_pos, self.pos.y, detail_1),
        );
    }

    pub fn set_text(&mut self, systems: &mut SystemHolder, text: String) {
        self.text.clear();
        self.data_text.clear();
        self.char_size.clear();
        self.caret_left = 0.0;
        self.caret_pos = 0;

        if text.is_empty() {
            systems
                .gfx
                .set_text(&mut systems.renderer, self.text_index, "");
            let pos = systems.gfx.get_pos(self.caret);
            systems
                .gfx
                .set_pos(self.caret, Vec3::new(self.pos.x, self.pos.y, pos.z));
            return;
        }

        self.text.push_str(&text);
        let msg = if self.hide_content {
            self.text.chars().map(|_| '*').collect()
        } else {
            self.text.clone()
        };
        self.data_text.push_str(&msg);

        for char in msg.chars().rev() {
            let size = measure_string(systems, char.to_string()).x;
            self.char_size.insert(self.caret_pos, size);
        }

        systems
            .gfx
            .set_text(&mut systems.renderer, self.text_index, &msg);

        self.move_caret_pos(systems, false, self.data_text.len(), false);
    }

    pub fn enter_text(
        &mut self,
        systems: &mut SystemHolder,
        key: &Key,
        pressed: bool,
        numeric_only: bool,
    ) {
        if !self.visible {
            return;
        }

        if let Key::Named(NamedKey::Control) = key {
            self.special_key_hold[KEY_CTRL] = pressed;
        }

        if !pressed || !self.is_selected {
            return;
        }

        let mut did_edit = false;
        if self.special_key_hold[KEY_CTRL] {
            if !numeric_only {
                match key {
                    Key::Character('c') => {
                        if self.disable_copy {
                            return;
                        }

                        if self.hold_initial_index == self.hold_final_index {
                            return;
                        }
                        let first = cmp::min(
                            self.hold_initial_index,
                            self.hold_final_index,
                        );
                        let second = cmp::max(
                            self.hold_initial_index,
                            self.hold_final_index,
                        );
                        let text = &self.text[first..second];

                        set_clipboard_text(text.to_string());
                    }
                    Key::Character('v') => {
                        if self.disable_paste {
                            return;
                        }

                        let clipboard_text = get_clipboard_text();
                        if self.data_text.len() + clipboard_text.len()
                            >= self.limit
                        {
                            return;
                        }

                        let can_proceed = if numeric_only {
                            is_numeric(&clipboard_text)
                        } else {
                            true
                        };

                        if can_proceed {
                            self.remove_selection(systems);

                            self.text
                                .insert_str(self.caret_pos, &clipboard_text);
                            let clipboard = if self.hide_content {
                                clipboard_text.chars().map(|_| '*').collect()
                            } else {
                                clipboard_text.clone()
                            };
                            self.data_text
                                .insert_str(self.caret_pos, &clipboard);

                            for char in clipboard.chars().rev() {
                                let size =
                                    measure_string(systems, char.to_string()).x;
                                self.char_size.insert(self.caret_pos, size);
                            }
                            self.move_caret_pos(
                                systems,
                                false,
                                clipboard_text.len(),
                                false,
                            );

                            did_edit = true;
                        }
                    }
                    _ => {}
                }
            }
        } else {
            match key {
                Key::Named(NamedKey::Backspace) => {
                    if self.hold_initial_index != self.hold_final_index {
                        self.remove_selection(systems);
                    } else if self.caret_pos == self.data_text.len() {
                        self.move_caret_pos(systems, true, 1, true);
                        self.text.pop();
                        self.data_text.pop();
                        self.char_size.pop();
                    } else if self.caret_pos > 0 {
                        self.move_caret_pos(systems, true, 1, true);
                        self.text.remove(self.caret_pos);
                        self.data_text.remove(self.caret_pos);
                        self.char_size.remove(self.caret_pos);
                    }
                    did_edit = true;
                }
                Key::Named(NamedKey::ArrowLeft) => {
                    self.move_caret_pos(systems, true, 1, false);
                    self.hold_final_index = self.caret_pos;
                    self.hold_initial_index = self.caret_pos;
                    self.update_selection(systems);
                    return;
                }
                Key::Named(NamedKey::ArrowRight) => {
                    self.move_caret_pos(systems, false, 1, false);
                    self.hold_final_index = self.caret_pos;
                    self.hold_initial_index = self.caret_pos;
                    self.update_selection(systems);
                    return;
                }
                _ => {
                    if self.data_text.len() >= self.limit {
                        return;
                    }
                    let key_char = if let Key::Character(char) = key {
                        Some(*char)
                    } else if Key::Named(NamedKey::Space) == *key {
                        Some(' ')
                    } else {
                        None
                    };

                    if let Some(char) = key_char {
                        let can_proceed = if numeric_only {
                            is_numeric(&char.to_string())
                        } else {
                            true
                        };
                        if can_proceed {
                            self.remove_selection(systems);

                            let msg =
                                if self.hide_content { '*' } else { char };

                            self.text.insert(self.caret_pos, char);
                            self.data_text.insert(self.caret_pos, msg);
                            let size =
                                measure_string(systems, msg.to_string()).x;
                            self.char_size.insert(self.caret_pos, size);
                            self.move_caret_pos(systems, false, 1, false);
                            did_edit = true;
                        }
                    }
                }
            };
        }

        if did_edit {
            systems.gfx.set_text(
                &mut systems.renderer,
                self.text_index,
                &self.data_text,
            );

            self.hold_final_index = self.caret_pos;
            self.hold_initial_index = self.caret_pos;
            self.update_selection(systems);
        }
    }

    pub fn remove_selection(&mut self, systems: &mut SystemHolder) {
        if self.hold_initial_index == self.hold_final_index {
            return;
        }
        let first = cmp::min(self.hold_initial_index, self.hold_final_index);
        let second = cmp::max(self.hold_initial_index, self.hold_final_index);
        let count = second - first;
        if self.hold_initial_index < self.hold_final_index {
            self.move_caret_pos(systems, true, count, true);
        } else {
            self.update_caret_pos(systems);
        }
        for _ in first..first + count {
            self.text.remove(first);
            self.data_text.remove(first);
            self.char_size.remove(first);
        }
    }

    pub fn move_caret_pos(
        &mut self,
        systems: &mut SystemHolder,
        move_left: bool,
        count: usize,
        remove_content: bool,
    ) {
        let (start, end) = if move_left {
            let end = self.caret_pos;
            self.caret_pos = self.caret_pos.saturating_sub(count);
            (self.caret_pos, end)
        } else {
            let start = self.caret_pos;
            self.caret_pos = self
                .caret_pos
                .saturating_add(count)
                .min(self.data_text.len());
            (start, self.caret_pos)
        };
        let size = measure_string(systems, self.text[start..end].to_string()).x;

        if move_left {
            self.caret_left -= size;
        } else {
            self.caret_left += size;
        }

        if self.caret_left < 0.0 {
            self.adjust_x += (self.caret_left * -1.0).max(0.0);
            self.caret_left = 0.0;
        } else if self.caret_left > self.size.x {
            self.adjust_x -= (self.caret_left - self.size.x).max(0.0);
            self.caret_left = self.size.x;
        }

        if remove_content {
            let mut text = self.data_text.clone();
            if self.hold_initial_index != self.hold_final_index {
                let first =
                    cmp::min(self.hold_initial_index, self.hold_final_index);
                let second =
                    cmp::max(self.hold_initial_index, self.hold_final_index);
                text.replace_range(first..second, "");
            } else if self.caret_pos < text.len() {
                text.replace_range(self.caret_pos..self.caret_pos + 1, "");
            };
            let total_size = measure_string(systems, text).x;
            if total_size > self.size.x {
                let visible_size = total_size + self.adjust_x;
                if visible_size > 0.0 {
                    let leftover = self.size.x - visible_size;
                    if leftover > 0.0 {
                        self.caret_left += leftover;
                        self.adjust_x += leftover;
                    }
                }
            } else if self.adjust_x < 0.0 {
                self.caret_left += self.adjust_x * -1.0;
                self.adjust_x = 0.0;
            }
        }

        systems.gfx.set_pos(
            self.text_index,
            Vec3::new(
                self.pos.x + self.adjust_x,
                self.pos.y,
                self.pos.z.sub_f32(self.z_step.0, self.z_step.1),
            ),
        );
        systems.gfx.set_pos(
            self.caret,
            Vec3::new(
                self.pos.x + self.caret_left,
                self.pos.y,
                self.pos.z.sub_f32(self.z_step.0, self.z_step.1),
            ),
        );
    }

    pub fn update_caret_pos(&mut self, systems: &mut SystemHolder) {
        let mut text = self.data_text.clone();
        if self.hold_initial_index != self.hold_final_index {
            let first =
                cmp::min(self.hold_initial_index, self.hold_final_index);
            let second =
                cmp::max(self.hold_initial_index, self.hold_final_index);
            text.replace_range(first..second, "");
        } else if self.caret_pos < text.len() {
            text.replace_range(self.caret_pos..self.caret_pos + 1, "");
        };
        let total_size = measure_string(systems, text).x;
        if total_size > self.size.x {
            let visible_size = total_size + self.adjust_x;
            if visible_size > 0.0 {
                let leftover = self.size.x - visible_size;
                if leftover > 0.0 {
                    self.caret_left += leftover;
                    self.adjust_x += leftover;
                }
            }
        } else if self.adjust_x < 0.0 {
            self.caret_left += self.adjust_x * -1.0;
            self.adjust_x = 0.0;
        }

        systems.gfx.set_pos(
            self.text_index,
            Vec3::new(
                self.pos.x + self.adjust_x,
                self.pos.y,
                self.pos.z.sub_f32(self.z_step.0, self.z_step.1),
            ),
        );
        systems.gfx.set_pos(
            self.caret,
            Vec3::new(
                self.pos.x + self.caret_left,
                self.pos.y,
                self.pos.z.sub_f32(self.z_step.0, self.z_step.1),
            ),
        );
    }

    pub fn select_text(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
    ) {
        if !is_within_area(
            screen_pos,
            Vec2::new(self.pos.x, self.pos.y),
            self.size,
        ) {
            return;
        }

        let mut found_index = None;
        for (index, pos) in self.char_pos.iter().enumerate() {
            if is_within_area(
                screen_pos,
                Vec2::new(self.pos.x + pos + self.adjust_x, self.pos.y),
                Vec2::new(self.char_size[index], self.size.y),
            ) {
                found_index = Some(index);
                break;
            }
        }
        if found_index.is_none() {
            found_index = Some(self.data_text.len());
        }
        if let Some(index) = found_index {
            let offset = index as i32 - self.caret_pos as i32;
            match offset.cmp(&0) {
                std::cmp::Ordering::Less => {
                    let move_count = offset.abs();
                    self.move_caret_pos(
                        systems,
                        true,
                        move_count as usize,
                        false,
                    );
                }
                std::cmp::Ordering::Greater => {
                    self.move_caret_pos(systems, false, offset as usize, false);
                }
                _ => {}
            }
            self.hold_initial_index = self.caret_pos;
            self.hold_final_index = self.caret_pos;
            self.update_selection(systems);
        }
    }

    pub fn set_hold(&mut self, is_hold: bool) {
        if self.in_hold == is_hold {
            return;
        }
        self.in_hold = is_hold;
        if self.in_hold {
            self.char_pos.clear();
            let mut pos_x = 0.0;
            for size in self.char_size.iter() {
                self.char_pos.push(pos_x);
                pos_x += size;
            }
        }
    }

    pub fn hold_move(&mut self, systems: &mut SystemHolder, screen_pos: Vec2) {
        if !self.in_hold || self.disable_selection {
            return;
        }

        let mut found_index = None;
        for (index, pos) in self.char_pos.iter().enumerate() {
            if is_within_area(
                screen_pos,
                Vec2::new(self.pos.x + pos + self.adjust_x, 0.0),
                Vec2::new(self.char_size[index], systems.size.height),
            ) {
                found_index = Some(index);
                break;
            }
        }
        if found_index.is_none() {
            if screen_pos.x > self.pos.x + self.size.x {
                found_index = Some(self.data_text.len());
            } else if screen_pos.x < self.pos.x {
                found_index = Some(0);
            }
        }
        if let Some(index) = found_index {
            let offset = index as i32 - self.caret_pos as i32;
            match offset.cmp(&0) {
                std::cmp::Ordering::Less => {
                    let move_count = offset.abs();
                    self.move_caret_pos(
                        systems,
                        true,
                        move_count as usize,
                        false,
                    );
                }
                std::cmp::Ordering::Greater => {
                    self.move_caret_pos(systems, false, offset as usize, false);
                }
                _ => {}
            }
            self.hold_final_index = self.caret_pos;
            self.update_selection(systems);
        }
    }

    pub fn update_selection(&mut self, systems: &mut SystemHolder) {
        let size = if self.hold_initial_index != self.hold_final_index
            && !self.data_text.is_empty()
        {
            let first =
                cmp::min(self.hold_initial_index, self.hold_final_index);
            let second =
                cmp::max(self.hold_initial_index, self.hold_final_index);
            let text = self.text[first..second].to_string();

            self.selection_pos =
                (self.adjust_x + self.char_pos[first]).floor().max(0.0);
            measure_string(systems, text)
                .x
                .floor()
                .min(self.size.x - self.selection_pos)
        } else {
            0.0
        };

        let pos = systems.gfx.get_pos(self.selection);
        systems.gfx.set_pos(
            self.selection,
            Vec3::new(self.pos.x + self.selection_pos, pos.y, pos.z),
        );
        systems
            .gfx
            .set_size(self.selection, Vec2::new(size, self.size.y));
    }
}

pub fn is_numeric(char: &str) -> bool {
    char.trim().parse::<i64>().is_ok()
}

pub fn get_clipboard_text() -> String {
    let mut clipboard = Clipboard::new().unwrap();
    match clipboard.get_text() {
        Ok(data) => data,
        Err(e) => {
            warn!("Get Clipboard Err: {}", e);
            String::new()
        }
    }
}

pub fn set_clipboard_text(message: String) {
    let mut clipboard = Clipboard::new().unwrap();
    match clipboard.set_text(message) {
        Ok(_) => {}
        Err(e) => {
            warn!("Set Clipboard Err: {}", e);
        }
    }
}
