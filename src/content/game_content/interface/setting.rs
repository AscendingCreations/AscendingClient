use graphics::*;

use crate::{
    data_types::*, is_within_area, logic::*, widget::*, Interface, SystemHolder,
};

pub struct Setting {
    pub visible: bool,
    bg: GfxType,
    header: GfxType,
    header_text: GfxType,
    pub sfx_scroll: Scrollbar,
    pub bgm_scroll: Scrollbar,
    button: Vec<Button>,
    checkbox: Vec<Checkbox>,
    label: Vec<GfxType>,
    sfx_label: GfxType,
    bgm_label: GfxType,

    pub pos: Vec2,
    pub size: Vec2,
    pub z_order: f32,
    pub order_index: usize,
    in_hold: bool,
    hold_pos: Vec2,
    header_pos: Vec2,
    header_size: Vec2,
    pub did_button_click: bool,
    pub did_checkbox_click: bool,

    min_bound: Vec2,
    max_bound: Vec2,
}

impl Setting {
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

        let mut rect = Rect::new(&mut systems.renderer, 0);
        rect.set_position(Vec3::new(w_pos.x - 1.0, w_pos.y - 1.0, w_pos.z))
            .set_size(w_size + 2.0)
            .set_color(Color::rgba(110, 110, 110, 255))
            .set_border_width(1.0)
            .set_border_color(Color::rgba(20, 20, 20, 255));
        let bg = systems.gfx.add_rect(rect, 0, "Settings BG".into(), false);

        let mut header_rect = Rect::new(&mut systems.renderer, 0);
        let header_pos = Vec2::new(
            w_pos.x,
            w_pos.y + (237.0 * systems.scale as f32).floor(),
        );
        let header_size = Vec2::new(orig_size.x, 30.0);
        let header_zpos = detail_1;
        header_rect
            .set_position(Vec3::new(header_pos.x, header_pos.y, header_zpos))
            .set_size((header_size * systems.scale as f32).floor())
            .set_color(Color::rgba(70, 70, 70, 255));
        let header = systems.gfx.add_rect(
            header_rect,
            0,
            "Settings Header".into(),
            false,
        );

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
            systems
                .gfx
                .add_text(text, 1, "Settings Header Text".into(), false);
        systems
            .gfx
            .set_text(&mut systems.renderer, &header_text, "Setting");
        systems.gfx.center_text(&header_text);

        let mut sfx_scroll = Scrollbar::new(
            systems,
            Vec2::new(w_pos.x, w_pos.y),
            Vec2::new(orig_size.x - 110.0, orig_size.y - 90.0),
            100.0,
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
            100,
            20.0,
            false,
            false,
            None,
        );
        sfx_scroll.set_value(systems, systems.config.sfx_volume as usize);

        let mut bgm_scroll = Scrollbar::new(
            systems,
            Vec2::new(w_pos.x, w_pos.y),
            Vec2::new(orig_size.x - 110.0, orig_size.y - 60.0),
            100.0,
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
            100,
            20.0,
            false,
            false,
            None,
        );
        bgm_scroll.set_value(systems, systems.config.bgm_volume as usize);

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

        let mut label = Vec::with_capacity(2);
        for i in 0..2 {
            let (msg, ypos) = match i {
                0 => ("BGM", w_size.y - (60.0 * systems.scale as f32).floor()),
                _ => ("SFX", w_size.y - (90.0 * systems.scale as f32).floor()),
            };
            let tpos = Vec2::new(
                w_pos.x + (10.0 * systems.scale as f32).floor(),
                w_pos.y + ypos,
            );
            let text = create_label(
                systems,
                Vec3::new(tpos.x, tpos.y, detail_1),
                (Vec2::new(100.0, 20.0) * systems.scale as f32).floor(),
                Bounds::new(
                    tpos.x,
                    tpos.y,
                    tpos.x + (100.0 * systems.scale as f32).floor(),
                    tpos.y + (20.0 * systems.scale as f32).floor(),
                ),
                Color::rgba(200, 200, 200, 255),
            );
            let label_index =
                systems
                    .gfx
                    .add_text(text, 1, "Settings Label".into(), false);
            systems
                .gfx
                .set_text(&mut systems.renderer, &label_index, msg);
            label.push(label_index);
        }

