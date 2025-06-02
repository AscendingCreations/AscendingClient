use graphics::*;

use crate::{SystemHolder, data_types::*, is_within_area, logic::*, widget::*};

use super::ItemDescription;

pub struct ShopItem {
    got_data: bool,
    icon_bg: GfxType,
    icon: Option<GfxType>,
    name: GfxType,
    price_icon: GfxType,
    price: GfxType,
    got_count: bool,
    amount_bg: GfxType,
    amount: GfxType,
    item_index: usize,
}

pub struct Shop {
    pub visible: bool,
    bg: GfxType,
    header: GfxType,
    header_text: GfxType,
    button: Vec<Button>,
    item: Vec<ShopItem>,
    pub item_scroll: Scrollbar,
    pub shop_start_pos: usize,
    shop_index: usize,

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

impl Shop {
    pub fn new(systems: &mut SystemHolder) -> Self {
        let orig_size = Vec2::new(255.0, 276.0);
        let w_size = (orig_size * systems.scale as f32).floor();
        let w_pos = Vec3::new(
            systems.size.width - w_size.x - 10.0,
            60.0,
            ORDER_GUI_WINDOW,
        );
        let pos = Vec2::new(w_pos.x, w_pos.y);

        let detail_1 = w_pos.z.sub_f32(0.001, 3);
        let detail_2 = w_pos.z.sub_f32(0.002, 3);
        let detail_3 = w_pos.z.sub_f32(0.003, 3);
        let detail_4 = w_pos.z.sub_f32(0.004, 3);

        let mut rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(w_pos.x - 1.0, w_pos.y - 1.0, w_pos.z),
            w_size + 2.0,
            0,
        );
        rect.set_color(Color::rgba(110, 110, 110, 255))
            .set_border_width(1.0)
            .set_border_color(Color::rgba(20, 20, 20, 255));
        let bg = systems.gfx.add_rect(rect, 0, "Shop BG", false);

        let header_pos = Vec2::new(
            w_pos.x,
            w_pos.y + (246.0 * systems.scale as f32).floor(),
        );
        let header_size = Vec2::new(orig_size.x, 30.0);
        let header_zpos = detail_1;
        let mut header_rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(header_pos.x, header_pos.y, header_zpos),
            (header_size * systems.scale as f32).floor(),
            0,
        );

        header_rect.set_color(Color::rgba(70, 70, 70, 255));
        let header = systems.gfx.add_rect(header_rect, 0, "Shop Header", false);

        let text = create_label(
            systems,
            Vec3::new(
                w_pos.x,
                w_pos.y + (251.0 * systems.scale as f32).floor(),
                detail_2,
            ),
            Vec2::new(w_size.x, (20.0 * systems.scale as f32).floor()),
            Bounds::new(
                w_pos.x,
                w_pos.y + (251.0 * systems.scale as f32).floor(),
                w_pos.x + w_size.x,
                w_pos.y + (271.0 * systems.scale as f32).floor(),
            ),
            Color::rgba(200, 200, 200, 255),
        );
        let header_text =
            systems.gfx.add_text(text, 1, "Shop Header Text", false);
        systems
            .gfx
            .set_text(&mut systems.renderer, &header_text, "Shop");
        systems.gfx.center_text(&header_text);

