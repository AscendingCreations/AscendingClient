use graphics::*;

use crate::{
    is_within_area, logic::*, socket::sends::*, values::*, widget::*, Alert,
    AlertIndex, AlertType, Interface, Item, Result, Socket, SystemHolder,
};

const MAX_INV_X: f32 = 5.0;

#[derive(Clone, Copy, Default)]
struct ItemSlot {
    need_update: bool,
    got_data: bool,
    got_count: bool,
    image: usize,
    count_bg: usize,
    count: usize,
    item_index: u16,
    count_data: u16,
}

pub struct Inventory {
    pub visible: bool,
    bg: usize,
    header: usize,
    header_text: usize,
    slot: [usize; MAX_INV],
    item_slot: [ItemSlot; MAX_INV],
    button: Vec<Button>,

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

    pub hold_slot: Option<usize>,
    pub hold_adjust_pos: Vec2,
}

impl Inventory {
    pub fn new(systems: &mut SystemHolder) -> Self {
        let w_size = Vec2::new(200.0, 267.0);
        let w_pos = Vec3::new(
            systems.size.width - w_size.x - 10.0,
            60.0,
            ORDER_GUI_WINDOW,
        );
        let pos = Vec2::new(w_pos.x, w_pos.y);

        let detail_1 = w_pos.z.sub_f32(0.001, 3);
        let detail_2 = w_pos.z.sub_f32(0.002, 3);

        let mut rect = Rect::new(&mut systems.renderer, 0);
        rect.set_position(Vec3::new(pos.x - 1.0, pos.y - 1.0, w_pos.z))
            .set_size(w_size + 2.0)
            .set_color(Color::rgba(110, 110, 110, 255))
            .set_border_width(1.0)
            .set_border_color(Color::rgba(20, 20, 20, 255));
        let bg = systems.gfx.add_rect(rect, 0);
        systems.gfx.set_visible(bg, false);

        let mut header_rect = Rect::new(&mut systems.renderer, 0);
        let header_pos = Vec2::new(pos.x, pos.y + 237.0);
        let header_size = Vec2::new(w_size.x, 30.0);
        let header_zpos = detail_1;
        header_rect
            .set_position(Vec3::new(header_pos.x, header_pos.y, header_zpos))
            .set_size(header_size)
            .set_color(Color::rgba(70, 70, 70, 255));
        let header = systems.gfx.add_rect(header_rect, 0);
        systems.gfx.set_visible(header, false);

        let text = create_label(
            systems,
            Vec3::new(pos.x, pos.y + 242.0, detail_2),
            Vec2::new(w_size.x, 20.0),
            Bounds::new(pos.x, pos.y + 242.0, pos.x + w_size.x, pos.y + 262.0),
            Color::rgba(200, 200, 200, 255),
        );
        let header_text = systems.gfx.add_text(text, 1);
        systems
            .gfx
            .set_text(&mut systems.renderer, header_text, "Inventory");
        systems.gfx.center_text(header_text);
        systems.gfx.set_visible(header_text, false);

        let mut slot = [0; MAX_INV];
        for (i, slot) in slot.iter_mut().enumerate() {
            let mut box_rect = Rect::new(&mut systems.renderer, 0);
            let frame_pos =
                Vec2::new(i as f32 % MAX_INV_X, (i as f32 / MAX_INV_X).floor());
            box_rect
                .set_position(Vec3::new(
                    w_pos.x + 10.0 + (37.0 * frame_pos.x),
                    w_pos.y + 10.0 + (37.0 * frame_pos.y),
                    detail_1,
                ))
                .set_size(Vec2::new(32.0, 32.0))
                .set_color(Color::rgba(200, 200, 200, 255));
            *slot = systems.gfx.add_rect(box_rect, 0);
            systems.gfx.set_visible(*slot, false);
        }

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

        Inventory {
            visible: false,
            bg,
            header,
            header_text,
            slot,
            item_slot: [ItemSlot::default(); MAX_INV],
            button,

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

            hold_slot: None,
            hold_adjust_pos: Vec2::new(0.0, 0.0),
        }
    }

