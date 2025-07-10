use graphics::*;

use crate::{
    Item, SystemHolder, data_types::*, is_within_area, logic::*, widget::*,
};

use super::ItemDescription;

pub enum ProfileLabel {
    Level,
    Money,
    Damage,
    Defense,
}

#[derive(Clone, Copy)]
struct EqData {
    img: GfxType,
    index: usize,
}

pub struct Profile {
    pub visible: bool,
    bg: GfxType,
    header: GfxType,
    header_text: GfxType,
    button: Vec<Button>,
    fixed_label: Vec<GfxType>,
    value_label: Vec<GfxType>,
    slot: [GfxType; MAX_EQPT],
    eq_data: [Option<EqData>; MAX_EQPT],

    pub pos: Vec2,
    pub size: Vec2,
    pub z_order: f32,
    pub order_index: usize,
    in_hold: bool,
    hold_pos: Vec2,
    header_pos: Vec2,
    header_size: Vec2,
    pub did_button_click: bool,

    min_bound: Vec2,
    max_bound: Vec2,
}

impl Profile {
    pub fn new(systems: &mut SystemHolder) -> Self {
        let orig_size = Vec2::new(200.0, 267.0);
        let w_size = (orig_size * systems.scale as f32).floor();
        let w_pos = Vec3::new(
            systems.size.width - w_size.x - 10.0,
            60.0,
            ORDER_GUI_WINDOW,
        );
        let pos = Vec2::new(w_pos.x, w_pos.y);

        let detail_1 = w_pos.z.sub_f32(0.001, 3);
        let detail_2 = w_pos.z.sub_f32(0.002, 3);

        let mut rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(w_pos.x - 1.0, w_pos.y - 1.0, w_pos.z),
            w_size + 2.0,
            Color::rgba(110, 110, 110, 255),
            0,
        );
        rect.set_border_width(1.0)
            .set_border_color(Color::rgba(20, 20, 20, 255));
        let bg = systems.gfx.add_rect(rect, 0, "Profile BG", false);