        let mut button = vec![
            Button::new(
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
                Vec2::new(header_size.x - 25.0, 251.0),
                detail_2,
                (0.0001, 4),
                Vec2::new(20.0, 20.0),
                0,
                false,
                None,
            ),
            Button::new(
                systems,
                ButtonType::Rect(ButtonRect {
                    rect_color: Color::rgba(80, 80, 80, 255),
                    got_border: false,
                    border_color: Color::rgba(0, 0, 0, 0),
                    border_radius: 0.0,
                    hover_change: ButtonChangeType::ColorChange(Color::rgba(
                        150, 150, 150, 255,
                    )),
                    click_change: ButtonChangeType::ColorChange(Color::rgba(
                        80, 80, 80, 255,
                    )),
                }),
                ButtonContentType::Image(ButtonContentImg {
                    res: systems.resource.vertical_arrow.allocation,
                    pos: Vec2::new(-2.0, -2.0),
                    uv: Vec2::new(0.0, 0.0),
                    size: Vec2::new(24.0, 24.0),
                    hover_change: ButtonChangeType::None,
                    click_change: ButtonChangeType::None,
                }),
                Vec2::new(w_pos.x, w_pos.y),
                Vec2::new(225.0, 221.0),
                detail_1,
                (0.0001, 5),
                Vec2::new(20.0, 20.0),
                0,
                false,
                None,
            ),
            Button::new(
                systems,
                ButtonType::Rect(ButtonRect {
                    rect_color: Color::rgba(80, 80, 80, 255),
                    got_border: false,
                    border_color: Color::rgba(0, 0, 0, 0),
                    border_radius: 0.0,
                    hover_change: ButtonChangeType::ColorChange(Color::rgba(
                        150, 150, 150, 255,
                    )),
                    click_change: ButtonChangeType::ColorChange(Color::rgba(
                        80, 80, 80, 255,
                    )),
                }),
                ButtonContentType::Image(ButtonContentImg {
                    res: systems.resource.vertical_arrow.allocation,
                    pos: Vec2::new(-2.0, -2.0),
                    uv: Vec2::new(24.0, 0.0),
                    size: Vec2::new(24.0, 24.0),
                    hover_change: ButtonChangeType::None,
                    click_change: ButtonChangeType::None,
                }),
                Vec2::new(w_pos.x, w_pos.y),
                Vec2::new(225.0, 9.0),
                detail_1,
                (0.0001, 5),
                Vec2::new(20.0, 20.0),
                0,
                false,
                None,
            ),
        ];

