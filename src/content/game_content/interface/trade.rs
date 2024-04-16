use graphics::*;

use crate::{data_types::*, is_within_area, logic::*, widget::*, SystemHolder};

use super::ItemDescription;

const MAX_TRADE_X: f32 = 5.0;

#[derive(Clone, Copy, Default)]
pub struct ItemSlot {
    need_update: bool,
    pub got_data: bool,
    got_count: bool,
    image: usize,
    count_bg: usize,
    count: usize,
    item_index: u16,
    pub count_data: u16,
}

pub struct Trade {
    pub visible: bool,
    bg: usize,
    header: usize,
    header_text: usize,
    pub button: Vec<Button>,
    name_bg: [usize; 2],
    money_icon: [usize; 2],
    slot: [usize; MAX_TRADE_SLOT * 2],
    pub money_input: Textbox,
    their_money: usize,
    pub my_items: [ItemSlot; MAX_TRADE_SLOT],
    their_items: [ItemSlot; MAX_TRADE_SLOT],
    my_status_text: usize,
    their_status_text: usize,
    status_text: usize,
    pub trade_status: TradeStatus,

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

impl Trade {
    pub fn new(systems: &mut SystemHolder) -> Self {
        let w_size = Vec2::new(402.0, 386.0);
        let w_pos = Vec3::new(
            systems.size.width - w_size.x - 10.0,
            60.0,
            ORDER_GUI_WINDOW,
        );
        let pos = Vec2::new(w_pos.x, w_pos.y);

        let detail_1 = w_pos.z.sub_f32(0.001, 3);
        let detail_2 = w_pos.z.sub_f32(0.002, 3);

        let mut rect = Rect::new(&mut systems.renderer, 0);
        rect.set_position(Vec3::new(w_pos.x - 1.0, w_pos.y - 1.0, w_pos.z))
            .set_size(w_size + 2.0)
            .set_color(Color::rgba(110, 110, 110, 255))
            .set_border_width(1.0)
            .set_border_color(Color::rgba(20, 20, 20, 255));
        let bg = systems.gfx.add_rect(rect, 0, "Trade Window".into());
        systems.gfx.set_visible(bg, false);

        let mut header_rect = Rect::new(&mut systems.renderer, 0);
        let header_pos = Vec2::new(w_pos.x, w_pos.y + 356.0);
        let header_size = Vec2::new(w_size.x, 30.0);
        let header_zpos = detail_1;
        header_rect
            .set_position(Vec3::new(header_pos.x, header_pos.y, header_zpos))
            .set_size(header_size)
            .set_color(Color::rgba(70, 70, 70, 255));
        let header =
            systems.gfx.add_rect(header_rect, 0, "Trade Header".into());
        systems.gfx.set_visible(header, false);

        let text = create_label(
            systems,
            Vec3::new(w_pos.x, w_pos.y + 361.0, detail_2),
            Vec2::new(w_size.x, 20.0),
            Bounds::new(
                w_pos.x,
                w_pos.y + 361.0,
                w_pos.x + w_size.x,
                w_pos.y + 381.0,
            ),
            Color::rgba(200, 200, 200, 255),
        );
        let header_text =
            systems.gfx.add_text(text, 1, "Trade Header Text".into());
        systems
            .gfx
            .set_text(&mut systems.renderer, header_text, "Trade");
        systems.gfx.center_text(header_text);
        systems.gfx.set_visible(header_text, false);

        let mut button = Vec::with_capacity(3);
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
            Vec2::new(header_size.x - 25.0, 361.0),
            detail_2,
            (0.0001, 4),
            Vec2::new(20.0, 20.0),
            0,
            false,
            None,
        );
        let confirm_button = Button::new(
            systems,
            ButtonType::Rect(ButtonRect {
                rect_color: Color::rgba(70, 70, 70, 255),
                got_border: true,
                border_color: Color::rgba(40, 40, 40, 255),
                border_radius: 0.0,
                hover_change: ButtonChangeType::ColorChange(Color::rgba(
                    150, 150, 150, 255,
                )),
                click_change: ButtonChangeType::ColorChange(Color::rgba(
                    200, 200, 200, 255,
                )),
            }),
            ButtonContentType::Text(ButtonContentText {
                text: "Confirm".into(),
                pos: Vec2::new(0.0, 5.0),
                color: Color::rgba(255, 255, 255, 255),
                render_layer: 1,
                hover_change: ButtonChangeType::None,
                click_change: ButtonChangeType::None,
            }),
            Vec2::new(w_pos.x, w_pos.y),
            Vec2::new(125.0, 35.0),
            detail_2,
            (0.0001, 4),
            Vec2::new(70.0, 30.0),
            0,
            false,
            None,
        );
        let cancel_button = Button::new(
            systems,
            ButtonType::Rect(ButtonRect {
                rect_color: Color::rgba(70, 70, 70, 255),
                got_border: true,
                border_color: Color::rgba(40, 40, 40, 255),
                border_radius: 0.0,
                hover_change: ButtonChangeType::ColorChange(Color::rgba(
                    150, 150, 150, 255,
                )),
                click_change: ButtonChangeType::ColorChange(Color::rgba(
                    200, 200, 200, 255,
                )),
            }),
            ButtonContentType::Text(ButtonContentText {
                text: "Cancel".into(),
                pos: Vec2::new(0.0, 5.0),
                color: Color::rgba(255, 255, 255, 255),
                render_layer: 1,
                hover_change: ButtonChangeType::None,
                click_change: ButtonChangeType::None,
            }),
            Vec2::new(w_pos.x, w_pos.y),
            Vec2::new(205.0, 35.0),
            detail_2,
            (0.0001, 4),
            Vec2::new(70.0, 30.0),
            0,
            false,
            None,
        );
        button.push(close_button);
        button.push(confirm_button);
        button.push(cancel_button);

