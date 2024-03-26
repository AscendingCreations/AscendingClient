use graphics::*;

use crate::{is_within_area, logic::*, values::*, widget::*, SystemHolder};

pub struct Setting {
    pub visible: bool,
    bg: usize,
    header: usize,
    header_text: usize,
    pub sfx_scroll: Scrollbar,
    pub bgm_scroll: Scrollbar,
    button: Vec<Button>,

    pub pos: Vec2,
    pub size: Vec2,
    pub z_order: f32,
    order_index: usize,
    in_hold: bool,
    hold_pos: Vec2,
    header_pos: Vec2,
    header_size: Vec2,
    pub did_button_click: bool,

    min_bound: Vec2,
    max_bound: Vec2,
}

impl Setting {
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
        rect.set_position(Vec3::new(w_pos.x - 1.0, w_pos.y - 1.0, w_pos.z))
            .set_size(w_size + 2.0)
            .set_color(Color::rgba(110, 110, 110, 255))
            .set_border_width(1.0)
            .set_border_color(Color::rgba(20, 20, 20, 255));
        let bg = systems.gfx.add_rect(rect, 0);
        systems.gfx.set_visible(bg, false);

        let mut header_rect = Rect::new(&mut systems.renderer, 0);
        let header_pos = Vec2::new(w_pos.x, w_pos.y + 237.0);
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
            Vec3::new(w_pos.x, w_pos.y + 242.0, detail_2),
            Vec2::new(w_size.x, 20.0),
            Bounds::new(
                w_pos.x,
                w_pos.y + 242.0,
                w_pos.x + w_size.x,
                w_pos.y + 262.0,
            ),
            Color::rgba(200, 200, 200, 255),
        );
        let header_text = systems.gfx.add_text(text, 1);
        systems
            .gfx
            .set_text(&mut systems.renderer, header_text, "Setting");
        systems.gfx.center_text(header_text);
        systems.gfx.set_visible(header_text, false);

        let sfx_scroll = Scrollbar::new(
            systems,
            Vec2::new(w_pos.x, w_pos.y),
            Vec2::new(5.0, 5.0),
            130.0,
            20.0,
            false,
            detail_1,
            (0.0001, 4),
            ScrollbarRect {
                color: Color::rgba(70, 70, 70, 255),
                render_layer: 0,
                got_border: false,
                border_color: Color::rgba(0, 0, 0, 0),
                hover_color: Color::rgba(100, 100, 100, 255),
                hold_color: Color::rgba(40, 40, 40, 255),
                radius: 0.0,
            },
            Some(ScrollbarBackground {
                color: Color::rgba(150, 150, 150, 255),
                render_layer: 0,
                got_border: false,
                border_color: Color::rgba(0, 0, 0, 0),
                radius: 0.0,
            }),
            8,
            20.0,
            false,
            false,
            None,
        );

        let bgm_scroll = Scrollbar::new(
            systems,
            Vec2::new(w_pos.x, w_pos.y),
            Vec2::new(5.0, 35.0),
            130.0,
            20.0,
            false,
            detail_1,
            (0.0001, 4),
            ScrollbarRect {
                color: Color::rgba(70, 70, 70, 255),
                render_layer: 0,
                got_border: false,
                border_color: Color::rgba(0, 0, 0, 0),
                hover_color: Color::rgba(100, 100, 100, 255),
                hold_color: Color::rgba(40, 40, 40, 255),
                radius: 0.0,
            },
            Some(ScrollbarBackground {
                color: Color::rgba(150, 150, 150, 255),
                render_layer: 0,
                got_border: false,
                border_color: Color::rgba(0, 0, 0, 0),
                radius: 0.0,
            }),
            8,
            20.0,
            false,
            false,
            None,
        );

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

        Setting {
            visible: false,
            bg,
            header,
            header_text,
            sfx_scroll,
            bgm_scroll,
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
        }
    }

    pub fn unload(&mut self, systems: &mut SystemHolder) {
        systems.gfx.remove_gfx(self.bg);
        systems.gfx.remove_gfx(self.header);
        systems.gfx.remove_gfx(self.header_text);
        self.sfx_scroll.unload(systems);
        self.bgm_scroll.unload(systems);
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
        self.sfx_scroll.set_visible(systems, visible);
        self.bgm_scroll.set_visible(systems, visible);
        self.button.iter_mut().for_each(|button| {
            button.set_visible(systems, visible);
        })
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

        self.sfx_scroll.set_z_order(systems, detail_1);
        self.bgm_scroll.set_z_order(systems, detail_1);

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

        self.sfx_scroll.set_pos(systems, self.pos);
        self.bgm_scroll.set_pos(systems, self.pos);
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