        let mut item = Vec::with_capacity(5);
        for i in 0..5 {
            let pos = Vec2::new(
                w_pos.x + (10.0 * systems.scale as f32).floor(),
                w_pos.y
                    + ((203.0 - (i as f32 * 48.0)) * systems.scale as f32)
                        .floor(),
            );
            let mut bg = Rect::new(
                &mut systems.renderer,
                Vec3::new(pos.x, pos.y, detail_1),
                (Vec2::new(32.0, 32.0) * systems.scale as f32).floor(),
                0,
            );
            bg.set_color(Color::rgba(200, 200, 200, 255));
            let icon_bg = systems.gfx.add_rect(bg, 0, "Shop Item BG", false);

            let pos = Vec2::new(
                w_pos.x + (48.0 * systems.scale as f32).floor(),
                w_pos.y
                    + ((220.0 - (i as f32 * 48.0)) * systems.scale as f32)
                        .floor(),
            );
            let item_name = create_label(
                systems,
                Vec3::new(pos.x, pos.y, detail_1),
                (Vec2::new(114.0, 20.0) * systems.scale as f32).floor(),
                Bounds::new(
                    pos.x,
                    pos.y,
                    pos.x + (114.0 * systems.scale as f32).floor(),
                    pos.y + (20.0 * systems.scale as f32).floor(),
                ),
                Color::rgba(200, 200, 200, 255),
            );
            let name =
                systems.gfx.add_text(item_name, 1, "Shop Item Name", false);
            systems.gfx.set_text(&mut systems.renderer, &name, "");

            let pos = Vec2::new(
                w_pos.x + (72.0 * systems.scale as f32).floor(),
                w_pos.y
                    + ((198.0 - (i as f32 * 48.0)) * systems.scale as f32)
                        .floor(),
            );
            let price_text = create_label(
                systems,
                Vec3::new(pos.x, pos.y, detail_1),
                (Vec2::new(90.0, 20.0) * systems.scale as f32).floor(),
                Bounds::new(
                    pos.x,
                    pos.y,
                    pos.x + (90.0 * systems.scale as f32).floor(),
                    pos.y + (20.0 * systems.scale as f32).floor(),
                ),
                Color::rgba(200, 200, 200, 255),
            );
            let price =
                systems
                    .gfx
                    .add_text(price_text, 1, "Shop Item Price", false);
            systems.gfx.set_text(&mut systems.renderer, &price, "");

            let p_icon = Image::new(
                Some(systems.resource.shop_currency_icon.allocation),
                &mut systems.renderer,
                Vec3::new(
                    w_pos.x + (48.0 * systems.scale as f32).floor(),
                    w_pos.y
                        + ((198.0 - (i as f32 * 48.0)) * systems.scale as f32)
                            .floor(),
                    detail_1,
                ),
                (Vec2::new(20.0, 20.0) * systems.scale as f32).floor(),
                Vec4::new(0.0, 0.0, 20.0, 20.0),
                0,
            );

            let price_icon =
                systems.gfx.add_image(p_icon, 0, "Shop Price Icon", false);

            let buy_button = Button::new(
                systems,
                ButtonType::Rect(ButtonRect {
                    rect_color: Color::rgba(70, 70, 70, 255),
                    got_border: true,
                    border_color: Color::rgba(20, 20, 20, 255),
                    border_radius: 0.0,
                    hover_change: ButtonChangeType::ColorChange(Color::rgba(
                        50, 50, 50, 255,
                    )),
                    click_change: ButtonChangeType::ColorChange(Color::rgba(
                        150, 150, 150, 255,
                    )),
                }),
                ButtonContentType::Text(ButtonContentText {
                    text: "Buy".into(),
                    pos: Vec2::new(0.0, 3.0),
                    color: Color::rgba(200, 200, 200, 255),
                    render_layer: 1,
                    hover_change: ButtonChangeType::None,
                    click_change: ButtonChangeType::None,
                }),
                Vec2::new(w_pos.x, w_pos.y),
                Vec2::new(167.0, 205.0 - (i as f32 * 48.0)),
                detail_1,
                (0.0001, 4),
                Vec2::new(51.0, 26.0),
                0,
                false,
                None,
            );
            button.push(buy_button);

            let pos = Vec2::new(
                w_pos.x + (10.0 * systems.scale as f32).floor(),
                w_pos.y
                    + ((203.0 - (i as f32 * 48.0)) * systems.scale as f32)
                        .floor(),
            );
            let mut amount_bg_rect = Rect::new(
                &mut systems.renderer,
                Vec3::new(pos.x, pos.y, detail_3),
                (Vec2::new(32.0, 16.0) * systems.scale as f32).floor(),
                0,
            );
            amount_bg_rect
                .set_color(Color::rgba(20, 20, 20, 120))
                .set_border_width(1.0)
                .set_border_color(Color::rgba(50, 50, 50, 180));
            let amount_bg = systems.gfx.add_rect(
                amount_bg_rect,
                1,
                "Shop Item Amount BG",
                false,
            );

            let text_size =
                (Vec2::new(32.0, 16.0) * systems.scale as f32).floor();
            let text = create_label(
                systems,
                Vec3::new(
                    pos.x + (2.0 * systems.scale as f32).floor(),
                    pos.y + (2.0 * systems.scale as f32).floor(),
                    detail_4,
                ),
                text_size,
                Bounds::new(
                    pos.x,
                    pos.y,
                    pos.x + text_size.x,
                    pos.y + text_size.y,
                ),
                Color::rgba(240, 240, 240, 255),
            );
            let amount =
                systems.gfx.add_text(text, 2, "Shop Item Amount", false);

            item.push(ShopItem {
                got_data: false,
                icon_bg,
                icon: None,
                name,
                price_icon,
                price,
                got_count: false,
                amount_bg,
                amount,
                item_index: 0,
            });
        }