        let mut slot = [0; MAX_TRADE_SLOT * 2];
        (0..MAX_TRADE_SLOT).for_each(|index| {
            let mut box_rect = Rect::new(&mut systems.renderer, 0);
            let frame_pos = Vec2::new(
                index as f32 % MAX_TRADE_X,
                (index as f32 / MAX_TRADE_X).floor(),
            );
            box_rect
                .set_position(Vec3::new(
                    w_pos.x + 10.0 + (37.0 * frame_pos.x),
                    w_pos.y + 287.0 - (37.0 * frame_pos.y),
                    detail_1,
                ))
                .set_size(Vec2::new(32.0, 32.0))
                .set_color(Color::rgba(200, 200, 200, 255));
            slot[index] =
                systems.gfx.add_rect(box_rect, 0, "Trade Slot BG".into());
            systems.gfx.set_visible(slot[index], false);
        });
        (MAX_TRADE_SLOT..MAX_TRADE_SLOT * 2).for_each(|index| {
            let render_index = index - MAX_TRADE_SLOT;
            let mut box_rect = Rect::new(&mut systems.renderer, 0);
            let frame_pos = Vec2::new(
                render_index as f32 % MAX_TRADE_X,
                (render_index as f32 / MAX_TRADE_X).floor(),
            );
            box_rect
                .set_position(Vec3::new(
                    w_pos.x + 210.0 + (37.0 * frame_pos.x),
                    w_pos.y + 287.0 - (37.0 * frame_pos.y),
                    detail_1,
                ))
                .set_size(Vec2::new(32.0, 32.0))
                .set_color(Color::rgba(200, 200, 200, 255));
            slot[index] =
                systems.gfx.add_rect(box_rect, 0, "Trade Slot BG".into());
            systems.gfx.set_visible(slot[index], false);
        });

