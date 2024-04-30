use arboard::Clipboard;
use cosmic_text::{Attrs, Metrics};
use graphics::{cosmic_text::rustybuzz::ttf_parser::name::Name, *};
use log::warn;
use std::cmp;

const KEY_CTRL: usize = 0;
const MAX_KEY: usize = 1;

use input::Key;
use winit::keyboard::NamedKey;

use crate::{logic::*, widget::*, GfxType, SystemHolder};

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
    text_index: GfxType,
    selection: GfxType,
    bg: GfxType,
    limit: usize,
    pub size: Vec2,
    pub base_pos: Vec3,
    pub adjust_pos: Vec2,
    adjust_x: f32,
    is_selected: bool,
    caret: GfxType,
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
        base_pos: Vec3,
        adjust_pos: Vec2,
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
        let detail_1 = base_pos.z.sub_f32(z_step.0, z_step.1);
        let detail_2 = detail_1.sub_f32(z_step.0, z_step.1);

        let b_pos = Vec2::new(base_pos.x, base_pos.y)
            + (adjust_pos * systems.scale as f32).floor();

        let mut rect = Rect::new(&mut systems.renderer, 0);
        rect.set_color(bg_color)
            .set_position(Vec3::new(b_pos.x, b_pos.y, base_pos.z))
            .set_size((size * systems.scale as f32).floor());
        let bg = systems.gfx.add_rect(rect, 0, "Textbox BG".into(), false);

        let mut select_rect = Rect::new(&mut systems.renderer, 0);
        select_rect
            .set_color(selection_bg_color)
            .set_position(Vec3::new(b_pos.x, b_pos.y, detail_1))
            .set_size(Vec2::new(0.0, size.y * systems.scale as f32).floor());
        let selection = systems.gfx.add_rect(
            select_rect,
            0,
            "Textbox Select".into(),
            visible,
        );

        let mut text_data = create_label(
            systems,
            Vec3::new(b_pos.x, b_pos.y, detail_2),
            size * systems.scale as f32,
            Bounds::new(
                b_pos.x,
                b_pos.y,
                b_pos.x + (size.x * systems.scale as f32),
                b_pos.y + (size.y * systems.scale as f32),
            ),
            text_color,
        );
        text_data.set_wrap(&mut systems.renderer, cosmic_text::Wrap::None);
        let text_index = systems.gfx.add_text(
            text_data,
            render_layer,
            "Textbox Text".into(),
            visible,
        );

        let mut caret_rect = Rect::new(&mut systems.renderer, 0);
        caret_rect
            .set_size((Vec2::new(2.0, size.y) * systems.scale as f32).floor())
            .set_position(Vec3::new(b_pos.x, b_pos.y, detail_2))
            .set_color(text_color);
        let caret =
            systems
                .gfx
                .add_rect(caret_rect, 0, "Textbox Caret".into(), false);

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
            base_pos,
            adjust_pos,
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
        systems.gfx.set_visible(&self.bg, self.is_selected);
        if self.is_selected {
            systems.caret.index = Some(self.caret);
        } else {
            systems.gfx.set_visible(&self.caret, false);
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
        systems.gfx.remove_gfx(&mut systems.renderer, &self.bg);
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, &self.text_index);
        systems.gfx.remove_gfx(&mut systems.renderer, &self.caret);
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, &self.selection);
        if let Some(index) = systems.caret.index {
            if index == self.caret {
                systems.caret.index = None;
            }
        }
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        if self.visible == visible {
            return;
        }
        self.visible = visible;
        systems
            .gfx
            .set_visible(&self.bg, self.is_selected && visible);
        systems.gfx.set_visible(&self.text_index, visible);
        systems.gfx.set_visible(&self.caret, false);
        systems.gfx.set_visible(&self.selection, visible);
    }

    pub fn set_z_order(&mut self, systems: &mut SystemHolder, z_order: f32) {
        self.base_pos.z = z_order;

        let detail_1 = self.base_pos.z.sub_f32(self.z_step.0, self.z_step.1);
        let detail_2 = detail_1.sub_f32(self.z_step.0, self.z_step.1);

        let b_pos = Vec2::new(self.base_pos.x, self.base_pos.y)
            + (self.adjust_pos * systems.scale as f32).floor();

        systems.gfx.set_pos(&self.bg, self.base_pos);
        systems.gfx.set_pos(
            &self.text_index,
            Vec3::new(b_pos.x + self.adjust_x, b_pos.y, detail_2),
        );
        systems.gfx.set_pos(
            &self.caret,
            Vec3::new(b_pos.x + self.caret_left, b_pos.y, detail_2),
        );
        systems.gfx.set_pos(
            &self.selection,
            Vec3::new(b_pos.x + self.selection_pos, b_pos.y, detail_1),
        );
    }

    pub fn set_pos(&mut self, systems: &mut SystemHolder, new_pos: Vec2) {
        let detail_1 = self.base_pos.z.sub_f32(self.z_step.0, self.z_step.1);
        let detail_2 = detail_1.sub_f32(self.z_step.0, self.z_step.1);

        self.base_pos.x = new_pos.x;
        self.base_pos.y = new_pos.y;

        let b_pos = Vec2::new(self.base_pos.x, self.base_pos.y)
            + (self.adjust_pos * systems.scale as f32).floor();

        systems
            .gfx
            .set_pos(&self.bg, Vec3::new(b_pos.x, b_pos.y, self.base_pos.z));
        systems.gfx.set_pos(
            &self.text_index,
            Vec3::new(b_pos.x + self.adjust_x, b_pos.y, detail_2),
        );
        systems.gfx.set_bound(
            &self.text_index,
            Bounds::new(
                b_pos.x + self.adjust_x,
                b_pos.y,
                b_pos.x + (self.size.x * systems.scale as f32).floor(),
                b_pos.y + (self.size.y * systems.scale as f32).floor(),
            ),
        );
        systems.gfx.set_pos(
            &self.caret,
            Vec3::new(b_pos.x + self.caret_left, b_pos.y, detail_2),
        );
        systems.gfx.set_pos(
            &self.selection,
            Vec3::new(b_pos.x + self.selection_pos, b_pos.y, detail_1),
        );
    }

    pub fn set_text(&mut self, systems: &mut SystemHolder, text: String) {
        self.text.clear();
        self.data_text.clear();
        self.char_size.clear();
        self.adjust_x = 0.0;
        self.caret_left = 0.0;
        self.caret_pos = 0;

        let b_pos = Vec2::new(self.base_pos.x, self.base_pos.y)
            + (self.adjust_pos * systems.scale as f32).floor();

        if text.is_empty() {
            systems
                .gfx
                .set_text(&mut systems.renderer, &self.text_index, "");
            let pos = systems.gfx.get_pos(&self.caret);
            systems
                .gfx
                .set_pos(&self.caret, Vec3::new(b_pos.x, b_pos.y, pos.z));
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
            .set_text(&mut systems.renderer, &self.text_index, &msg);

        self.move_caret_pos(
            systems,
            false,
            self.data_text.chars().count(),
            false,
        );
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
                        if self.data_text.chars().count()
                            + clipboard_text.chars().count()
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

                            self.text = insert_text(
                                self.text.clone(),
                                self.caret_pos,
                                &clipboard_text,
                            );
                            let clipboard = if self.hide_content {
                                clipboard_text.chars().map(|_| '*').collect()
                            } else {
                                clipboard_text.clone()
                            };
                            self.data_text = insert_text(
                                self.data_text.clone(),
                                self.caret_pos,
                                &clipboard,
                            );

                            for char in clipboard.chars().rev() {
                                let size =
                                    measure_string(systems, char.to_string()).x;
                                self.char_size.insert(self.caret_pos, size);
                            }
                            self.move_caret_pos(
                                systems,
                                false,
                                clipboard_text.chars().count(),
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
                    } else if self.caret_pos == self.data_text.chars().count() {
                        self.move_caret_pos(systems, true, 1, true);
                        self.text.pop();
                        self.data_text.pop();
                        self.char_size.pop();
                    } else if self.caret_pos > 0 {
                        self.move_caret_pos(systems, true, 1, true);
                        self.text =
                            remove_text(self.text.clone(), self.caret_pos);
                        self.data_text =
                            remove_text(self.data_text.clone(), self.caret_pos);
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
                    if self.data_text.chars().count() >= self.limit {
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

                            self.text = insert_char(
                                self.text.clone(),
                                self.caret_pos,
                                char,
                            );
                            self.data_text = insert_char(
                                self.data_text.clone(),
                                self.caret_pos,
                                msg,
                            );
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
                &self.text_index,
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
            self.text = remove_text(self.text.clone(), first);
            self.data_text = remove_text(self.data_text.clone(), first);
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
                .min(self.data_text.chars().count());
            (start, self.caret_pos)
        };
        let edit_text: String = self
            .data_text
            .chars()
            .skip(start)
            .take(end - start)
            .collect();
        let size = measure_string(systems, edit_text).x;

        if move_left {
            self.caret_left -= size;
        } else {
            self.caret_left += size;
        }

        if self.caret_left < 0.0 {
            self.adjust_x += (self.caret_left * -1.0).max(0.0);
            self.caret_left = 0.0;
        } else if self.caret_left > (self.size.x * systems.scale as f32).floor()
        {
            self.adjust_x -= (self.caret_left
                - (self.size.x * systems.scale as f32).floor())
            .max(0.0);
            self.caret_left = (self.size.x * systems.scale as f32).floor();
        }

        if remove_content {
            let mut text = self.data_text.clone();
            if self.hold_initial_index != self.hold_final_index {
                let first =
                    cmp::min(self.hold_initial_index, self.hold_final_index);
                let second =
                    cmp::max(self.hold_initial_index, self.hold_final_index);
                text = replace_text(text.clone(), first, second, String::new());
            } else if self.caret_pos < text.chars().count() {
                text = replace_text(
                    text.clone(),
                    self.caret_pos,
                    self.caret_pos + 1,
                    String::new(),
                );
            };
            let total_size = measure_string(systems, text).x;
            if total_size > (self.size.x * systems.scale as f32).floor() {
                let visible_size = total_size + self.adjust_x;
                if visible_size > 0.0 {
                    let leftover = (self.size.x * systems.scale as f32).floor()
                        - visible_size;
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

        let b_pos = Vec2::new(self.base_pos.x, self.base_pos.y)
            + (self.adjust_pos * systems.scale as f32).floor();

        systems.gfx.set_pos(
            &self.text_index,
            Vec3::new(
                b_pos.x + self.adjust_x,
                b_pos.y,
                self.base_pos.z.sub_f32(self.z_step.0, self.z_step.1),
            ),
        );
        systems.gfx.set_pos(
            &self.caret,
            Vec3::new(
                b_pos.x + self.caret_left,
                b_pos.y,
                self.base_pos.z.sub_f32(self.z_step.0, self.z_step.1),
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
            text = replace_text(text.clone(), first, second + 1, String::new());
        } else if self.caret_pos < text.chars().count() {
            text = replace_text(
                text.clone(),
                self.caret_pos,
                self.caret_pos + 1,
                String::new(),
            );
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

        let b_pos = Vec2::new(self.base_pos.x, self.base_pos.y)
            + (self.adjust_pos * systems.scale as f32).floor();

        systems.gfx.set_pos(
            &self.text_index,
            Vec3::new(
                b_pos.x + self.adjust_x,
                b_pos.y,
                self.base_pos.z.sub_f32(self.z_step.0, self.z_step.1),
            ),
        );
        systems.gfx.set_pos(
            &self.caret,
            Vec3::new(
                b_pos.x + self.caret_left,
                b_pos.y,
                self.base_pos.z.sub_f32(self.z_step.0, self.z_step.1),
            ),
        );
    }

    pub fn select_text(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
    ) {
        let b_pos = Vec2::new(self.base_pos.x, self.base_pos.y)
            + (self.adjust_pos * systems.scale as f32).floor();

        if !is_within_area(
            screen_pos,
            Vec2::new(b_pos.x, b_pos.y),
            (self.size * systems.scale as f32).floor(),
        ) {
            return;
        }

        let mut found_index = None;
        for (index, pos) in self.char_pos.iter().enumerate() {
            if is_within_area(
                screen_pos,
                Vec2::new(b_pos.x + pos + self.adjust_x, b_pos.y),
                Vec2::new(self.char_size[index], self.size.y),
            ) {
                found_index = Some(index);
                break;
            }
        }
        if found_index.is_none() {
            found_index = Some(self.data_text.chars().count());
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

        let b_pos = Vec2::new(self.base_pos.x, self.base_pos.y)
            + (self.adjust_pos * systems.scale as f32).floor();

        let mut found_index = None;
        for (index, pos) in self.char_pos.iter().enumerate() {
            if is_within_area(
                screen_pos,
                Vec2::new(b_pos.x + pos + self.adjust_x, 0.0),
                Vec2::new(self.char_size[index], systems.size.height),
            ) {
                found_index = Some(index);
                break;
            }
        }
        if found_index.is_none() {
            if screen_pos.x > b_pos.x + self.size.x {
                found_index = Some(self.data_text.chars().count());
            } else if screen_pos.x < b_pos.x {
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

            let text: String = self
                .data_text
                .chars()
                .skip(first)
                .take(second - first)
                .collect();

            self.selection_pos =
                (self.adjust_x + self.char_pos[first]).floor().max(0.0);
            measure_string(systems, text)
                .x
                .floor()
                .min((self.size.x * systems.scale as f32) - self.selection_pos)
        } else {
            0.0
        };

        let b_pos = Vec2::new(self.base_pos.x, self.base_pos.y)
            + (self.adjust_pos * systems.scale as f32).floor();

        let pos = systems.gfx.get_pos(&self.selection);
        systems.gfx.set_pos(
            &self.selection,
            Vec3::new(b_pos.x + self.selection_pos, pos.y, pos.z),
        );
        systems.gfx.set_size(
            &self.selection,
            Vec2::new(size, (self.size.y * systems.scale as f32).floor()),
        );
    }
}

pub fn insert_text(text: String, pos: usize, insert_text: &str) -> String {
    let mut first_text = String::new();
    let mut second_text = String::new();
    for (cur_pos, char) in text.chars().enumerate() {
        if cur_pos < pos {
            first_text.push(char);
        } else {
            second_text.push(char);
        }
    }
    format!("{}{}{}", first_text, insert_text, second_text)
}

pub fn insert_char(text: String, pos: usize, insert_text: char) -> String {
    let mut first_text = String::new();
    let mut second_text = String::new();
    for (cur_pos, char) in text.chars().enumerate() {
        if cur_pos < pos {
            first_text.push(char);
        } else {
            second_text.push(char);
        }
    }
    format!("{}{}{}", first_text, insert_text, second_text)
}

pub fn replace_text(
    text: String,
    from_pos: usize,
    to_pos: usize,
    replace_to: String,
) -> String {
    let mut first_text = String::new();
    let mut second_text = String::new();
    for (cur_pos, char) in text.chars().enumerate() {
        if cur_pos < from_pos {
            first_text.push(char);
        } else if cur_pos >= to_pos {
            second_text.push(char);
        }
    }
    format!("{}{}{}", first_text, replace_to, second_text)
}

pub fn remove_text(text: String, pos: usize) -> String {
    let mut first_text = String::new();
    let mut second_text = String::new();
    for (cur_pos, char) in text.chars().enumerate() {
        match cur_pos.cmp(&pos) {
            std::cmp::Ordering::Less => first_text.push(char),
            std::cmp::Ordering::Greater => second_text.push(char),
            std::cmp::Ordering::Equal => {}
        }
    }
    format!("{}{}", first_text, second_text)
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