    pub fn unload(&mut self, systems: &mut SystemHolder) {
        systems.gfx.remove_gfx(self.bg);
        systems.gfx.remove_gfx(self.header);
        systems.gfx.remove_gfx(self.header_text);
        self.slot.iter().for_each(|slot| {
            systems.gfx.remove_gfx(*slot);
        });
        self.item_slot.iter().for_each(|item_slot| {
            if item_slot.got_data {
                systems.gfx.remove_gfx(item_slot.image);
                if item_slot.got_count {
                    systems.gfx.remove_gfx(item_slot.count_bg);
                    systems.gfx.remove_gfx(item_slot.count);
                }
            }
        });
        self.button.iter_mut().for_each(|button| {
            button.unload(systems);
        });
        self.button.clear();
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        if self.visible == visible {
            return;
        }
        self.visible = visible;
        self.z_order = 0.0;
        systems.gfx.set_visible(self.bg, visible);
        systems.gfx.set_visible(self.header, visible);
        systems.gfx.set_visible(self.header_text, visible);
        self.slot.iter().for_each(|slot| {
            systems.gfx.set_visible(*slot, visible);
        });
        self.item_slot.iter().for_each(|item_slot| {
            if item_slot.got_data {
                systems.gfx.set_visible(item_slot.image, visible);
                if item_slot.got_count {
                    systems.gfx.set_visible(item_slot.count_bg, visible);
                    systems.gfx.set_visible(item_slot.count, visible);
                }
            }
        });
        self.hold_slot = None;
        self.button.iter_mut().for_each(|button| {
            button.set_visible(systems, visible);
        })
    }

    pub fn hold_inv_slot(&mut self, slot: usize, screen_pos: Vec2) {
        if self.hold_slot.is_some() || self.item_slot[slot].need_update {
            return;
        }

        self.hold_slot = Some(slot);

        let frame_pos = Vec2::new(
            slot as f32 % MAX_INV_X,
            (slot as f32 / MAX_INV_X).floor(),
        );
        let slot_pos = Vec2::new(
            self.pos.x + 16.0 + (37.0 * frame_pos.x),
            self.pos.y + 195.0 - (37.0 * frame_pos.y),
        );

        self.hold_adjust_pos = screen_pos - slot_pos;
    }

    pub fn move_inv_slot(
        &mut self,
        systems: &mut SystemHolder,
        slot: usize,
        screen_pos: Vec2,
    ) {
        if slot >= MAX_INV || !self.item_slot[slot].got_data {
            return;
        }

        systems.gfx.set_pos(
            self.item_slot[slot].image,
            Vec3::new(
                screen_pos.x - self.hold_adjust_pos.x,
                screen_pos.y - self.hold_adjust_pos.x,
                ORDER_HOLD_ITEM,
            ),
        );

        if self.item_slot[slot].got_count {
            systems.gfx.set_visible(self.item_slot[slot].count, false);
            systems
                .gfx
                .set_visible(self.item_slot[slot].count_bg, false);
        }
    }