        let item_scroll = Scrollbar::new(
            systems,
            Vec2::new(w_pos.x, w_pos.y),
            Vec2::new(226.0, 32.0),
            186.0,
            18.0,
            true,
            detail_1,
            (0.0001, 4),
            ScrollbarRect {
                color: Color::rgba(190, 190, 190, 255),
                render_layer: 0,
                got_border: true,
                border_color: Color::rgba(50, 50, 50, 255),
                hover_color: Color::rgba(240, 240, 240, 255),
                hold_color: Color::rgba(50, 50, 50, 255),
                radius: 5.0,
            },
            Some(ScrollbarBackground {
                color: Color::rgba(80, 80, 80, 255),
                render_layer: 0,
                got_border: false,
                border_color: Color::rgba(0, 0, 0, 255),
                radius: 0.0,
            }),
            0,
            30.0,
            false,
            false,
            None,
        );

        Shop {
            visible: false,
            bg,
            header,
            header_text,
            button,
            item,
            item_scroll,
            shop_start_pos: 0,
            shop_index: 0,

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
        self.item.iter_mut().for_each(|item| {
            if let Some(sprite) = item.icon {
                systems.gfx.remove_gfx(&mut systems.renderer, &sprite)
            }
            systems.gfx.remove_gfx(&mut systems.renderer, &item.icon_bg);
            systems.gfx.remove_gfx(&mut systems.renderer, &item.name);
            systems
                .gfx
                .remove_gfx(&mut systems.renderer, &item.price_icon);
            systems.gfx.remove_gfx(&mut systems.renderer, &item.price);
            systems
                .gfx
                .remove_gfx(&mut systems.renderer, &item.amount_bg);
            systems.gfx.remove_gfx(&mut systems.renderer, &item.amount);
        });
        self.item_scroll.unload(systems);
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
        self.button
            .iter_mut()
            .enumerate()
            .for_each(|(index, button)| match index {
                0..=2 => button.set_visible(systems, visible),
                3..=7 => {
                    let itemindex = index.saturating_sub(3);
                    let set_visible = if visible {
                        self.item[itemindex].got_data
                    } else {
                        visible
                    };
                    button.set_visible(systems, set_visible);
                }
                _ => {}
            });
        self.item.iter_mut().for_each(|item| {
            if item.got_data {
                systems.gfx.set_visible(&item.icon_bg, visible);
                if let Some(item_sprite) = item.icon {
                    systems.gfx.set_visible(&item_sprite, visible);
                }
                systems.gfx.set_visible(&item.name, visible);
                systems.gfx.set_visible(&item.price_icon, visible);
                systems.gfx.set_visible(&item.price, visible);
                if item.got_count {
                    systems.gfx.set_visible(&item.amount_bg, visible);
                    systems.gfx.set_visible(&item.amount, visible);
                }
            }
        });
        self.item_scroll.set_visible(systems, visible);
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
        let detail_3 = detail_origin.sub_f32(0.003, 3);
        let detail_4 = detail_origin.sub_f32(0.004, 3);

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

        self.item.iter_mut().for_each(|item| {
            let mut pos = systems.gfx.get_pos(&item.icon_bg);
            pos.z = detail_1;
            systems.gfx.set_pos(&item.icon_bg, pos);

            if let Some(item_sprite) = item.icon {
                let mut pos = systems.gfx.get_pos(&item_sprite);
                pos.z = detail_2;
                systems.gfx.set_pos(&item_sprite, pos);
            }

            let mut pos = systems.gfx.get_pos(&item.name);
            pos.z = detail_1;
            systems.gfx.set_pos(&item.name, pos);

            let mut pos = systems.gfx.get_pos(&item.price_icon);
            pos.z = detail_1;
            systems.gfx.set_pos(&item.price_icon, pos);

            let mut pos = systems.gfx.get_pos(&item.price);
            pos.z = detail_1;
            systems.gfx.set_pos(&item.price, pos);

            let mut pos = systems.gfx.get_pos(&item.amount_bg);
            pos.z = detail_3;
            systems.gfx.set_pos(&item.amount_bg, pos);

            let mut pos = systems.gfx.get_pos(&item.amount);
            pos.z = detail_4;
            systems.gfx.set_pos(&item.amount, pos);
        });

        self.item_scroll.set_z_order(systems, detail_1);
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
            self.pos.y + (246.0 * systems.scale as f32).floor(),
        );
        systems.gfx.set_pos(
            &self.header,
            Vec3::new(
                self.pos.x,
                self.pos.y + (246.0 * systems.scale as f32).floor(),
                pos.z,
            ),
        );
        let pos = systems.gfx.get_pos(&self.header_text);
        systems.gfx.set_pos(
            &self.header_text,
            Vec3::new(
                self.pos.x,
                self.pos.y + (251.0 * systems.scale as f32).floor(),
                pos.z,
            ),
        );
        systems.gfx.set_bound(
            &self.header_text,
            Bounds::new(
                self.pos.x,
                self.pos.y + (251.0 * systems.scale as f32).floor(),
                self.pos.x + self.size.x,
                self.pos.y + (271.0 * systems.scale as f32).floor(),
            ),
        );
        systems.gfx.center_text(&self.header_text);