        let tpos = Vec3::new(
            w_pos.x + (50.0 * systems.scale as f32).floor(),
            w_pos.y + w_size.y - (60.0 * systems.scale as f32).floor(),
            detail_1,
        );
        let tsize = (Vec2::new(50.0, 20.0) * systems.scale as f32).floor();
        let slabel = create_label(
            systems,
            tpos,
            tsize,
            Bounds::new(tpos.x, tpos.y, tpos.x + tsize.x, tpos.y + tsize.y),
            Color::rgba(200, 200, 200, 255),
        );
        let bgm_label =
            systems
                .gfx
                .add_text(slabel, 1, "Settings BGM Label".into(), false);
        systems.gfx.set_text(
            &mut systems.renderer,
            &bgm_label,
            &format!("{}", systems.config.bgm_volume),
        );

        let tpos = Vec3::new(
            w_pos.x + (50.0 * systems.scale as f32).floor(),
            w_pos.y + w_size.y - (90.0 * systems.scale as f32).floor(),
            detail_1,
        );
        let slabel = create_label(
            systems,
            tpos,
            tsize,
            Bounds::new(tpos.x, tpos.y, tpos.x + tsize.x, tpos.y + tsize.y),
            Color::rgba(200, 200, 200, 255),
        );
        let sfx_label =
            systems
                .gfx
                .add_text(slabel, 1, "Settings SFX Label".into(), false);
        systems.gfx.set_text(
            &mut systems.renderer,
            &sfx_label,
            &format!("{}", systems.config.sfx_volume),
        );