        let mut my_name_bg = Rect::new(&mut systems.renderer, 0);
        my_name_bg
            .set_size(Vec2::new(180.0, 20.0))
            .set_color(Color::rgba(80, 80, 80, 255))
            .set_position(Vec3::new(w_pos.x + 10.0, w_pos.y + 324.0, detail_1));
        let mut their_name_bg = Rect::new(&mut systems.renderer, 0);
        their_name_bg
            .set_size(Vec2::new(180.0, 20.0))
            .set_color(Color::rgba(80, 80, 80, 255))
            .set_position(Vec3::new(
                w_pos.x + 210.0,
                w_pos.y + 324.0,
                detail_1,
            ));
        let name_bg = [
            systems.gfx.add_rect(my_name_bg, 0, "Trade Name".into()),
            systems.gfx.add_rect(their_name_bg, 0, "Trade Name".into()),
        ];
        systems.gfx.set_visible(name_bg[0], false);
        systems.gfx.set_visible(name_bg[1], false);

        let mystatus = create_label(
            systems,
            Vec3::new(w_pos.x + 15.0, w_pos.y + 324.0, detail_2),
            Vec2::new(180.0, 20.0),
            Bounds::new(
                w_pos.x + 15.0,
                w_pos.y + 324.0,
                w_pos.x + 185.0,
                w_pos.y + 344.0,
            ),
            Color::rgba(220, 220, 220, 255),
        );
        let my_status_text =
            systems.gfx.add_text(mystatus, 1, "Trade Status".into());
        systems.gfx.set_text(
            &mut systems.renderer,
            my_status_text,
            "My Trade: Preparing...",
        );
        systems.gfx.set_visible(my_status_text, false);
        let theirstatus = create_label(
            systems,
            Vec3::new(w_pos.x + 215.0, w_pos.y + 324.0, detail_2),
            Vec2::new(180.0, 20.0),
            Bounds::new(
                w_pos.x + 215.0,
                w_pos.y + 324.0,
                w_pos.x + 385.0,
                w_pos.y + 344.0,
            ),
            Color::rgba(220, 220, 220, 255),
        );
        let their_status_text =
            systems.gfx.add_text(theirstatus, 1, "Trade Status".into());
        systems.gfx.set_text(
            &mut systems.renderer,
            their_status_text,
            "Their Trade: Preparing...",
        );
        systems.gfx.set_visible(their_status_text, false);

        let mut my_money_icon = Image::new(
            Some(systems.resource.shop_currency_icon.allocation),
            &mut systems.renderer,
            0,
        );
        my_money_icon.pos = Vec3::new(w_pos.x + 10.0, w_pos.y + 77.0, detail_1);
        my_money_icon.hw = Vec2::new(20.0, 20.0);
        my_money_icon.uv = Vec4::new(0.0, 0.0, 20.0, 20.0);
        let mut their_money_icon = Image::new(
            Some(systems.resource.shop_currency_icon.allocation),
            &mut systems.renderer,
            0,
        );
        their_money_icon.pos =
            Vec3::new(w_pos.x + 210.0, w_pos.y + 77.0, detail_1);
        their_money_icon.hw = Vec2::new(20.0, 20.0);
        their_money_icon.uv = Vec4::new(0.0, 0.0, 20.0, 20.0);
        let money_icon = [
            systems
                .gfx
                .add_image(my_money_icon, 0, "Trade Money Icon".into()),
            systems.gfx.add_image(
                their_money_icon,
                0,
                "Trade Money Icon".into(),
            ),
        ];
        systems.gfx.set_visible(money_icon[0], false);
        systems.gfx.set_visible(money_icon[1], false);

        let mut money_input = Textbox::new(
            systems,
            Vec3::new(w_pos.x + 32.0, w_pos.y + 77.0, detail_1),
            (0.0001, 4),
            Vec2::new(158.0, 20.0),
            Color::rgba(200, 200, 200, 255),
            1,
            10,
            Color::rgba(80, 80, 80, 255),
            Color::rgba(10, 10, 150, 255),
            false,
            false,
            None,
            vec![],
        );
        money_input.set_text(systems, "0".into());

