use graphics::*;
use guillotiere::euclid::num::Floor;

use crate::{
    is_within_area,
    DrawSetting,
    logic::*,
};

pub struct ScrollbarBackground {
    pub color: Color,
    pub render_layer: usize,
    pub got_border: bool,
    pub border_color: Color,
    pub radius: f32,
}

pub struct ScrollbarRect {
    pub color: Color,
    pub render_layer: usize,
    pub got_border: bool,
    pub border_color: Color,
    pub hover_color: Color,
    pub hold_color: Color,
    pub radius: f32,
}

pub struct Scrollbar {
    pub visible: bool,
    is_vertical: bool,
    bg: Option<usize>,
    scroll: usize,

    z_pos: f32,

    base_pos: Vec2,
    adjust_pos: Vec2,
    hold_pos: Vec2,
    pos: Vec2,
    size: Vec2,
    pub value: usize,
    max_value: usize,
    start_pos: usize,
    end_pos: usize,
    length: usize,
    default_color: Color,
    hover_color: Color,
    hold_color: Color,
    in_hover: bool,
    pub in_hold: bool,
}

impl Scrollbar {
    pub fn new(
        systems: &mut DrawSetting,
        base_pos: Vec2,
        adjust_pos: Vec2,
        size: f32,
        thickness: f32,
        is_vertical: bool,
        z_pos: f32,
        scrollbar: ScrollbarRect,
        background: Option<ScrollbarBackground>,
        max_value: usize,
        min_bar_size: f32,
        visible: bool,
    ) -> Self {
        let bg = if let Some(data) = background {
            let mut scrollbg_rect = Rect::new(&mut systems.renderer, 0);
            let pos = base_pos + adjust_pos;
            let bg_pos = Vec3::new(pos.x - 1.0, pos.y - 1.0, z_pos);
            scrollbg_rect.set_position(bg_pos)
                .set_color(data.color)
                .set_radius(data.radius);
            if data.got_border {
                scrollbg_rect.set_border_width(1.0)
                    .set_border_color(data.border_color);
            }
            if is_vertical {
                scrollbg_rect.set_size(Vec2::new(thickness + 2.0, size + 2.0));
            } else {
                scrollbg_rect.set_size(Vec2::new(size + 2.0, thickness + 2.0));
            }
            let bg = systems.gfx.add_rect(scrollbg_rect, data.render_layer);
            systems.gfx.set_visible(bg, visible);
            Some(bg)
        } else {
            None
        };

        let scrollbar_size = 
            (size / (max_value as f32 + 1.0)).floor().max(min_bar_size);
        
        let (start_pos, end_pos) = if is_vertical {
            (adjust_pos.y as usize + (size as usize - scrollbar_size as usize),
                adjust_pos.y as usize)
        } else {
            (adjust_pos.x as usize,
                adjust_pos.x as usize + (size as usize - scrollbar_size as usize))
        };
        let length =  if is_vertical { start_pos - end_pos } else { end_pos - start_pos };

        let mut scroll_rect = Rect::new(&mut systems.renderer, 0);
        let (pos, size) = if is_vertical {
            (Vec2::new(adjust_pos.x, start_pos as f32),
                Vec2::new(thickness, scrollbar_size))
        } else {
            (Vec2::new(start_pos as f32, adjust_pos.y),
                Vec2::new(scrollbar_size, thickness))
        };
        scroll_rect.set_position(
                Vec3::new(base_pos.x + pos.x, 
                    base_pos.y + pos.y, 
                    next_down(z_pos)))
            .set_color(scrollbar.color)
            .set_size(size)
            .set_radius(scrollbar.radius);
        if scrollbar.got_border {
            scroll_rect.set_border_width(1.0)
                .set_border_color(scrollbar.border_color);
        }
        let scroll = systems.gfx.add_rect(scroll_rect, scrollbar.render_layer);
        systems.gfx.set_visible(scroll, visible);
        
        Scrollbar {
            visible,
            bg,
            scroll,
            z_pos,
            is_vertical,
            base_pos,
            adjust_pos,
            hold_pos: Vec2::new(0.0, 0.0),
            pos,
            size,
            value: 0,
            max_value,
            start_pos,
            end_pos,
            length,
            default_color: scrollbar.color,
            hover_color: scrollbar.hover_color,
            hold_color: scrollbar.hold_color,
            in_hover: false,
            in_hold: false,
        }
    }

