use crate::{is_within_area, logic::*, GfxType, SystemHolder};
use graphics::*;

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
    reverse_value: bool,
    bg: Option<GfxType>,
    scroll: GfxType,

    z_pos: f32,

    base_pos: Vec2,
    adjust_pos: Vec2,
    hold_pos: Vec2,
    pos: Vec2,
    size: Vec2,
    bar_size: f32,
    pub value: usize,
    pub max_value: usize,
    start_pos: usize,
    end_pos: usize,
    length: usize,
    min_bar_size: f32,
    default_color: Color,
    hover_color: Color,
    hold_color: Color,
    in_hover: bool,
    pub in_hold: bool,
    z_step: (f32, i32),
    pub tooltip: Option<String>,
}

impl Scrollbar {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        systems: &mut SystemHolder,
        base_pos: Vec2,
        adjust_pos: Vec2,
        bar_size: f32,
        thickness: f32,
        is_vertical: bool,
        z_pos: f32,
        z_step: (f32, i32),
        scrollbar: ScrollbarRect,
        background: Option<ScrollbarBackground>,
        max_value: usize,
        min_bar_size: f32,
        reverse_value: bool,
        visible: bool,
        tooltip: Option<String>,
    ) -> Self {
        let bg = if let Some(data) = background {
            let mut scrollbg_rect = Rect::new(&mut systems.renderer, 0);
            let pos = base_pos + (adjust_pos * systems.scale as f32).floor();
            let bg_pos = Vec3::new(pos.x - 1.0, pos.y - 1.0, z_pos);
            scrollbg_rect
                .set_position(bg_pos)
                .set_color(data.color)
                .set_radius(data.radius);
            if data.got_border {
                scrollbg_rect
                    .set_border_width(1.0)
                    .set_border_color(data.border_color);
            }
            if is_vertical {
                scrollbg_rect.set_size(
                    (Vec2::new(thickness + 2.0, bar_size + 2.0)
                        * systems.scale as f32)
                        .floor(),
                );
            } else {
                scrollbg_rect.set_size(
                    (Vec2::new(bar_size + 2.0, thickness + 2.0)
                        * systems.scale as f32)
                        .floor(),
                );
            }
            let bg = systems.gfx.add_rect(
                scrollbg_rect,
                data.render_layer,
                "Scrollbar BG".into(),
                visible,
            );
            Some(bg)
        } else {
            None
        };

        let scrollbar_size = ((bar_size - (min_bar_size * max_value as f32))
            .max(min_bar_size)
            * systems.scale as f32)
            .floor();

        let (start_pos, end_pos) = if is_vertical {
            (
                (adjust_pos.y * systems.scale as f32).floor() as usize
                    + ((bar_size * systems.scale as f32).floor() as usize
                        - scrollbar_size as usize),
                (adjust_pos.y * systems.scale as f32).floor() as usize,
            )
        } else {
            (
                (adjust_pos.x * systems.scale as f32).floor() as usize,
                (adjust_pos.x * systems.scale as f32).floor() as usize
                    + ((bar_size * systems.scale as f32).floor() as usize
                        - scrollbar_size as usize),
            )
        };
        let length = if is_vertical {
            start_pos - end_pos
        } else {
            end_pos - start_pos
        };

        let mut scroll_rect = Rect::new(&mut systems.renderer, 0);
        let (pos, size) = if is_vertical {
            (
                Vec2::new(
                    (adjust_pos.x * systems.scale as f32).floor(),
                    start_pos as f32,
                ),
                Vec2::new(
                    (thickness * systems.scale as f32).floor(),
                    scrollbar_size,
                ),
            )
        } else {
            (
                Vec2::new(
                    start_pos as f32,
                    (adjust_pos.y * systems.scale as f32).floor(),
                ),
                Vec2::new(
                    scrollbar_size,
                    (thickness * systems.scale as f32).floor(),
                ),
            )
        };
        scroll_rect
            .set_position(Vec3::new(
                base_pos.x + pos.x,
                base_pos.y + pos.y,
                z_pos.sub_f32(z_step.0, z_step.1),
            ))
            .set_color(scrollbar.color)
            .set_size(size)
            .set_radius(scrollbar.radius);
        if scrollbar.got_border {
            scroll_rect
                .set_border_width(1.0)
                .set_border_color(scrollbar.border_color);
        }
        let scroll = systems.gfx.add_rect(
            scroll_rect,
            scrollbar.render_layer,
            "Scrollbar Scroll".into(),
            visible,
        );

        Scrollbar {
            visible,
            bg,
            scroll,
            z_pos,
            z_step,
            reverse_value,
            is_vertical,
            base_pos,
            adjust_pos,
            hold_pos: Vec2::new(0.0, 0.0),
            pos,
            size,
            bar_size,
            value: 0,
            max_value,
            start_pos,
            end_pos,
            length,
            min_bar_size,
            default_color: scrollbar.color,
            hover_color: scrollbar.hover_color,
            hold_color: scrollbar.hold_color,
            in_hover: false,
            in_hold: false,
            tooltip,
        }
    }

    pub fn unload(&self, systems: &mut SystemHolder) {
        if let Some(index) = self.bg {
            systems.gfx.remove_gfx(&mut systems.renderer, &index);
        }
        systems.gfx.remove_gfx(&mut systems.renderer, &self.scroll);
    }

    pub fn in_scroll(&mut self, screen_pos: Vec2) -> bool {
        is_within_area(screen_pos, self.base_pos + self.pos, self.size)
    }

    pub fn set_hover(&mut self, systems: &mut SystemHolder, in_hover: bool) {
        if self.in_hover == in_hover {
            return;
        }
        self.in_hover = in_hover;
        if self.in_hold {
            return;
        }
        if self.in_hover {
            systems.gfx.set_color(&self.scroll, self.hover_color);
        } else {
            systems.gfx.set_color(&self.scroll, self.default_color);
        }
    }

    pub fn set_move_scroll(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
    ) {
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
        self.value = ((y_pos / self.length as f32) * self.max_value as f32)
            .floor() as usize;

        if self.reverse_value {
            self.value = self.max_value.saturating_sub(self.value);
        }

        let pos = systems.gfx.get_pos(&self.scroll);
        systems.gfx.set_pos(
            &self.scroll,
            Vec3::new(
                self.base_pos.x + self.pos.x,
                self.base_pos.y + self.pos.y,
                pos.z,
            ),
        );
    }

    pub fn set_hold(
        &mut self,
        systems: &mut SystemHolder,
        in_hold: bool,
        screen_pos: Vec2,
    ) {
        if self.in_hold == in_hold {
            return;
        }
        self.in_hold = in_hold;
        if self.in_hold {
            systems.gfx.set_color(&self.scroll, self.default_color);
            self.hold_pos = screen_pos - (self.base_pos + self.pos);
        } else if self.in_hover {
            systems.gfx.set_color(&self.scroll, self.hover_color);
        } else {
            systems.gfx.set_color(&self.scroll, self.default_color);
        }
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        if self.visible == visible {
            return;
        }
        self.visible = visible;
        if let Some(index) = self.bg {
            systems.gfx.set_visible(&index, visible);
        }
        systems.gfx.set_visible(&self.scroll, visible);
    }

    pub fn set_z_order(&mut self, systems: &mut SystemHolder, z_order: f32) {
        self.z_pos = z_order;
        if let Some(index) = self.bg {
            let pos = systems.gfx.get_pos(&index);
            systems
                .gfx
                .set_pos(&index, Vec3::new(pos.x, pos.y, self.z_pos));
        }
        let pos = systems.gfx.get_pos(&self.scroll);
        systems.gfx.set_pos(
            &self.scroll,
            Vec3::new(
                pos.x,
                pos.y,
                self.z_pos.sub_f32(self.z_step.0, self.z_step.1),
            ),
        );
    }

    pub fn set_pos(&mut self, systems: &mut SystemHolder, new_pos: Vec2) {
        self.base_pos = new_pos;
        if let Some(index) = self.bg {
            let pos = systems.gfx.get_pos(&index);
            systems.gfx.set_pos(
                &index,
                Vec3::new(
                    new_pos.x
                        + ((self.adjust_pos.x - 1.0) * systems.scale as f32)
                            .floor(),
                    new_pos.y
                        + ((self.adjust_pos.y - 1.0) * systems.scale as f32)
                            .floor(),
                    pos.z,
                ),
            );
        }
        let pos = systems.gfx.get_pos(&self.scroll);
        systems.gfx.set_pos(
            &self.scroll,
            Vec3::new(new_pos.x + self.pos.x, new_pos.y + self.pos.y, pos.z),
        );
    }

    pub fn set_value(&mut self, systems: &mut SystemHolder, value: usize) {
        let new_value = if self.reverse_value {
            self.max_value.saturating_sub(value)
        } else {
            value
        };
        if new_value > self.max_value {
            return;
        }
        let new_pos = ((new_value as f32 / self.max_value as f32)
            * self.length as f32)
            .floor();
        self.value = value;
        let pos = systems.gfx.get_pos(&self.scroll);
        if self.is_vertical {
            self.pos.y = (self.adjust_pos.y * systems.scale as f32).floor()
                + (self.length as f32 - new_pos);
            systems.gfx.set_pos(
                &self.scroll,
                Vec3::new(
                    self.base_pos.x + self.pos.x,
                    self.base_pos.y + self.pos.y,
                    pos.z,
                ),
            );
        } else {
            self.pos.x =
                (self.adjust_pos.x * systems.scale as f32).floor() + new_pos;
            systems.gfx.set_pos(
                &self.scroll,
                Vec3::new(
                    self.base_pos.x + self.pos.x,
                    self.base_pos.y + self.pos.y,
                    pos.z,
                ),
            );
        }
    }

    pub fn set_max_value(
        &mut self,
        systems: &mut SystemHolder,
        max_value: usize,
    ) {
        if self.max_value == max_value {
            return;
        }
        self.max_value = max_value;

        let scrollbar_size = ((self.bar_size
            - (self.min_bar_size * self.max_value as f32))
            .max(self.min_bar_size)
            * systems.scale as f32)
            .floor();

        (self.start_pos, self.end_pos) = if self.is_vertical {
            (
                (self.adjust_pos.y * systems.scale as f32).floor() as usize
                    + ((self.bar_size * systems.scale as f32).floor() as usize
                        - scrollbar_size as usize),
                (self.adjust_pos.y * systems.scale as f32).floor() as usize,
            )
        } else {
            (
                (self.adjust_pos.x * systems.scale as f32).floor() as usize,
                (self.adjust_pos.x * systems.scale as f32).floor() as usize
                    + ((self.bar_size * systems.scale as f32).floor() as usize
                        - scrollbar_size as usize),
            )
        };
        self.length = if self.is_vertical {
            self.start_pos - self.end_pos
        } else {
            self.end_pos - self.start_pos
        };

        (self.pos, self.size) = if self.is_vertical {
            (
                Vec2::new(
                    (self.adjust_pos.x * systems.scale as f32).floor(),
                    self.start_pos as f32,
                ),
                Vec2::new(self.size.x, scrollbar_size),
            )
        } else {
            (
                Vec2::new(
                    self.start_pos as f32,
                    (self.adjust_pos.y * systems.scale as f32).floor(),
                ),
                Vec2::new(scrollbar_size, self.size.y),
            )
        };
        systems.gfx.set_pos(
            &self.scroll,
            Vec3::new(
                self.base_pos.x + self.pos.x,
                self.base_pos.y + self.pos.y,
                self.z_pos.sub_f32(self.z_step.0, self.z_step.1),
            ),
        );
        systems.gfx.set_size(&self.scroll, self.size);
    }
}