        let mut checkbox = vec![
            Checkbox::new(
                systems,
                CheckboxType::Rect(CheckboxRect {
                    rect_color: Color::rgba(100, 100, 100, 255),
                    got_border: true,
                    border_color: Color::rgba(50, 50, 50, 255),
                    border_radius: 2.0,
                    hover_change: CheckboxChangeType::ColorChange(Color::rgba(
                        140, 140, 140, 255,
                    )),
                    click_change: CheckboxChangeType::ColorChange(Color::rgba(
                        70, 70, 70, 255,
                    )),
                }),
                CheckType::SetRect(CheckRect {
                    rect_color: Color::rgba(200, 200, 200, 255),
                    got_border: false,
                    border_color: Color::rgba(255, 255, 255, 255),
                    border_radius: 2.0,
                    pos: Vec2::new(5.0, 5.0),
                    size: Vec2::new(14.0, 14.0),
                }),
                Vec2::new(w_pos.x, w_pos.y),
                Vec2::new(10.0, orig_size.y - 130.0),
                detail_1,
                (0.0001, 4),
                Vec2::new(24.0, 24.0),
                0,
                Some(CheckboxText {
                    text: "Show FPS?".to_string(),
                    offset_pos: Vec2::new(3.0, 2.0),
                    render_layer: 1,
                    label_size: Vec2::new(180.0, 20.0),
                    color: Color::rgba(200, 200, 200, 255),
                    hover_change: CheckboxChangeType::ColorChange(Color::rgba(
                        240, 240, 240, 255,
                    )),
                    click_change: CheckboxChangeType::ColorChange(Color::rgba(
                        80, 80, 80, 255,
                    )),
                }),
                false,
                None,
            ),
            Checkbox::new(
                systems,
                CheckboxType::Rect(CheckboxRect {
                    rect_color: Color::rgba(100, 100, 100, 255),
                    got_border: true,
                    border_color: Color::rgba(50, 50, 50, 255),
                    border_radius: 2.0,
                    hover_change: CheckboxChangeType::ColorChange(Color::rgba(
                        140, 140, 140, 255,
                    )),
                    click_change: CheckboxChangeType::ColorChange(Color::rgba(
                        70, 70, 70, 255,
                    )),
                }),
                CheckType::SetRect(CheckRect {
                    rect_color: Color::rgba(200, 200, 200, 255),
                    got_border: false,
                    border_color: Color::rgba(255, 255, 255, 255),
                    border_radius: 2.0,
                    pos: Vec2::new(5.0, 5.0),
                    size: Vec2::new(14.0, 14.0),
                }),
                Vec2::new(w_pos.x, w_pos.y),
                Vec2::new(10.0, orig_size.y - 154.0),
                detail_1,
                (0.0001, 4),
                Vec2::new(24.0, 24.0),
                0,
                Some(CheckboxText {
                    text: "Show Ping?".to_string(),
                    offset_pos: Vec2::new(3.0, 2.0),
                    render_layer: 1,
                    label_size: Vec2::new(180.0, 20.0),
                    color: Color::rgba(200, 200, 200, 255),
                    hover_change: CheckboxChangeType::ColorChange(Color::rgba(
                        240, 240, 240, 255,
                    )),
                    click_change: CheckboxChangeType::ColorChange(Color::rgba(
                        80, 80, 80, 255,
                    )),
                }),
                false,
                None,
            ),
            Checkbox::new(
                systems,
                CheckboxType::Rect(CheckboxRect {
                    rect_color: Color::rgba(100, 100, 100, 255),
                    got_border: true,
                    border_color: Color::rgba(50, 50, 50, 255),
                    border_radius: 2.0,
                    hover_change: CheckboxChangeType::ColorChange(Color::rgba(
                        140, 140, 140, 255,
                    )),
                    click_change: CheckboxChangeType::ColorChange(Color::rgba(
                        70, 70, 70, 255,
                    )),
                }),
                CheckType::SetRect(CheckRect {
                    rect_color: Color::rgba(200, 200, 200, 255),
                    got_border: false,
                    border_color: Color::rgba(255, 255, 255, 255),
                    border_radius: 2.0,
                    pos: Vec2::new(5.0, 5.0),
                    size: Vec2::new(14.0, 14.0),
                }),
                Vec2::new(w_pos.x, w_pos.y),
                Vec2::new(10.0, orig_size.y - 178.0),
                detail_1,
                (0.0001, 4),
                Vec2::new(24.0, 24.0),
                0,
                Some(CheckboxText {
                    text: "Show Average Ping?".to_string(),
                    offset_pos: Vec2::new(3.0, 2.0),
                    render_layer: 1,
                    label_size: Vec2::new(180.0, 20.0),
                    color: Color::rgba(200, 200, 200, 255),
                    hover_change: CheckboxChangeType::ColorChange(Color::rgba(
                        240, 240, 240, 255,
                    )),
                    click_change: CheckboxChangeType::ColorChange(Color::rgba(
                        80, 80, 80, 255,
                    )),
                }),
                false,
                None,
            ),
            Checkbox::new(
                systems,
                CheckboxType::Rect(CheckboxRect {
                    rect_color: Color::rgba(100, 100, 100, 255),
                    got_border: true,
                    border_color: Color::rgba(50, 50, 50, 255),
                    border_radius: 2.0,
                    hover_change: CheckboxChangeType::ColorChange(Color::rgba(
                        140, 140, 140, 255,
                    )),
                    click_change: CheckboxChangeType::ColorChange(Color::rgba(
                        70, 70, 70, 255,
                    )),
                }),
                CheckType::SetRect(CheckRect {
                    rect_color: Color::rgba(200, 200, 200, 255),
                    got_border: false,
                    border_color: Color::rgba(255, 255, 255, 255),
                    border_radius: 2.0,
                    pos: Vec2::new(5.0, 5.0),
                    size: Vec2::new(14.0, 14.0),
                }),
                Vec2::new(w_pos.x, w_pos.y),
                Vec2::new(10.0, orig_size.y - 202.0),
                detail_1,
                (0.0001, 4),
                Vec2::new(24.0, 24.0),
                0,
                Some(CheckboxText {
                    text: "Show Frame Jitter?".to_string(),
                    offset_pos: Vec2::new(3.0, 2.0),
                    render_layer: 1,
                    label_size: Vec2::new(180.0, 20.0),
                    color: Color::rgba(200, 200, 200, 255),
                    hover_change: CheckboxChangeType::ColorChange(Color::rgba(
                        240, 240, 240, 255,
                    )),
                    click_change: CheckboxChangeType::ColorChange(Color::rgba(
                        80, 80, 80, 255,
                    )),
                }),
                false,
                None,
            ),
        ];
        checkbox[0].set_value(systems, systems.config.show_fps);
        checkbox[1].set_value(systems, systems.config.show_ping);
        checkbox[2].set_value(systems, systems.config.show_average_ping);
        checkbox[3].set_value(systems, systems.config.show_frame_loop);