        let header_pos = Vec2::new(
            w_pos.x,
            w_pos.y + (237.0 * systems.scale as f32).floor(),
        );
        let header_size = Vec2::new(orig_size.x, 30.0);
        let header_zpos = detail_1;
        let header_rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(header_pos.x, header_pos.y, header_zpos),
            Vec2::new(
                (header_size.x * systems.scale as f32).floor(),
                (header_size.y * systems.scale as f32).floor(),
            ),
            Color::rgba(70, 70, 70, 255),
            0,
        );

        let header =
            systems
                .gfx
                .add_rect(header_rect, 0, "Profile Header", false);

        let text = create_label(
            systems,
            Vec3::new(
                w_pos.x,
                w_pos.y + (242.0 * systems.scale as f32).floor(),
                detail_2,
            ),
            Vec2::new(w_size.x, (20.0 * systems.scale as f32).floor()),
            Bounds::new(
                w_pos.x,
                w_pos.y + (242.0 * systems.scale as f32).floor(),
                w_pos.x + w_size.x,
                w_pos.y + (262.0 * systems.scale as f32).floor(),
            ),
            Color::rgba(200, 200, 200, 255),
        );
        let header_text =
            systems.gfx.add_text(text, 1, "Profile Header Text", false);
        systems
            .gfx
            .set_text(&mut systems.renderer, &header_text, "Profile");
        systems.gfx.center_text(&header_text);

        let mut button = Vec::with_capacity(1);
        let close_button = Button::new(
            systems,
            ButtonType::Rect(ButtonRect {
                rect_color: Color::rgba(70, 70, 70, 255),
                got_border: false,
                border_color: Color::rgba(0, 0, 0, 255),
                border_radius: 0.0,
                hover_change: ButtonChangeType::ColorChange(Color::rgba(
                    50, 50, 50, 255,
                )),
                click_change: ButtonChangeType::ColorChange(Color::rgba(
                    150, 150, 150, 255,
                )),
            }),
            ButtonContentType::Image(ButtonContentImg {
                res: systems.resource.window_button_icon.allocation,
                pos: Vec2::new(0.0, 0.0),
                uv: Vec2::new(0.0, 0.0),
                size: Vec2::new(20.0, 20.0),
                hover_change: ButtonChangeType::None,
                click_change: ButtonChangeType::None,
            }),
            Vec2::new(w_pos.x, w_pos.y),
            Vec2::new(header_size.x - 25.0, 242.0),
            detail_2,
            (0.0001, 4),
            Vec2::new(20.0, 20.0),
            0,
            false,
            None,
        );
        button.push(close_button);

        let mut slot = [GfxType::None; MAX_EQPT];
        for (i, slot) in slot.iter_mut().enumerate() {
            let box_rect = Rect::new(
                &mut systems.renderer,
                Vec3::new(
                    w_pos.x
                        + ((10.0 + (37.0 * i as f32)) * systems.scale as f32)
                            .floor(),
                    w_pos.y + (10.0 * systems.scale as f32).floor(),
                    detail_1,
                ),
                (Vec2::new(32.0, 32.0) * systems.scale as f32).floor(),
                Color::rgba(200, 200, 200, 255),
                0,
            );
            *slot =
                systems
                    .gfx
                    .add_rect(box_rect, 0, "Profile EQ Slot BG", false);
        }

        let mut fixed_label = Vec::with_capacity(5);
        for index in 0..5 {
            let (pos, size, msg) = match index {
                0 => (
                    Vec3::new(
                        w_pos.x + (10.0 * systems.scale as f32).floor(),
                        w_pos.y + w_size.y
                            - (55.0 * systems.scale as f32).floor(),
                        detail_1,
                    ),
                    (Vec2::new(100.0, 20.0) * systems.scale as f32).floor(),
                    "Level",
                ),
                1 => (
                    Vec3::new(
                        w_pos.x + (10.0 * systems.scale as f32).floor(),
                        w_pos.y + w_size.y
                            - (80.0 * systems.scale as f32).floor(),
                        detail_1,
                    ),
                    (Vec2::new(100.0, 20.0) * systems.scale as f32).floor(),
                    "Money",
                ),
                2 => (
                    Vec3::new(
                        w_pos.x + (10.0 * systems.scale as f32).floor(),
                        w_pos.y + w_size.y
                            - (105.0 * systems.scale as f32).floor(),
                        detail_1,
                    ),
                    (Vec2::new(100.0, 20.0) * systems.scale as f32).floor(),
                    "Damage",
                ),
                3 => (
                    Vec3::new(
                        w_pos.x + (10.0 * systems.scale as f32).floor(),
                        w_pos.y + w_size.y
                            - (130.0 * systems.scale as f32).floor(),
                        detail_1,
                    ),
                    (Vec2::new(100.0, 20.0) * systems.scale as f32).floor(),
                    "Defense",
                ),
                _ => (
                    Vec3::new(
                        w_pos.x + (10.0 * systems.scale as f32).floor(),
                        w_pos.y + (47.0 * systems.scale as f32).floor(),
                        detail_1,
                    ),
                    (Vec2::new(100.0, 20.0) * systems.scale as f32).floor(),
                    "Equipment",
                ),
            };
            let text = create_label(
                systems,
                pos,
                size,
                Bounds::new(pos.x, pos.y, pos.x + size.x, pos.y + size.y),
                Color::rgba(200, 200, 200, 255),
            );
            let label = systems.gfx.add_text(text, 1, "Profile Label", false);
            systems.gfx.set_text(&mut systems.renderer, &label, msg);
            fixed_label.push(label);
        }

        let mut value_label = Vec::with_capacity(4);
        for index in 0..4 {
            let (pos, size) = (
                Vec3::new(
                    w_pos.x + (90.0 * systems.scale as f32).floor(),
                    w_pos.y + w_size.y
                        - ((55.0 + (25.0 * index as f32))
                            * systems.scale as f32)
                            .floor(),
                    detail_1,
                ),
                (Vec2::new(100.0, 20.0) * systems.scale as f32).floor(),
            );
            let text = create_label(
                systems,
                pos,
                size,
                Bounds::new(pos.x, pos.y, pos.x + size.x, pos.y + size.y),
                Color::rgba(200, 200, 200, 255),
            );
            let label =
                systems.gfx.add_text(text, 1, "Profile Label Value", false);
            systems.gfx.set_text(&mut systems.renderer, &label, "0");
            value_label.push(label);
        }

        Profile {
            visible: false,
            bg,
            header,
            header_text,
            button,
            fixed_label,
            value_label,
            slot,
            eq_data: [None; MAX_EQPT],

            pos,
            size: w_size,
            z_order: 0.0,
            order_index: 0,
            in_hold: false,
            hold_pos: Vec2::new(0.0, 0.0),
            header_pos,
            header_size,
            did_button_click: false,

            min_bound: Vec2::new(
                systems.size.width - w_size.x - 1.0,
                systems.size.height - w_size.y - 1.0,
            ),
            max_bound: Vec2::new(1.0, 1.0),
        }
    }

    pub fn unload(&mut self, systems: &mut SystemHolder) {
        systems.gfx.remove_gfx(&mut systems.renderer, &self.bg);
        systems.gfx.remove_gfx(&mut systems.renderer, &self.header);
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, &self.header_text);
        self.button.iter_mut().for_each(|button| {
            button.unload(systems);
        });
        self.button.clear();
        self.slot.iter().for_each(|slot| {
            systems.gfx.remove_gfx(&mut systems.renderer, slot);
        });
        for i in 0..MAX_EQPT {
            if let Some(data) = self.eq_data[i] {
                systems.gfx.remove_gfx(&mut systems.renderer, &data.img);
            }
            self.eq_data[i] = None;
        }
        self.fixed_label.iter().for_each(|label| {
            systems.gfx.remove_gfx(&mut systems.renderer, label);
        });
        self.value_label.iter().for_each(|label| {
            systems.gfx.remove_gfx(&mut systems.renderer, label);
        });
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        if self.visible == visible {
            return;
        }
        self.visible = visible;
        self.z_order = 0.0;
        systems.gfx.set_visible(&self.bg, visible);
        systems.gfx.set_visible(&self.header, visible);
        systems.gfx.set_visible(&self.header_text, visible);
        self.button.iter_mut().for_each(|button| {
            button.set_visible(systems, visible);
        });
        self.slot.iter().for_each(|slot| {
            systems.gfx.set_visible(slot, visible);
        });
        self.eq_data.iter().for_each(|slot| {
            if let Some(data) = slot {
                systems.gfx.set_visible(&data.img, visible);
            }
        });
        self.fixed_label.iter().for_each(|label| {
            systems.gfx.set_visible(label, visible);
        });
        self.value_label.iter().for_each(|label| {
            systems.gfx.set_visible(label, visible);
        });
    }

    pub fn can_hold(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
    ) -> bool {
        if !self.visible {
            return false;
        }
        is_within_area(
            screen_pos,
            self.header_pos,
            (self.header_size * systems.scale as f32).floor(),
        )
    }

    pub fn in_window(&mut self, screen_pos: Vec2) -> bool {
        if !self.visible {
            return false;
        }
        is_within_area(screen_pos, self.pos, self.size)
    }

    pub fn hold_window(&mut self, screen_pos: Vec2) {
        if self.in_hold {
            return;
        }
        self.in_hold = true;
        self.hold_pos = screen_pos - self.pos;
    }

    pub fn release_window(&mut self) {
        self.in_hold = false;
    }

    pub fn set_z_order(
        &mut self,
        systems: &mut SystemHolder,
        z_order: f32,
        order_index: usize,
    ) {
        if self.z_order == z_order {
            return;
        }
        self.z_order = z_order;
        self.order_index = order_index;

        let detail_origin = ORDER_GUI_WINDOW.sub_f32(self.z_order, 3);
        let detail_1 = detail_origin.sub_f32(0.001, 3);
        let detail_2 = detail_origin.sub_f32(0.002, 3);

        let mut pos = systems.gfx.get_pos(&self.bg);
        pos.z = detail_origin;
        systems.gfx.set_pos(&self.bg, pos);

        let mut pos = systems.gfx.get_pos(&self.header);
        let header_zpos = detail_1;
        pos.z = header_zpos;
        systems.gfx.set_pos(&self.header, pos);

        let mut pos = systems.gfx.get_pos(&self.header_text);
        pos.z = detail_2;
        systems.gfx.set_pos(&self.header_text, pos);

        self.button.iter_mut().for_each(|button| {
            button.set_z_order(systems, detail_2);
        });

        for i in 0..MAX_EQPT {
            let mut pos = systems.gfx.get_pos(&self.slot[i]);
            pos.z = detail_1;
            systems.gfx.set_pos(&self.slot[i], pos);

            if let Some(data) = self.eq_data[i] {
                let mut pos = systems.gfx.get_pos(&data.img);
                pos.z = detail_2;
                systems.gfx.set_pos(&data.img, pos);
            }
        }

        self.fixed_label.iter().for_each(|label| {
            let mut pos = systems.gfx.get_pos(label);
            pos.z = detail_1;
            systems.gfx.set_pos(label, pos);
        });
        self.value_label.iter().for_each(|label| {
            let mut pos = systems.gfx.get_pos(label);
            pos.z = detail_1;
            systems.gfx.set_pos(label, pos);
        });
    }

    pub fn move_window(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
    ) {
        if !self.in_hold {
            return;
        }
        self.pos = (screen_pos - self.hold_pos)
            .max(self.max_bound)
            .min(self.min_bound);

        let pos = systems.gfx.get_pos(&self.bg);
        systems.gfx.set_pos(
            &self.bg,
            Vec3::new(self.pos.x - 1.0, self.pos.y - 1.0, pos.z),
        );
        let pos = systems.gfx.get_pos(&self.header);
        self.header_pos = Vec2::new(
            self.pos.x,
            self.pos.y + (237.0 * systems.scale as f32).floor(),
        );
        systems.gfx.set_pos(
            &self.header,
            Vec3::new(
                self.pos.x,
                self.pos.y + (237.0 * systems.scale as f32).floor(),
                pos.z,
            ),
        );
        let pos = systems.gfx.get_pos(&self.header_text);
        systems.gfx.set_pos(
            &self.header_text,
            Vec3::new(
                self.pos.x,
                self.pos.y + (242.0 * systems.scale as f32).floor(),
                pos.z,
            ),
        );
        systems.gfx.set_bound(
            &self.header_text,
            Bounds::new(
                self.pos.x,
                self.pos.y + (242.0 * systems.scale as f32).floor(),
                self.pos.x + self.size.x,
                self.pos.y + (262.0 * systems.scale as f32).floor(),
            ),
        );
        systems.gfx.center_text(&self.header_text);

        self.button.iter_mut().for_each(|button| {
            button.set_pos(systems, self.pos);
        });

        for i in 0..MAX_EQPT {
            let slot_pos = Vec2::new(
                self.pos.x
                    + ((10.0 + (37.0 * i as f32)) * systems.scale as f32)
                        .floor(),
                self.pos.y + (10.0 * systems.scale as f32).floor(),
            );

            let pos = systems.gfx.get_pos(&self.slot[i]);
            systems.gfx.set_pos(
                &self.slot[i],
                Vec3::new(slot_pos.x, slot_pos.y, pos.z),
            );

            if let Some(data) = self.eq_data[i] {
                let pos = systems.gfx.get_pos(&data.img);
                systems.gfx.set_pos(
                    &data.img,
                    Vec3::new(
                        slot_pos.x + (6.0 * systems.scale as f32).floor(),
                        slot_pos.y + (6.0 * systems.scale as f32).floor(),
                        pos.z,
                    ),
                );
            }
        }

        for index in 0..5 {
            let spos = systems.gfx.get_pos(&self.fixed_label[index]);
            let (pos, size) = match index {
                0 => (
                    Vec3::new(
                        self.pos.x + (10.0 * systems.scale as f32).floor(),
                        self.pos.y + self.size.y
                            - (55.0 * systems.scale as f32).floor(),
                        spos.z,
                    ),
                    (Vec2::new(100.0, 20.0) * systems.scale as f32).floor(),
                ),
                1 => (
                    Vec3::new(
                        self.pos.x + (10.0 * systems.scale as f32).floor(),
                        self.pos.y + self.size.y
                            - (80.0 * systems.scale as f32).floor(),
                        spos.z,
                    ),
                    (Vec2::new(100.0, 20.0) * systems.scale as f32).floor(),
                ),
                2 => (
                    Vec3::new(
                        self.pos.x + (10.0 * systems.scale as f32).floor(),
                        self.pos.y + self.size.y
                            - (105.0 * systems.scale as f32).floor(),
                        spos.z,
                    ),
                    (Vec2::new(100.0, 20.0) * systems.scale as f32).floor(),
                ),
                3 => (
                    Vec3::new(
                        self.pos.x + (10.0 * systems.scale as f32).floor(),
                        self.pos.y + self.size.y
                            - (130.0 * systems.scale as f32).floor(),
                        spos.z,
                    ),
                    (Vec2::new(100.0, 20.0) * systems.scale as f32).floor(),
                ),
                _ => (
                    Vec3::new(
                        self.pos.x + (10.0 * systems.scale as f32).floor(),
                        self.pos.y + (47.0 * systems.scale as f32).floor(),
                        spos.z,
                    ),
                    (Vec2::new(100.0, 20.0) * systems.scale as f32).floor(),
                ),
            };
            systems.gfx.set_pos(
                &self.fixed_label[index],
                Vec3::new(pos.x, pos.y, pos.z),
            );
            systems.gfx.set_bound(
                &self.fixed_label[index],
                Bounds::new(pos.x, pos.y, pos.x + size.x, pos.y + size.y),
            );
        }

        for index in 0..4 {
            let spos = systems.gfx.get_pos(&self.value_label[index]);
            let (pos, size) = (
                Vec3::new(
                    self.pos.x + (90.0 * systems.scale as f32).floor(),
                    self.pos.y + self.size.y
                        - ((55.0 + (25.0 * index as f32))
                            * systems.scale as f32)
                            .floor(),
                    spos.z,
                ),
                (Vec2::new(100.0, 20.0) * systems.scale as f32).floor(),
            );
            systems.gfx.set_pos(
                &self.value_label[index],
                Vec3::new(pos.x, pos.y, pos.z),
            );
            systems.gfx.set_bound(
                &self.value_label[index],
                Bounds::new(pos.x, pos.y, pos.x + size.x, pos.y + size.y),
            );
        }
    }

    pub fn hover_data(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
        itemdesc: &mut ItemDescription,
    ) {
        if !self.visible || self.order_index != 0 {
            return;
        }

        if let Some(slot) = self.find_eq_slot(systems, screen_pos, false) {
            if let Some(data) = self.eq_data[slot] {
                itemdesc.set_visible(systems, true);
                itemdesc.set_data(systems, data.index);
                itemdesc.set_position(systems, screen_pos);
            }
        } else {
            itemdesc.set_visible(systems, false);
        }
    }

    pub fn hover_buttons(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
    ) {
        if !self.visible {
            return;
        }

        for button in self.button.iter_mut() {
            if is_within_area(
                screen_pos,
                Vec2::new(
                    button.base_pos.x
                        + (button.adjust_pos.x * systems.scale as f32).floor(),
                    button.base_pos.y
                        + (button.adjust_pos.y * systems.scale as f32).floor(),
                ),
                (button.size * systems.scale as f32).floor(),
            ) {
                button.set_hover(systems, true);
            } else {
                button.set_hover(systems, false);
            }
        }
    }

    pub fn click_buttons(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
    ) -> Option<usize> {
        if !self.visible {
            return None;
        }

        let mut button_found = None;
        for (index, button) in self.button.iter_mut().enumerate() {
            if is_within_area(
                screen_pos,
                Vec2::new(
                    button.base_pos.x
                        + (button.adjust_pos.x * systems.scale as f32).floor(),
                    button.base_pos.y
                        + (button.adjust_pos.y * systems.scale as f32).floor(),
                ),
                (button.size * systems.scale as f32).floor(),
            ) {
                button.set_click(systems, true);
                button_found = Some(index)
            }
        }
        button_found
    }

    pub fn reset_buttons(&mut self, systems: &mut SystemHolder) {
        if !self.did_button_click || !self.visible {
            return;
        }
        self.did_button_click = false;

        self.button.iter_mut().for_each(|button| {
            button.set_click(systems, false);
        });
    }

    pub fn find_eq_slot(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
        check_empty: bool,
    ) -> Option<usize> {
        for slot in 0..MAX_EQPT {
            let can_proceed = if self.eq_data[slot].is_some() {
                true
            } else {
                check_empty
            };
            if can_proceed {
                let slot_pos = Vec2::new(
                    self.pos.x
                        + ((10.0 + (37.0 * slot as f32))
                            * systems.scale as f32)
                            .floor(),
                    self.pos.y + (10.0 * systems.scale as f32).floor(),
                );

                if screen_pos.x >= slot_pos.x
                    && screen_pos.x
                        <= slot_pos.x + (32.0 * systems.scale as f32).floor()
                    && screen_pos.y >= slot_pos.y
                    && screen_pos.y
                        <= slot_pos.y + (32.0 * systems.scale as f32).floor()
                {
                    return Some(slot);
                }
            }
        }
        None
    }

    pub fn set_profile_label_value(
        &mut self,
        systems: &mut SystemHolder,
        label: ProfileLabel,
        value: u64,
    ) {
        let label_index = match label {
            ProfileLabel::Level => 0,
            ProfileLabel::Money => 1,
            ProfileLabel::Damage => 2,
            ProfileLabel::Defense => 3,
        };
        systems.gfx.set_text(
            &mut systems.renderer,
            &self.value_label[label_index],
            &format!("{value}"),
        );
    }

    pub fn update_equipment_slot(
        &mut self,
        systems: &mut SystemHolder,
        slot: usize,
        item: &Item,
    ) {
        if let Some(data) = self.eq_data[slot] {
            systems.gfx.remove_gfx(&mut systems.renderer, &data.img);
        }

        if item.val == 0 {
            self.eq_data[slot] = None;
            return;
        }

        let detail_origin = ORDER_GUI_WINDOW.sub_f32(self.z_order, 3);
        let z_order = detail_origin.sub_f32(0.002, 3);

        let slot_pos = Vec2::new(
            self.pos.x
                + ((10.0 + (37.0 * slot as f32)) * systems.scale as f32)
                    .floor(),
            self.pos.y + (10.0 * systems.scale as f32).floor(),
        );

        let item_sprite = systems.base.item[item.num as usize].sprite;

        let img = Image::new(
            Some(systems.resource.items[item_sprite as usize].allocation),
            &mut systems.renderer,
            Vec3::new(
                slot_pos.x + (6.0 * systems.scale as f32).floor(),
                slot_pos.y + (6.0 * systems.scale as f32).floor(),
                z_order,
            ),
            (Vec2::new(20.0, 20.0) * systems.scale as f32).floor(),
            Vec4::new(0.0, 0.0, 20.0, 20.0),
            0,
        );
        let eq_img =
            systems
                .gfx
                .add_image(img, 0, "Profile EQ Image", self.visible);

        self.eq_data[slot] = Some(EqData {
            img: eq_img,
            index: item.num as usize,
        });
    }
}