        let their_money_text = create_label(
            systems,
            Vec3::new(w_pos.x + 232.0, w_pos.y + 77.0, detail_1),
            Vec2::new(158.0, 20.0),
            Bounds::new(
                w_pos.x + 232.0,
                w_pos.y + 77.0,
                w_pos.x + 390.0,
                w_pos.y + 97.0,
            ),
            Color::rgba(200, 200, 200, 255),
        );
        let their_money =
            systems
                .gfx
                .add_text(their_money_text, 1, "Trade Money".into());
        systems
            .gfx
            .set_text(&mut systems.renderer, their_money, "0");
        systems.gfx.set_visible(their_money, false);

        let statustext = create_label(
            systems,
            Vec3::new(w_pos.x, w_pos.y + 10.0, detail_1),
            Vec2::new(180.0, 20.0),
            Bounds::new(
                w_pos.x,
                w_pos.y + 10.0,
                w_pos.x + w_size.x,
                w_pos.y + 30.0,
            ),
            Color::rgba(220, 220, 220, 255),
        );
        let status_text =
            systems
                .gfx
                .add_text(statustext, 1, "Trade Main Status".into());
        systems.gfx.set_text(&mut systems.renderer, status_text, "");
        systems.gfx.center_text(status_text);
        systems.gfx.set_visible(status_text, false);

