use enum_iterator::next;
use graphics::*;

use crate::{
    gfx_order::*, is_within_area, next_down, widget::*, DrawSetting
};

const MAX_INV_SLOT: usize = 30;
const MAX_INV_X: f32 = 5.0;

pub struct Inventory {
    pub visible: bool,
    bg: usize,
    header: usize,
    header_text: usize,
    slot: [usize; MAX_INV_SLOT],

    pub pos: Vec2,
    pub size: Vec2,
    pub z_order: f32,
    in_hold: bool,
    hold_pos: Vec2,
    header_pos: Vec2,
    header_size: Vec2
}

impl Inventory {
    pub fn new(systems: &mut DrawSetting) -> Self {
        let w_size = Vec2::new(200.0, 267.0);
        let w_pos = Vec3::new(systems.size.width - w_size.x - 10.0, 60.0, ORDER_GUI_WINDOW);
        let pos = Vec2::new(w_pos.x, w_pos.y);

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
        let header_zpos = next_down(w_pos.z);
        header_rect.set_position(Vec3::new(header_pos.x, header_pos.y, header_zpos))
            .set_size(header_size)
            .set_color(Color::rgba(70, 70, 70, 255));
        let header = systems.gfx.add_rect(header_rect, 0);
        systems.gfx.set_visible(header, false);

        let text = create_label(systems, 
            Vec3::new(w_pos.x, w_pos.y + 242.0, next_down(header_zpos)),
            Vec2::new(w_size.x, 20.0),
            Bounds::new(w_pos.x, w_pos.y + 242.0, w_pos.x + w_size.x, w_pos.y + 262.0),
            Color::rgba(200, 200, 200, 255));
        let header_text = systems.gfx.add_text(text, 1);
        systems.gfx.set_text(&mut systems.renderer, header_text, "Inventory");
        systems.gfx.center_text(header_text);
        systems.gfx.set_visible(header_text, false);
        
        let mut slot = [0; MAX_INV_SLOT];
        for i in 0..MAX_INV_SLOT {
            let mut box_rect = Rect::new(&mut systems.renderer, 0);
            let frame_pos = Vec2::new(i as f32 % MAX_INV_X,
                (i  as f32 / MAX_INV_X).floor());
            box_rect.set_position(
                Vec3::new(w_pos.x + 10.0 + (37.0 * frame_pos.x), 
                    w_pos.y + 10.0 + (37.0 * frame_pos.y), 
                    next_down(w_pos.z))
                )
                .set_size(Vec2::new(32.0, 32.0))
                .set_color(Color::rgba(200, 200, 200, 255));
            slot[i] = systems.gfx.add_rect(box_rect, 0);
            systems.gfx.set_visible(slot[i], false);
        }

        Inventory {
            visible: false,
            bg,
            header,
            header_text,
            slot,

            pos,
            size: w_size,
            z_order: 0.0,
            in_hold: false,
            hold_pos: Vec2::new(0.0, 0.0),
            header_pos,
            header_size,
        }
    }

    pub fn unload(&self, systems: &mut DrawSetting) {
        systems.gfx.remove_gfx(self.bg);
        systems.gfx.remove_gfx(self.header);
        systems.gfx.remove_gfx(self.header_text);
        self.slot.iter().for_each(|slot| {
            systems.gfx.remove_gfx(*slot);
        });
    }

    pub fn set_visible(&mut self, systems: &mut DrawSetting, visible: bool) {
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

    pub fn set_z_order(&mut self, systems: &mut DrawSetting, z_order: f32) {
        if self.z_order == z_order {
            return;
        }
        self.z_order = z_order;
        let z_order_result = self.z_order * 0.01;

        let set_pos_z = ORDER_GUI_WINDOW - z_order_result;
        let mut pos = systems.gfx.get_pos(self.bg);
        pos.z = set_pos_z;
        systems.gfx.set_pos(self.bg, pos);

        let mut pos = systems.gfx.get_pos(self.header);
        let header_zpos = next_down(set_pos_z);
        pos.z = header_zpos;
        systems.gfx.set_pos(self.header, pos);

        let mut pos = systems.gfx.get_pos(self.header_text);
        pos.z = next_down(header_zpos);
        systems.gfx.set_pos(self.header_text, pos);

        for i in 0..MAX_INV_SLOT {
            let mut pos = systems.gfx.get_pos(self.slot[i]);
            pos.z = next_down(set_pos_z);
            systems.gfx.set_pos(self.slot[i], pos);
        }
    }

    pub fn move_window(&mut self, systems: &mut DrawSetting, screen_pos: Vec2) {
        if !self.in_hold {
            return;
        }
        self.pos = screen_pos - self.hold_pos;

        let pos = systems.gfx.get_pos(self.bg);
        systems.gfx.set_pos(self.bg, Vec3::new(self.pos.x - 1.0, self.pos.y - 1.0, pos.z));
        let pos = systems.gfx.get_pos(self.header);
        self.header_pos = Vec2::new(self.pos.x, self.pos.y + 237.0);
        systems.gfx.set_pos(self.header, Vec3::new(self.pos.x, self.pos.y + 237.0, pos.z));
        let pos = systems.gfx.get_pos(self.header_text);
        systems.gfx.set_pos(self.header_text, Vec3::new(self.pos.x, self.pos.y + 242.0, pos.z));
        systems.gfx.set_bound(self.header_text,
            Bounds::new(self.pos.x, self.pos.y + 242.0, self.pos.x + self.size.x, self.pos.y + 262.0));
        systems.gfx.center_text(self.header_text);

        for i in 0..MAX_INV_SLOT {
            let frame_pos = Vec2::new(i as f32 % MAX_INV_X,
                (i  as f32 / MAX_INV_X).floor());
            let pos = systems.gfx.get_pos(self.slot[i]);
            systems.gfx.set_pos(self.slot[i], 
                Vec3::new(self.pos.x + 10.0 + (37.0 * frame_pos.x),
                        self.pos.y + 10.0 + (37.0 * frame_pos.y), pos.z));
        }
    }
}