    pub fn update_inv_slot(
        &mut self,
        systems: &mut SystemHolder,
        slot: usize,
        data: &Item,
    ) {
        if slot >= MAX_INV {
            return;
        }

        self.item_slot[slot].need_update = false;

        if self.item_slot[slot].got_data {
            if self.item_slot[slot].item_index == data.num as u16
                && self.item_slot[slot].count_data == data.val
            {
                return;
            }
            systems.gfx.remove_gfx(self.item_slot[slot].image);
            if self.item_slot[slot].got_count {
                systems.gfx.remove_gfx(self.item_slot[slot].count_bg);
                systems.gfx.remove_gfx(self.item_slot[slot].count);
            }
            self.item_slot[slot].got_data = false;
            self.item_slot[slot].got_count = false;
            self.item_slot[slot].item_index = 0;
            self.item_slot[slot].count_data = 0;
        }

        if data.val == 0 {
            return;
        }

        let detail_origin = ORDER_GUI_WINDOW.sub_f32(self.z_order, 3);
        let item_zpos = detail_origin.sub_f32(0.002, 3);
        let textbg_zpos = detail_origin.sub_f32(0.003, 3);
        let text_zpos = detail_origin.sub_f32(0.004, 3);

        let frame_pos = Vec2::new(
            slot as f32 % MAX_INV_X,
            (slot as f32 / MAX_INV_X).floor(),
        );
        let slot_pos = Vec2::new(
            self.pos.x + 10.0 + (37.0 * frame_pos.x),
            self.pos.y + 195.0 - (37.0 * frame_pos.y),
        );

        let sprite =
            if let Some(itemdata) = systems.base.item.get(data.num as usize) {
                itemdata.sprite as usize
            } else {
                0
            };

        let mut image = Image::new(
            Some(systems.resource.items[sprite].allocation),
            &mut systems.renderer,
            0,
        );
        image.hw = Vec2::new(20.0, 20.0);
        image.uv = Vec4::new(0.0, 0.0, 20.0, 20.0);
        image.pos = Vec3::new(slot_pos.x + 6.0, slot_pos.y + 6.0, item_zpos);
        let image_index = systems.gfx.add_image(image, 0);
        systems.gfx.set_visible(image_index, self.visible);

        self.item_slot[slot].image = image_index;
        self.item_slot[slot].item_index = data.num as u16;
        self.item_slot[slot].count_data = data.val;

        if data.val > 1 {
            let mut text_bg = Rect::new(&mut systems.renderer, 0);
            text_bg
                .set_size(Vec2::new(32.0, 16.0))
                .set_position(Vec3::new(slot_pos.x, slot_pos.y, textbg_zpos))
                .set_color(Color::rgba(20, 20, 20, 120))
                .set_border_width(1.0)
                .set_border_color(Color::rgba(50, 50, 50, 180));
            let text_bg_index = systems.gfx.add_rect(text_bg, 1);
            systems.gfx.set_visible(text_bg_index, self.visible);

            let text_size = Vec2::new(32.0, 16.0);
            let text = create_label(
                systems,
                Vec3::new(slot_pos.x + 2.0, slot_pos.y + 2.0, text_zpos),
                text_size,
                Bounds::new(
                    slot_pos.x,
                    slot_pos.y,
                    slot_pos.x + text_size.x,
                    slot_pos.y + text_size.y,
                ),
                Color::rgba(240, 240, 240, 255),
            );
            let text_index = systems.gfx.add_text(text, 2);
            systems.gfx.set_text(
                &mut systems.renderer,
                text_index,
                &format!("{}", data.val),
            );
            systems.gfx.set_visible(text_index, self.visible);

            self.item_slot[slot].count = text_index;
            self.item_slot[slot].count_bg = text_bg_index;
            self.item_slot[slot].got_count = true;
        }

        self.item_slot[slot].got_data = true;
    }

    pub fn can_hold(&mut self, screen_pos: Vec2) -> bool {
        if !self.visible {
            return false;
        }
        is_within_area(screen_pos, self.header_pos, self.header_size)
    }