        Setting {
            visible: false,
            bg,
            header,
            header_text,
            sfx_scroll,
            bgm_scroll,
            button,
            checkbox,
            label,
            bgm_label,
            sfx_label,

            pos,
            size: w_size,
            z_order: 0.0,
            order_index: 0,
            in_hold: false,
            hold_pos: Vec2::new(0.0, 0.0),
            header_pos,
            header_size,
            did_button_click: false,
            did_checkbox_click: false,

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
        self.sfx_scroll.unload(systems);
        self.bgm_scroll.unload(systems);
        self.button.iter_mut().for_each(|button| {
            button.unload(systems);
        });
        self.checkbox.iter_mut().for_each(|checkbox| {
            checkbox.unload(systems);
        });
        self.label.iter().for_each(|text| {
            systems.gfx.remove_gfx(&mut systems.renderer, text);
        });
        self.button.clear();
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, &self.bgm_label);
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, &self.sfx_label);
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
        self.sfx_scroll.set_visible(systems, visible);
        self.bgm_scroll.set_visible(systems, visible);
        self.button.iter_mut().for_each(|button| {
            button.set_visible(systems, visible);
        });
        self.checkbox.iter_mut().for_each(|checkbox| {
            checkbox.set_visible(systems, visible);
        });
        self.label.iter().for_each(|text| {
            systems.gfx.set_visible(text, visible);
        });
        systems.gfx.set_visible(&self.bgm_label, visible);
        systems.gfx.set_visible(&self.sfx_label, visible);
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

        self.sfx_scroll.set_z_order(systems, detail_1);
        self.bgm_scroll.set_z_order(systems, detail_1);

        self.button.iter_mut().for_each(|button| {
            button.set_z_order(systems, detail_2);
        });

        self.checkbox.iter_mut().for_each(|checkbox| {
            checkbox.set_z_order(systems, detail_1);
        });

        self.label.iter().for_each(|text| {
            let mut pos = systems.gfx.get_pos(text);
            pos.z = detail_1;
            systems.gfx.set_pos(text, pos);
        });

        let mut pos = systems.gfx.get_pos(&self.bgm_label);
        pos.z = detail_1;
        systems.gfx.set_pos(&self.bgm_label, pos);

        let mut pos = systems.gfx.get_pos(&self.sfx_label);
        pos.z = detail_1;
        systems.gfx.set_pos(&self.sfx_label, pos);
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
        self.header_pos = Vec2::new(self.pos.x, self.pos.y + 237.0);
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

        self.checkbox.iter_mut().for_each(|checkbox| {
            checkbox.set_pos(systems, self.pos);
        });

        self.sfx_scroll.set_pos(systems, self.pos);
        self.bgm_scroll.set_pos(systems, self.pos);

        self.label.iter().enumerate().for_each(|(index, text)| {
            let ypos = match index {
                0 => self.size.y - (60.0 * systems.scale as f32).floor(),
                _ => self.size.y - (90.0 * systems.scale as f32).floor(),
            };
            let tpos = Vec2::new(
                self.pos.x + (10.0 * systems.scale as f32).floor(),
                self.pos.y + ypos,
            );

            let pos = systems.gfx.get_pos(text);
            systems.gfx.set_pos(text, Vec3::new(tpos.x, tpos.y, pos.z));
            systems.gfx.set_bound(
                text,
                Bounds::new(
                    tpos.x,
                    tpos.y,
                    tpos.x + (100.0 * systems.scale as f32).floor(),
                    tpos.y + (20.0 * systems.scale as f32).floor(),
                ),
            );
        });

        let tpos = Vec2::new(
            self.pos.x + (50.0 * systems.scale as f32).floor(),
            self.pos.y + self.size.y - (60.0 * systems.scale as f32).floor(),
        );
        let tsize = (Vec2::new(50.0, 20.0) * systems.scale as f32).floor();
        let pos = systems.gfx.get_pos(&self.bgm_label);
        systems
            .gfx
            .set_pos(&self.bgm_label, Vec3::new(tpos.x, tpos.y, pos.z));
        systems.gfx.set_bound(
            &self.bgm_label,
            Bounds::new(tpos.x, tpos.y, tpos.x + tsize.x, tpos.y + tsize.y),
        );

        let tpos = Vec2::new(
            self.pos.x + (50.0 * systems.scale as f32).floor(),
            self.pos.y + self.size.y - (90.0 * systems.scale as f32).floor(),
        );
        let pos = systems.gfx.get_pos(&self.sfx_label);
        systems
            .gfx
            .set_pos(&self.sfx_label, Vec3::new(tpos.x, tpos.y, pos.z));
        systems.gfx.set_bound(
            &self.sfx_label,
            Bounds::new(tpos.x, tpos.y, tpos.x + tsize.x, tpos.y + tsize.y),
        );
    }

    pub fn hover_scrollbar(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
    ) {
        if !self.visible || self.order_index != 0 {
            return;
        }

        if self.sfx_scroll.in_scroll(screen_pos) {
            self.sfx_scroll.set_hover(systems, true);
        } else {
            self.sfx_scroll.set_hover(systems, false);
        }
        if self.bgm_scroll.in_scroll(screen_pos) {
            self.bgm_scroll.set_hover(systems, true);
        } else {
            self.bgm_scroll.set_hover(systems, false);
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

    pub fn hover_checkbox(
        &mut self,
        systems: &mut SystemHolder,
        tooltip: &mut Tooltip,
        screen_pos: Vec2,
    ) {
        for checkbox in self.checkbox.iter_mut() {
            if is_within_area(
                screen_pos,
                Vec2::new(
                    checkbox.base_pos.x
                        + (checkbox.adjust_pos.x * systems.scale as f32)
                            .floor(),
                    checkbox.base_pos.y
                        + (checkbox.adjust_pos.y * systems.scale as f32)
                            .floor(),
                ),
                (Vec2::new(
                    checkbox.box_size.x + checkbox.adjust_x,
                    checkbox.box_size.y,
                ) * systems.scale as f32)
                    .floor(),
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
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
    ) -> Option<usize> {
        let mut checkbox_found = None;
        for (index, checkbox) in self.checkbox.iter_mut().enumerate() {
            if is_within_area(
                screen_pos,
                Vec2::new(
                    checkbox.base_pos.x
                        + (checkbox.adjust_pos.x * systems.scale as f32)
                            .floor(),
                    checkbox.base_pos.y
                        + (checkbox.adjust_pos.y * systems.scale as f32)
                            .floor(),
                ),
                (Vec2::new(
                    checkbox.box_size.x + checkbox.adjust_x,
                    checkbox.box_size.y,
                ) * systems.scale as f32)
                    .floor(),
            ) {
                checkbox.set_click(systems, true);
                checkbox_found = Some(index)
            }
        }
        checkbox_found
    }

    pub fn trigger_checkbox(
        &mut self,
        systems: &mut SystemHolder,
        index: usize,
        ping_index: &GfxType,
        average_ping_index: &GfxType,
        frame_jitter_index: &GfxType,
    ) {
        #[allow(clippy::single_match)]
        match index {
            0 => {
                systems.config.show_fps = self.checkbox[index].value;
                systems
                    .gfx
                    .set_visible(&systems.fps, systems.config.show_fps);
                systems.config.save_config("settings.toml");
            }
            1 => {
                systems.config.show_ping = self.checkbox[index].value;
                systems
                    .gfx
                    .set_visible(ping_index, systems.config.show_ping);
                systems.config.save_config("settings.toml");
            }
            2 => {
                systems.config.show_average_ping = self.checkbox[index].value;
                systems.gfx.set_visible(
                    average_ping_index,
                    systems.config.show_average_ping,
                );
                systems.config.save_config("settings.toml");
            }
            3 => {
                systems.config.show_frame_loop = self.checkbox[index].value;
                systems.gfx.set_visible(
                    frame_jitter_index,
                    systems.config.show_frame_loop,
                );
                systems.config.save_config("settings.toml");
            }
            _ => {}
        }
    }

    pub fn reset_checkbox(&mut self, systems: &mut SystemHolder) {
        if !self.did_checkbox_click {
            return;
        }
        self.did_checkbox_click = false;

        self.checkbox.iter_mut().for_each(|checkbox| {
            checkbox.set_click(systems, false);
        });
    }

    pub fn update_bgm_value(
        &mut self,
        systems: &mut SystemHolder,
        value: usize,
    ) {
        systems.gfx.set_text(
            &mut systems.renderer,
            &self.bgm_label,
            &format!("{}", value),
        );
    }

    pub fn update_sfx_value(
        &mut self,
        systems: &mut SystemHolder,
        value: usize,
    ) {
        systems.gfx.set_text(
            &mut systems.renderer,
            &self.sfx_label,
            &format!("{}", value),
        );
    }
}