        self.button.iter_mut().for_each(|button| {
            button.set_pos(systems, self.pos);
        });

        self.item_scroll.set_pos(systems, self.pos);

        for i in 0..5 {
            let pos = systems.gfx.get_pos(&self.item[i].icon_bg);
            systems.gfx.set_pos(
                &self.item[i].icon_bg,
                Vec3::new(
                    self.pos.x + (10.0 * systems.scale as f32).floor(),
                    self.pos.y
                        + ((203.0 - (i as f32 * 48.0)) * systems.scale as f32)
                            .floor(),
                    pos.z,
                ),
            );

            if let Some(item_sprite) = self.item[i].icon {
                let pos = systems.gfx.get_pos(&item_sprite);
                systems.gfx.set_pos(
                    &item_sprite,
                    Vec3::new(
                        self.pos.x + (16.0 * systems.scale as f32).floor(),
                        self.pos.y
                            + ((209.0 - (i as f32 * 48.0))
                                * systems.scale as f32)
                                .floor(),
                        pos.z,
                    ),
                );
            }

            let set_pos = Vec2::new(
                self.pos.x + (48.0 * systems.scale as f32).floor(),
                self.pos.y
                    + ((220.0 - (i as f32 * 48.0)) * systems.scale as f32)
                        .floor(),
            );
            let pos = systems.gfx.get_pos(&self.item[i].name);
            systems.gfx.set_pos(
                &self.item[i].name,
                Vec3::new(set_pos.x, set_pos.y, pos.z),
            );
            systems.gfx.set_bound(
                &self.item[i].name,
                Bounds::new(
                    set_pos.x,
                    set_pos.y,
                    set_pos.x + (114.0 * systems.scale as f32).floor(),
                    set_pos.y + (20.0 * systems.scale as f32).floor(),
                ),
            );

            let set_pos = Vec2::new(
                self.pos.x + (72.0 * systems.scale as f32).floor(),
                self.pos.y
                    + ((198.0 - (i as f32 * 48.0)) * systems.scale as f32)
                        .floor(),
            );
            let pos = systems.gfx.get_pos(&self.item[i].price);
            systems.gfx.set_pos(
                &self.item[i].price,
                Vec3::new(set_pos.x, set_pos.y, pos.z),
            );
            systems.gfx.set_bound(
                &self.item[i].price,
                Bounds::new(
                    set_pos.x,
                    set_pos.y,
                    set_pos.x + (90.0 * systems.scale as f32).floor(),
                    set_pos.y + (20.0 * systems.scale as f32).floor(),
                ),
            );

            let pos = systems.gfx.get_pos(&self.item[i].price_icon);
            systems.gfx.set_pos(
                &self.item[i].price_icon,
                Vec3::new(
                    self.pos.x + (48.0 * systems.scale as f32).floor(),
                    self.pos.y
                        + ((198.0 - (i as f32 * 48.0)) * systems.scale as f32)
                            .floor(),
                    pos.z,
                ),
            );

            let pos = systems.gfx.get_pos(&self.item[i].amount_bg);
            systems.gfx.set_pos(
                &self.item[i].amount_bg,
                Vec3::new(
                    self.pos.x + (10.0 * systems.scale as f32).floor(),
                    self.pos.y
                        + ((203.0 - (i as f32 * 48.0)) * systems.scale as f32)
                            .floor(),
                    pos.z,
                ),
            );

            let set_pos = Vec2::new(
                self.pos.x + (10.0 * systems.scale as f32).floor(),
                self.pos.y
                    + ((203.0 - (i as f32 * 48.0)) * systems.scale as f32)
                        .floor(),
            );
            let pos = systems.gfx.get_pos(&self.item[i].amount);
            systems.gfx.set_pos(
                &self.item[i].amount,
                Vec3::new(
                    set_pos.x + (2.0 * systems.scale as f32).floor(),
                    set_pos.y + (2.0 * systems.scale as f32).floor(),
                    pos.z,
                ),
            );
            systems.gfx.set_bound(
                &self.item[i].amount,
                Bounds::new(
                    set_pos.x,
                    set_pos.y,
                    set_pos.x + (32.0 * systems.scale as f32).floor(),
                    set_pos.y + (16.0 * systems.scale as f32).floor(),
                ),
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

        let mut got_item = None;
        for i in 0..5 {
            let pos = Vec2::new(
                self.pos.x + (10.0 * systems.scale as f32).floor(),
                self.pos.y
                    + ((203.0 - (i as f32 * 48.0)) * systems.scale as f32)
                        .floor(),
            );

            if is_within_area(
                screen_pos,
                pos,
                (Vec2::new(32.0, 32.0) * systems.scale as f32).floor(),
            ) {
                got_item = Some(i);
            }
        }

        if let Some(slot) = got_item {
            itemdesc.set_visible(systems, true);
            itemdesc.set_data(systems, self.item[slot].item_index);
            itemdesc.set_position(systems, screen_pos);
        } else {
            itemdesc.set_visible(systems, false);
        }
    }

    pub fn hover_scrollbar(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
    ) {
        if !self.visible || self.order_index != 0 {
            return;
        }

        if self.item_scroll.in_scroll(screen_pos) {
            self.item_scroll.set_hover(systems, true);
        } else {
            self.item_scroll.set_hover(systems, false);
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

    pub fn set_shop(&mut self, systems: &mut SystemHolder, shop_index: usize) {
        let shopdata = systems.base.shop[shop_index].clone();

        let shop_max_item = shopdata.max_item as usize;

        self.shop_index = shop_index;

        self.shop_start_pos = 0;
        self.item_scroll
            .set_max_value(systems, shop_max_item.saturating_sub(5));

        self.item.iter_mut().for_each(|item| {
            item.got_data = false;
            item.got_count = false;
            systems.gfx.set_visible(&item.amount, false);
            systems.gfx.set_visible(&item.amount_bg, false);
            if let Some(item_sprite) = item.icon {
                systems.gfx.remove_gfx(&mut systems.renderer, &item_sprite);
            }
        });

        let detail_origin = ORDER_GUI_WINDOW.sub_f32(self.z_order, 3);
        let item_zpos = detail_origin.sub_f32(0.002, 3);

        let max_item = shop_max_item.min(5);
        (0..max_item).for_each(|index| {
            let item_data =
                systems.base.item[shopdata.item[index].index as usize].clone();

            self.item[index].got_data = true;
            self.button[3 + index].set_visible(systems, self.visible);
            systems
                .gfx
                .set_visible(&self.item[index].icon_bg, self.visible);
            systems
                .gfx
                .set_visible(&self.item[index].name, self.visible);
            systems
                .gfx
                .set_visible(&self.item[index].price_icon, self.visible);
            systems
                .gfx
                .set_visible(&self.item[index].price, self.visible);

            systems.gfx.set_text(
                &mut systems.renderer,
                &self.item[index].name,
                &item_data.name,
            );
            systems.gfx.set_text(
                &mut systems.renderer,
                &self.item[index].price,
                &format!("{}", shopdata.item[index].price),
            );

            if shopdata.item[index].amount > 1 {
                self.item[index].got_count = true;

                systems.gfx.set_text(
                    &mut systems.renderer,
                    &self.item[index].amount,
                    &format!("{}", shopdata.item[index].amount),
                );

                systems
                    .gfx
                    .set_visible(&self.item[index].amount, self.visible);
                systems
                    .gfx
                    .set_visible(&self.item[index].amount_bg, self.visible);
            }

            let item_pic = item_data.sprite;
            let item_sprite = Image::new(
                Some(systems.resource.items[item_pic as usize].allocation),
                &mut systems.renderer,
                Vec3::new(
                    self.pos.x + (16.0 * systems.scale as f32).floor(),
                    self.pos.y
                        + ((209.0 - (index as f32 * 48.0))
                            * systems.scale as f32)
                            .floor(),
                    item_zpos,
                ),
                (Vec2::new(20.0, 20.0) * systems.scale as f32).floor(),
                Vec4::new(0.0, 0.0, 20.0, 20.0),
                0,
            );

            let item_index = systems.gfx.add_image(
                item_sprite,
                0,
                "Shop Item",
                self.visible,
            );
            self.item[index].icon = Some(item_index);
            self.item[index].item_index = shopdata.item[index].index as usize;
        });
    }

    pub fn set_shop_scroll_value(&mut self, systems: &mut SystemHolder) {
        if self.item_scroll.max_value == 0 {
            return;
        }

        let shopdata = systems.base.shop[self.shop_index].clone();

        let detail_origin = ORDER_GUI_WINDOW.sub_f32(self.z_order, 3);
        let item_zpos = detail_origin.sub_f32(0.002, 3);

        self.shop_start_pos = self.item_scroll.value;
        (self.shop_start_pos..self.shop_start_pos + 5).for_each(|index| {
            let item_data =
                systems.base.item[shopdata.item[index].index as usize].clone();

            let default_index = index - self.shop_start_pos;

            self.item[default_index].item_index =
                shopdata.item[index].index as usize;

            if let Some(sprite_icon) = self.item[default_index].icon {
                systems.gfx.remove_gfx(&mut systems.renderer, &sprite_icon);
            }

            systems.gfx.set_text(
                &mut systems.renderer,
                &self.item[default_index].name,
                &item_data.name,
            );
            systems.gfx.set_text(
                &mut systems.renderer,
                &self.item[default_index].price,
                &format!("{}", shopdata.item[index].price),
            );

            if shopdata.item[index].amount > 1 {
                self.item[default_index].got_count = true;

                systems.gfx.set_text(
                    &mut systems.renderer,
                    &self.item[default_index].amount,
                    &format!("{}", shopdata.item[index].amount),
                );

                systems.gfx.set_visible(
                    &self.item[default_index].amount,
                    self.visible,
                );
                systems.gfx.set_visible(
                    &self.item[default_index].amount_bg,
                    self.visible,
                );
            } else {
                self.item[default_index].got_count = false;
                systems
                    .gfx
                    .set_visible(&self.item[default_index].amount, false);
                systems
                    .gfx
                    .set_visible(&self.item[default_index].amount_bg, false);
            }

            let item_pic = item_data.sprite;
            let item_sprite = Image::new(
                Some(systems.resource.items[item_pic as usize].allocation),
                &mut systems.renderer,
                Vec3::new(
                    self.pos.x + (16.0 * systems.scale as f32).floor(),
                    self.pos.y
                        + ((209.0 - (default_index as f32 * 48.0))
                            * systems.scale as f32)
                            .floor(),
                    item_zpos,
                ),
                (Vec2::new(20.0, 20.0) * systems.scale as f32).floor(),
                Vec4::new(0.0, 0.0, 20.0, 20.0),
                0,
            );

            let item_index = systems.gfx.add_image(
                item_sprite,
                0,
                "Shop Item",
                self.visible,
            );
            self.item[default_index].icon = Some(item_index);
        })
    }
}