    pub fn unload(&self, systems: &mut DrawSetting) {
        if let Some(index) = self.bg {
            systems.gfx.remove_gfx(index);
        }
        systems.gfx.remove_gfx(self.scroll);
    }

    pub fn in_scroll(&mut self, screen_pos: Vec2) -> bool {
        is_within_area(screen_pos, self.base_pos + self.pos, self.size)
    }

    pub fn set_hover(&mut self, systems: &mut DrawSetting, in_hover: bool) {
        if self.in_hover == in_hover {
            return;
        }
        self.in_hover = in_hover;
        if self.in_hold {
            return;
        }
        if self.in_hover {
            systems.gfx.set_color(self.scroll, self.hover_color);
        } else {
            systems.gfx.set_color(self.scroll, self.default_color);
        }
    }

    pub fn set_move_scroll(&mut self, systems: &mut DrawSetting, screen_pos: Vec2) {
        if !self.in_hold {
            return;
        }
        let y_pos = if self.is_vertical {
            let new_pos = ((screen_pos.y - self.base_pos.y) - self.hold_pos.y)
                .clamp(self.end_pos as f32, self.start_pos as f32);
            self.pos.y = new_pos;
            self.start_pos as f32 - new_pos
        } else {
            let new_pos = ((screen_pos.x - self.base_pos.x) - self.hold_pos.x)
                .clamp(self.start_pos as f32, self.end_pos as f32);
            self.pos.x = new_pos;
            new_pos - self.start_pos as f32
        };
        self.value = ((y_pos / self.length as f32)
                * self.max_value as f32)
                .floor() as usize;
        let pos = systems.gfx.get_pos(self.scroll);
        systems.gfx.set_pos(self.scroll, Vec3::new(self.base_pos.x + self.pos.x, self.base_pos.y + self.pos.y, pos.z));
    }

    pub fn set_hold(&mut self, systems: &mut DrawSetting, in_hold: bool, screen_pos: Vec2) {
        if self.in_hold == in_hold {
            return;
        }
        self.in_hold = in_hold;
        if self.in_hold {
            systems.gfx.set_color(self.scroll, self.default_color);
            self.hold_pos = screen_pos - (self.base_pos + self.pos);
        } else {
            if self.in_hover {
                systems.gfx.set_color(self.scroll, self.hover_color);
            } else {
                systems.gfx.set_color(self.scroll, self.default_color);
            }
        }
    }

    pub fn set_visible(&mut self, systems: &mut DrawSetting, visible: bool) {
        if self.visible == visible {
            return;
        }
        self.visible = visible;
        if let Some(index) = self.bg {
            systems.gfx.set_visible(index, visible);
        }
        systems.gfx.set_visible(self.scroll, visible);
    }

    pub fn set_z_order(&mut self, systems: &mut DrawSetting, z_order: f32) {
        self.z_pos = z_order;
        if let Some(index) = self.bg {
            let pos = systems.gfx.get_pos(index);
            systems.gfx.set_pos(index, Vec3::new(pos.x, pos.y, self.z_pos));
        }
        let pos = systems.gfx.get_pos(self.scroll);
        systems.gfx.set_pos(self.scroll, Vec3::new(pos.x, pos.y, next_down(self.z_pos)));
    }

    pub fn set_pos(&mut self, systems: &mut DrawSetting, new_pos: Vec2) {
        self.base_pos = new_pos;
        if let Some(index) = self.bg {
            let pos = systems.gfx.get_pos(index);
            systems.gfx.set_pos(index, 
                Vec3::new(new_pos.x + self.adjust_pos.x - 1.0,
                        new_pos.y + self.adjust_pos.y - 1.0,
                        pos.z));
        }
        let pos = systems.gfx.get_pos(self.scroll);
        systems.gfx.set_pos(self.scroll, Vec3::new(new_pos.x + self.pos.x, new_pos.y + self.pos.y, pos.z));
    }
}