        Trade {
            visible: false,
            bg,
            header,
            header_text,
            button,
            name_bg,
            money_icon,
            slot,
            money_input,
            their_money,
            my_items: [ItemSlot::default(); MAX_TRADE_SLOT],
            their_items: [ItemSlot::default(); MAX_TRADE_SLOT],
            trade_status: TradeStatus::None,
            my_status_text,
            their_status_text,
            status_text,

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
        systems.gfx.remove_gfx(&mut systems.renderer, self.bg);
        systems.gfx.remove_gfx(&mut systems.renderer, self.header);
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, self.header_text);
        self.button.iter_mut().for_each(|button| {
            button.unload(systems);
        });
        self.button.clear();
        self.slot.iter().for_each(|slot| {
            systems.gfx.remove_gfx(&mut systems.renderer, *slot);
        });
        self.name_bg.iter().for_each(|image| {
            systems.gfx.remove_gfx(&mut systems.renderer, *image);
        });
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, self.my_status_text);
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, self.their_status_text);
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, self.status_text);
        self.money_icon.iter().for_each(|image| {
            systems.gfx.remove_gfx(&mut systems.renderer, *image);
        });
        self.money_input.unload(systems);
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, self.their_money);
        self.trade_status = TradeStatus::default();
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
        self.button.iter_mut().for_each(|button| {
            button.set_visible(systems, visible);
        });
        self.slot.iter().for_each(|slot| {
            systems.gfx.set_visible(*slot, visible);
        });
        self.name_bg.iter().for_each(|image| {
            systems.gfx.set_visible(*image, visible);
        });
        systems.gfx.set_visible(self.my_status_text, visible);
        systems.gfx.set_visible(self.their_status_text, visible);
        systems.gfx.set_visible(self.status_text, visible);
        self.money_icon.iter().for_each(|image| {
            systems.gfx.set_visible(*image, visible);
        });
        self.money_input.set_visible(systems, visible);
        systems.gfx.set_visible(self.their_money, visible);
        if !visible {
            self.trade_status = TradeStatus::default();
            self.button[1].change_text(systems, "Submit".into());
            self.update_my_status(systems, "My Trade: Preparing...".into());
            self.update_their_status(systems, "My Trade: Preparing...".into());
            self.update_status(systems, String::new());
        }
    }

    pub fn can_hold(&mut self, screen_pos: Vec2) -> bool {
        if !self.visible {
            return false;
        }
        is_within_area(screen_pos, self.header_pos, self.header_size)
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

        self.button.iter_mut().for_each(|button| {
            button.set_z_order(systems, detail_2);
        });

        for i in 0..MAX_TRADE_SLOT * 2 {
            let mut pos = systems.gfx.get_pos(self.slot[i]);
            pos.z = detail_1;
            systems.gfx.set_pos(self.slot[i], pos);

            let (item_slot, render_index) = if i >= MAX_TRADE_SLOT {
                (&self.their_items, i - MAX_TRADE_SLOT)
            } else {
                (&self.my_items, i)
            };
            if item_slot[render_index].got_data {
                let mut pos =
                    systems.gfx.get_pos(item_slot[render_index].image);
                pos.z = detail_2;
                systems.gfx.set_pos(item_slot[render_index].image, pos);
            }
            if item_slot[render_index].got_count {
                let mut pos =
                    systems.gfx.get_pos(item_slot[render_index].count_bg);
                pos.z = detail_3;
                systems.gfx.set_pos(item_slot[render_index].count_bg, pos);

                let mut pos =
                    systems.gfx.get_pos(item_slot[render_index].count);
                pos.z = detail_4;
                systems.gfx.set_pos(item_slot[render_index].count, pos);
            }
        }

        self.name_bg.iter().for_each(|image| {
            let mut pos = systems.gfx.get_pos(*image);
            pos.z = detail_1;
            systems.gfx.set_pos(*image, pos);
        });

        let mut pos = systems.gfx.get_pos(self.my_status_text);
        pos.z = detail_2;
        systems.gfx.set_pos(self.my_status_text, pos);
        let mut pos = systems.gfx.get_pos(self.their_status_text);
        pos.z = detail_2;
        systems.gfx.set_pos(self.their_status_text, pos);

        self.money_icon.iter().for_each(|image| {
            let mut pos = systems.gfx.get_pos(*image);
            pos.z = detail_1;
            systems.gfx.set_pos(*image, pos);
        });

        self.money_input.set_z_order(systems, detail_1);

        let mut pos = systems.gfx.get_pos(self.their_money);
        pos.z = detail_2;
        systems.gfx.set_pos(self.their_money, pos);

        let mut pos = systems.gfx.get_pos(self.status_text);
        pos.z = detail_2;
        systems.gfx.set_pos(self.status_text, pos);
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
        self.header_pos = Vec2::new(self.pos.x, self.pos.y + 356.0);
        systems.gfx.set_pos(
            self.header,
            Vec3::new(self.pos.x, self.pos.y + 356.0, pos.z),
        );
        let pos = systems.gfx.get_pos(self.header_text);
        systems.gfx.set_pos(
            self.header_text,
            Vec3::new(self.pos.x, self.pos.y + 361.0, pos.z),
        );
        systems.gfx.set_bound(
            self.header_text,
            Bounds::new(
                self.pos.x,
                self.pos.y + 361.0,
                self.pos.x + self.size.x,
                self.pos.y + 381.0,
            ),
        );
        systems.gfx.center_text(self.header_text);

        self.button.iter_mut().for_each(|button| {
            button.set_pos(systems, self.pos);
        });

        let item_text_size = Vec2::new(32.0, 16.0);
        for i in 0..MAX_TRADE_SLOT * 2 {
            let (item_slot, render_index) = if i >= MAX_TRADE_SLOT {
                (&self.their_items, i - MAX_TRADE_SLOT)
            } else {
                (&self.my_items, i)
            };

            let frame_pos = Vec2::new(
                render_index as f32 % MAX_TRADE_X,
                (render_index as f32 / MAX_TRADE_X).floor(),
            );
            let slot_pos = if i >= MAX_TRADE_SLOT {
                Vec2::new(
                    self.pos.x + 210.0 + (37.0 * frame_pos.x),
                    self.pos.y + 287.0 - (37.0 * frame_pos.y),
                )
            } else {
                Vec2::new(
                    self.pos.x + 10.0 + (37.0 * frame_pos.x),
                    self.pos.y + 287.0 - (37.0 * frame_pos.y),
                )
            };

            let pos = systems.gfx.get_pos(self.slot[i]);
            systems.gfx.set_pos(
                self.slot[i],
                Vec3::new(slot_pos.x, slot_pos.y, pos.z),
            );

            if item_slot[render_index].got_data {
                let pos = systems.gfx.get_pos(item_slot[render_index].image);
                systems.gfx.set_pos(
                    item_slot[render_index].image,
                    Vec3::new(slot_pos.x + 6.0, slot_pos.y + 6.0, pos.z),
                );

                if item_slot[render_index].got_count {
                    let pos =
                        systems.gfx.get_pos(item_slot[render_index].count_bg);
                    systems.gfx.set_pos(
                        item_slot[render_index].count_bg,
                        Vec3::new(slot_pos.x, slot_pos.y, pos.z),
                    );

                    let pos =
                        systems.gfx.get_pos(item_slot[render_index].count);
                    systems.gfx.set_pos(
                        item_slot[render_index].count,
                        Vec3::new(slot_pos.x + 2.0, slot_pos.y + 2.0, pos.z),
                    );
                    systems.gfx.set_bound(
                        item_slot[render_index].count,
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

        let pos = systems.gfx.get_pos(self.name_bg[0]);
        systems.gfx.set_pos(
            self.name_bg[0],
            Vec3::new(self.pos.x + 10.0, self.pos.y + 324.0, pos.z),
        );
        let pos = systems.gfx.get_pos(self.name_bg[1]);
        systems.gfx.set_pos(
            self.name_bg[1],
            Vec3::new(self.pos.x + 210.0, self.pos.y + 324.0, pos.z),
        );

        let pos = systems.gfx.get_pos(self.my_status_text);
        systems.gfx.set_pos(
            self.my_status_text,
            Vec3::new(self.pos.x + 15.0, self.pos.y + 324.0, pos.z),
        );
        systems.gfx.set_bound(
            self.my_status_text,
            Bounds::new(
                self.pos.x + 15.0,
                self.pos.y + 324.0,
                self.pos.x + 185.0,
                self.pos.y + 344.0,
            ),
        );

        let pos = systems.gfx.get_pos(self.their_status_text);
        systems.gfx.set_pos(
            self.their_status_text,
            Vec3::new(self.pos.x + 215.0, self.pos.y + 324.0, pos.z),
        );
        systems.gfx.set_bound(
            self.their_status_text,
            Bounds::new(
                self.pos.x + 215.0,
                self.pos.y + 324.0,
                self.pos.x + 385.0,
                self.pos.y + 344.0,
            ),
        );

        let pos = systems.gfx.get_pos(self.money_icon[0]);
        systems.gfx.set_pos(
            self.money_icon[0],
            Vec3::new(self.pos.x + 10.0, self.pos.y + 77.0, pos.z),
        );

        let pos = systems.gfx.get_pos(self.money_icon[1]);
        systems.gfx.set_pos(
            self.money_icon[1],
            Vec3::new(self.pos.x + 210.0, self.pos.y + 77.0, pos.z),
        );

        let pos = Vec2::new(self.pos.x + 32.0, self.pos.y + 77.0);
        self.money_input.set_pos(systems, pos);

        let pos = systems.gfx.get_pos(self.their_money);
        systems.gfx.set_pos(
            self.their_money,
            Vec3::new(self.pos.x + 232.0, self.pos.y + 77.0, pos.z),
        );
        systems.gfx.set_bound(
            self.their_money,
            Bounds::new(
                self.pos.x + 232.0,
                self.pos.y + 77.0,
                self.pos.x + 390.0,
                self.pos.y + 97.0,
            ),
        );

        let pos = systems.gfx.get_pos(self.status_text);
        systems.gfx.set_pos(
            self.status_text,
            Vec3::new(self.pos.x, self.pos.y + 10.0, pos.z),
        );
        systems.gfx.set_bound(
            self.status_text,
            Bounds::new(
                self.pos.x,
                self.pos.y + 10.0,
                self.pos.x + self.size.x,
                self.pos.y + 30.0,
            ),
        );
        systems.gfx.center_text(self.status_text);
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

        if let Some(slot) = self.find_mytrade_slot(screen_pos) {
            let itemindex = self.my_items[slot].item_index;
            itemdesc.set_visible(systems, true);
            itemdesc.set_data(systems, itemindex as usize);
            itemdesc.set_position(systems, screen_pos);
        } else {
            itemdesc.set_visible(systems, false);
        }

        if let Some(slot) = self.find_theirtrade_slot(screen_pos) {
            let itemindex = self.their_items[slot].item_index;
            itemdesc.set_visible(systems, true);
            itemdesc.set_data(systems, itemindex as usize);
            itemdesc.set_position(systems, screen_pos);
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

    pub fn clear_trade_items(&mut self, systems: &mut SystemHolder) {
        for slot in 0..MAX_TRADE_SLOT * 2 {
            let (item_slot, render_slot) = if slot >= MAX_TRADE_SLOT {
                (&mut self.their_items, slot - MAX_TRADE_SLOT)
            } else {
                (&mut self.my_items, slot)
            };
            if item_slot[render_slot].got_data {
                systems.gfx.remove_gfx(
                    &mut systems.renderer,
                    item_slot[render_slot].image,
                );
                if item_slot[render_slot].got_count {
                    systems.gfx.remove_gfx(
                        &mut systems.renderer,
                        item_slot[render_slot].count_bg,
                    );
                    systems.gfx.remove_gfx(
                        &mut systems.renderer,
                        item_slot[render_slot].count,
                    );
                }
                item_slot[render_slot].got_data = false;
                item_slot[render_slot].got_count = false;
                item_slot[render_slot].item_index = 0;
                item_slot[render_slot].count_data = 0;
            }
        }
    }

    pub fn find_mytrade_slot(&mut self, screen_pos: Vec2) -> Option<usize> {
        for slot in 0..MAX_TRADE_SLOT {
            let frame_pos = Vec2::new(
                slot as f32 % MAX_TRADE_X,
                (slot as f32 / MAX_TRADE_X).floor(),
            );
            let slot_pos = Vec2::new(
                self.pos.x + 10.0 + (37.0 * frame_pos.x),
                self.pos.y + 287.0 - (37.0 * frame_pos.y),
            );

            if is_within_area(screen_pos, slot_pos, Vec2::new(32.0, 32.0)) {
                return Some(slot);
            }
        }
        None
    }

    pub fn find_theirtrade_slot(&mut self, screen_pos: Vec2) -> Option<usize> {
        for slot in MAX_TRADE_SLOT..MAX_TRADE_SLOT * 2 {
            let slotindex = slot - MAX_TRADE_SLOT;
            let frame_pos = Vec2::new(
                slotindex as f32 % MAX_TRADE_X,
                (slotindex as f32 / MAX_TRADE_X).floor(),
            );
            let slot_pos = Vec2::new(
                self.pos.x + 210.0 + (37.0 * frame_pos.x),
                self.pos.y + 287.0 - (37.0 * frame_pos.y),
            );

            if is_within_area(screen_pos, slot_pos, Vec2::new(32.0, 32.0)) {
                return Some(slot);
            }
        }
        None
    }

    pub fn update_trade_slot(
        &mut self,
        systems: &mut SystemHolder,
        slot: usize,
        data: &Item,
        same_entity: bool,
    ) {
        if slot >= MAX_TRADE_SLOT {
            return;
        }

        let item_slot = if same_entity {
            &mut self.my_items
        } else {
            &mut self.their_items
        };

        item_slot[slot].need_update = false;

        if item_slot[slot].got_data {
            if item_slot[slot].item_index == data.num as u16
                && item_slot[slot].count_data == data.val
            {
                return;
            }
            systems
                .gfx
                .remove_gfx(&mut systems.renderer, item_slot[slot].image);
            if item_slot[slot].got_count {
                systems.gfx.remove_gfx(
                    &mut systems.renderer,
                    item_slot[slot].count_bg,
                );
                systems
                    .gfx
                    .remove_gfx(&mut systems.renderer, item_slot[slot].count);
            }
            item_slot[slot].got_data = false;
            item_slot[slot].got_count = false;
            item_slot[slot].item_index = 0;
            item_slot[slot].count_data = 0;
        }

        if data.val == 0 {
            return;
        }

        let detail_origin = ORDER_GUI_WINDOW.sub_f32(self.z_order, 3);
        let item_zpos = detail_origin.sub_f32(0.002, 3);
        let textbg_zpos = detail_origin.sub_f32(0.003, 3);
        let text_zpos = detail_origin.sub_f32(0.004, 3);

        let frame_pos = Vec2::new(
            slot as f32 % MAX_TRADE_X,
            (slot as f32 / MAX_TRADE_X).floor(),
        );
        let slot_pos = if same_entity {
            Vec2::new(
                self.pos.x + 10.0 + (37.0 * frame_pos.x),
                self.pos.y + 287.0 - (37.0 * frame_pos.y),
            )
        } else {
            Vec2::new(
                self.pos.x + 210.0 + (37.0 * frame_pos.x),
                self.pos.y + 287.0 - (37.0 * frame_pos.y),
            )
        };

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
        let image_index = systems.gfx.add_image(image, 0, "Trade Item".into());
        systems.gfx.set_visible(image_index, self.visible);

        item_slot[slot].image = image_index;
        item_slot[slot].item_index = data.num as u16;
        item_slot[slot].count_data = data.val;

        if data.val > 1 {
            let mut text_bg = Rect::new(&mut systems.renderer, 0);
            text_bg
                .set_size(Vec2::new(32.0, 16.0))
                .set_position(Vec3::new(slot_pos.x, slot_pos.y, textbg_zpos))
                .set_color(Color::rgba(20, 20, 20, 120))
                .set_border_width(1.0)
                .set_border_color(Color::rgba(50, 50, 50, 180));
            let text_bg_index =
                systems.gfx.add_rect(text_bg, 1, "Trade Item BG".into());
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
            let text_index =
                systems.gfx.add_text(text, 2, "Trade Item Amount".into());
            systems.gfx.set_text(
                &mut systems.renderer,
                text_index,
                &format!("{}", data.val),
            );
            systems.gfx.set_visible(text_index, self.visible);

            item_slot[slot].count = text_index;
            item_slot[slot].count_bg = text_bg_index;
            item_slot[slot].got_count = true;
        }

        item_slot[slot].got_data = true;
    }

    pub fn update_trade_money(
        &mut self,
        systems: &mut SystemHolder,
        amount: u64,
    ) {
        systems.gfx.set_text(
            &mut systems.renderer,
            self.their_money,
            &format!("{}", amount),
        );
    }

    pub fn update_my_status(
        &mut self,
        systems: &mut SystemHolder,
        text: String,
    ) {
        if !self.visible {
            return;
        }

        systems
            .gfx
            .set_text(&mut systems.renderer, self.my_status_text, &text);
    }

    pub fn update_their_status(
        &mut self,
        systems: &mut SystemHolder,
        text: String,
    ) {
        if !self.visible {
            return;
        }

        systems.gfx.set_text(
            &mut systems.renderer,
            self.their_status_text,
            &text,
        );
    }

    pub fn update_status(&mut self, systems: &mut SystemHolder, text: String) {
        if !self.visible {
            return;
        }

        systems
            .gfx
            .set_text(&mut systems.renderer, self.status_text, &text);
        systems.gfx.center_text(self.status_text);
    }
}