    pub fn find_inv_slot(
        &mut self,
        screen_pos: Vec2,
        check_empty: bool,
    ) -> Option<usize> {
        for slot in 0..MAX_INV {
            let can_proceed = if self.item_slot[slot].got_data {
                true
            } else {
                check_empty
            };
            if can_proceed {
                let frame_pos = Vec2::new(
                    slot as f32 % MAX_INV_X,
                    (slot as f32 / MAX_INV_X).floor(),
                );
                let slot_pos = Vec2::new(
                    self.pos.x + 10.0 + (37.0 * frame_pos.x),
                    self.pos.y + 195.0 - (37.0 * frame_pos.y),
                );

                if screen_pos.x >= slot_pos.x
                    && screen_pos.x <= slot_pos.x + 32.0
                    && screen_pos.y >= slot_pos.y
                    && screen_pos.y <= slot_pos.y + 32.0
                {
                    return Some(slot);
                }
            }
        }
        None
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
        self.hold_pos = Vec2::new(0.0, 0.0);
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
        let detail_3 = detail_origin.sub_f32(0.003, 3);
        let detail_4 = detail_origin.sub_f32(0.004, 3);

        let mut pos = systems.gfx.get_pos(self.bg);
        pos.z = detail_origin;
        systems.gfx.set_pos(self.bg, pos);

        let mut pos = systems.gfx.get_pos(self.header);
        let header_zpos = detail_1;
        pos.z = header_zpos;
        systems.gfx.set_pos(self.header, pos);

        let mut pos = systems.gfx.get_pos(self.header_text);
        pos.z = detail_2;
        systems.gfx.set_pos(self.header_text, pos);

        for i in 0..MAX_INV {
            let mut pos = systems.gfx.get_pos(self.slot[i]);
            pos.z = detail_1;
            systems.gfx.set_pos(self.slot[i], pos);

            let can_proceed = if let Some(hold_slot) = self.hold_slot {
                hold_slot != i
            } else {
                true
            };
            if self.item_slot[i].got_data && can_proceed {
                let mut pos = systems.gfx.get_pos(self.item_slot[i].image);
                pos.z = detail_2;
                systems.gfx.set_pos(self.item_slot[i].image, pos);
            }

            if self.item_slot[i].got_count {
                let mut pos = systems.gfx.get_pos(self.item_slot[i].count_bg);
                pos.z = detail_3;
                systems.gfx.set_pos(self.item_slot[i].count_bg, pos);

                let mut pos = systems.gfx.get_pos(self.item_slot[i].count);
                pos.z = detail_4;
                systems.gfx.set_pos(self.item_slot[i].count, pos);
            }
        }

        self.button.iter_mut().for_each(|button| {
            button.set_z_order(systems, detail_2);
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

        let pos = systems.gfx.get_pos(self.bg);
        systems.gfx.set_pos(
            self.bg,
            Vec3::new(self.pos.x - 1.0, self.pos.y - 1.0, pos.z),
        );
        let pos = systems.gfx.get_pos(self.header);
        self.header_pos = Vec2::new(self.pos.x, self.pos.y + 237.0);
        systems.gfx.set_pos(
            self.header,
            Vec3::new(self.pos.x, self.pos.y + 237.0, pos.z),
        );
        let pos = systems.gfx.get_pos(self.header_text);
        systems.gfx.set_pos(
            self.header_text,
            Vec3::new(self.pos.x, self.pos.y + 242.0, pos.z),
        );
        systems.gfx.set_bound(
            self.header_text,
            Bounds::new(
                self.pos.x,
                self.pos.y + 242.0,
                self.pos.x + self.size.x,
                self.pos.y + 262.0,
            ),
        );
        systems.gfx.center_text(self.header_text);

        self.button.iter_mut().for_each(|button| {
            button.set_pos(systems, self.pos);
        });

        let item_text_size = Vec2::new(32.0, 16.0);
        for i in 0..MAX_INV {
            let frame_pos =
                Vec2::new(i as f32 % MAX_INV_X, (i as f32 / MAX_INV_X).floor());
            let slot_pos = Vec2::new(
                self.pos.x + 10.0 + (37.0 * frame_pos.x),
                self.pos.y + 195.0 - (37.0 * frame_pos.y),
            );

            let pos = systems.gfx.get_pos(self.slot[i]);
            systems.gfx.set_pos(
                self.slot[i],
                Vec3::new(slot_pos.x, slot_pos.y, pos.z),
            );

            if self.item_slot[i].got_data {
                let pos = systems.gfx.get_pos(self.item_slot[i].image);
                systems.gfx.set_pos(
                    self.item_slot[i].image,
                    Vec3::new(slot_pos.x + 6.0, slot_pos.y + 6.0, pos.z),
                );

                if self.item_slot[i].got_count {
                    let pos = systems.gfx.get_pos(self.item_slot[i].count_bg);
                    systems.gfx.set_pos(
                        self.item_slot[i].count_bg,
                        Vec3::new(slot_pos.x, slot_pos.y, pos.z),
                    );

                    let pos = systems.gfx.get_pos(self.item_slot[i].count);
                    systems.gfx.set_pos(
                        self.item_slot[i].count,
                        Vec3::new(slot_pos.x + 2.0, slot_pos.y + 2.0, pos.z),
                    );
                    systems.gfx.set_bound(
                        self.item_slot[i].count,
                        Bounds::new(
                            slot_pos.x,
                            slot_pos.y,
                            slot_pos.x + item_text_size.x,
                            slot_pos.y + item_text_size.y,
                        ),
                    );
                }
            }
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
                    button.base_pos.x + button.adjust_pos.x,
                    button.base_pos.y + button.adjust_pos.y,
                ),
                button.size,
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

    pub fn reset_buttons(&mut self, systems: &mut SystemHolder) {
        if !self.did_button_click || !self.visible {
            return;
        }
        self.did_button_click = false;

        self.button.iter_mut().for_each(|button| {
            button.set_click(systems, false);
        });
    }
}

pub fn release_inv_slot(
    interface: &mut Interface,
    socket: &mut Socket,
    systems: &mut SystemHolder,
    alert: &mut Alert,
    slot: usize,
    screen_pos: Vec2,
) -> Result<()> {
    if slot >= MAX_INV
        || !interface.inventory.item_slot[slot].got_data
        || interface.inventory.item_slot[slot].need_update
    {
        return Ok(());
    }

    if interface.inventory.in_window(screen_pos)
        && interface.inventory.order_index == 0
        && !interface.trade.visible
    {
        let find_slot = interface.inventory.find_inv_slot(screen_pos, true);
        if let Some(new_slot) = find_slot {
            if new_slot != slot {
                if interface.inventory.item_slot[slot].item_index
                    == interface.inventory.item_slot[new_slot].item_index
                {
                    alert.show_alert(
                        systems,
                        AlertType::Input,
                        String::new(),
                        "Enter the amount to merge".into(),
                        250,
                        AlertIndex::MergeInv(slot as u16, new_slot as u16),
                        true,
                    );
                } else {
                    send_switchinvslot(
                        socket,
                        slot as u16,
                        new_slot as u16,
                        interface.inventory.item_slot[slot].count_data,
                    )?;

                    interface.inventory.update_inv_slot(
                        systems,
                        slot,
                        &Item {
                            num: interface.inventory.item_slot[new_slot]
                                .item_index
                                as u32,
                            val: interface.inventory.item_slot[new_slot]
                                .count_data,
                            ..Default::default()
                        },
                    );
                    interface.inventory.update_inv_slot(
                        systems,
                        new_slot,
                        &Item {
                            num: interface.inventory.item_slot[slot].item_index
                                as u32,
                            val: interface.inventory.item_slot[slot].count_data,
                            ..Default::default()
                        },
                    );

                    interface.inventory.item_slot[slot].need_update = true;
                    interface.inventory.item_slot[new_slot].need_update = true;
                    return Ok(());
                }
            }
        }
    } else if interface.storage.in_window(screen_pos)
        && interface.storage.order_index == 0
    {
        let find_slot = interface.storage.find_storage_slot(screen_pos, true);
        if let Some(bank_slot) = find_slot {
            if interface.inventory.item_slot[slot].count_data > 1 {
                alert.show_alert(
                    systems,
                    AlertType::Input,
                    String::new(),
                    "Enter the amount to Deposit".into(),
                    250,
                    AlertIndex::Deposit(slot as u16, bank_slot as u16),
                    true,
                );
            } else {
                return send_deposititem(
                    socket,
                    slot as u16,
                    bank_slot as u16,
                    interface.inventory.item_slot[slot].count_data,
                );
            }
        }
    } else if interface.shop.in_window(screen_pos)
        && interface.shop.order_index == 0
    {
        if interface.inventory.item_slot[slot].count_data > 1 {
            alert.show_alert(
                systems,
                AlertType::Input,
                String::new(),
                "Enter the amount to Sell".into(),
                250,
                AlertIndex::Sell(slot as u16),
                true,
            );
        } else {
            send_sellitem(
                socket,
                slot as u16,
                interface.inventory.item_slot[slot].count_data,
            )?;
        }
    } else if interface.trade.in_window(screen_pos)
        && interface.trade.order_index == 0
        && interface.trade.trade_status == TradeStatus::None
    {
        if interface.inventory.item_slot[slot].count_data > 1 {
            alert.show_alert(
                systems,
                AlertType::Input,
                String::new(),
                "Enter the amount to Trade".into(),
                250,
                AlertIndex::AddTradeTradeItem(slot as u16),
                true,
            );
        } else {
            send_addtradeitem(
                socket,
                slot as u16,
                interface.inventory.item_slot[slot].count_data,
            )?;
        }
    } else if interface.inventory.item_slot[slot].count_data > 1 {
        alert.show_alert(
            systems,
            AlertType::Input,
            String::new(),
            "Enter the amount to Drop".into(),
            250,
            AlertIndex::Drop(slot as u16),
            true,
        );
    } else {
        send_dropitem(
            socket,
            slot as u16,
            interface.inventory.item_slot[slot].count_data,
        )?;
    }

    let detail_origin =
        ORDER_GUI_WINDOW.sub_f32(interface.inventory.z_order, 3);
    let z_pos = detail_origin.sub_f32(0.002, 3);

    let frame_pos =
        Vec2::new(slot as f32 % MAX_INV_X, (slot as f32 / MAX_INV_X).floor());
    let slot_pos = Vec2::new(
        interface.inventory.pos.x + 10.0 + (37.0 * frame_pos.x),
        interface.inventory.pos.y + 195.0 - (37.0 * frame_pos.y),
    );

    systems.gfx.set_pos(
        interface.inventory.item_slot[slot].image,
        Vec3::new(slot_pos.x + 6.0, slot_pos.y + 6.0, z_pos),
    );
    if interface.inventory.item_slot[slot].got_count {
        systems
            .gfx
            .set_visible(interface.inventory.item_slot[slot].count, true);
        systems
            .gfx
            .set_visible(interface.inventory.item_slot[slot].count_bg, true);
    }
    Ok(())
}
