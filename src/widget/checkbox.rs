use cosmic_text::{Attrs, Metrics};
use graphics::*;

use crate::{logic::*, widget::*, DrawSetting};

#[derive(Clone)]
pub enum CheckboxChangeType {
    None,
    ImageFrame(usize),
    ColorChange(Color),
}

#[derive(Clone)]
pub struct CheckRect {
    pub rect_color: Color,
    pub got_border: bool,
    pub border_color: Color,
    pub border_radius: f32,
    pub pos: Vec2,
    pub size: Vec2,
}

#[derive(Clone)]
pub struct CheckImage {
    pub res: usize,
    pub pos: Vec2,
    pub size: Vec2,
    pub uv: Vec2,
}

#[derive(Clone)]
pub struct CheckboxRect {
    pub rect_color: Color,
    pub got_border: bool,
    pub border_color: Color,
    pub border_radius: f32,
    pub hover_change: CheckboxChangeType,
    pub click_change: CheckboxChangeType,
}

#[derive(Clone)]
pub struct CheckboxImage {
    pub res: usize,
    pub hover_change: CheckboxChangeType,
    pub click_change: CheckboxChangeType,
}

#[derive(Clone)]
pub struct CheckboxText {
    pub text: String,
    pub offset_pos: Vec2,
    pub render_layer: usize,
    pub label_size: Vec2,
    pub color: Color,
    pub hover_change: CheckboxChangeType,
    pub click_change: CheckboxChangeType,
}

#[derive(Clone)]
pub enum CheckType {
    SetRect(CheckRect),
    SetImage(CheckImage),
}

#[derive(Clone)]
pub enum CheckboxType {
    Rect(CheckboxRect),
    Image(CheckboxImage),
}

pub struct Checkbox {
    visible: bool,
    image: usize,
    check_image: usize,
    box_type: CheckboxType,
    check_type: CheckType,
    text_type: Option<(usize, CheckboxText)>,

    in_hover: bool,
    in_click: bool,
    pub value: bool,

    pub base_pos: Vec2,
    pub adjust_pos: Vec2,
    pub z_order: f32,
    pub box_size: Vec2,
    pub adjust_x: f32,
    z_step: (f32, i32),
    pub tooltip: Option<String>,
}

impl Checkbox {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        systems: &mut DrawSetting,
        box_type: CheckboxType,
        check_type: CheckType,
        base_pos: Vec2,
        adjust_pos: Vec2,
        z_order: f32,
        z_step: (f32, i32),
        box_size: Vec2,
        render_layer: usize,
        text_data: Option<CheckboxText>,
        visible: bool,
        tooltip: Option<String>,
    ) -> Self {
        let boxtype = box_type.clone();
        let checktype = check_type.clone();

        let pos = base_pos + adjust_pos;
        let image = match boxtype {
            CheckboxType::Rect(data) => {
                let mut rect = Rect::new(&mut systems.renderer, 0);
                rect.set_color(data.rect_color)
                    .set_position(Vec3::new(pos.x, pos.y, z_order))
                    .set_size(box_size)
                    .set_radius(data.border_radius);
                if data.got_border {
                    rect.set_border_width(1.0)
                        .set_border_color(data.border_color);
                }
                systems.gfx.add_rect(rect, render_layer)
            }
            CheckboxType::Image(data) => {
                let mut img =
                    Image::new(Some(data.res), &mut systems.renderer, 0);
                img.pos = Vec3::new(pos.x, pos.y, z_order);
                img.hw = box_size;
                img.uv = Vec4::new(0.0, 0.0, box_size.x, box_size.y);
                systems.gfx.add_image(img, render_layer)
            }
        };
        systems.gfx.set_visible(image, visible);

        let check_image = match checktype {
            CheckType::SetRect(data) => {
                let mut rect = Rect::new(&mut systems.renderer, 0);
                rect.set_position(Vec3::new(
                    pos.x + data.pos.x,
                    pos.y + data.pos.y,
                    z_order.sub_f32(z_step.0, z_step.1),
                ))
                .set_size(data.size)
                .set_color(data.rect_color)
                .set_radius(data.border_radius);
                if data.got_border {
                    rect.set_border_width(1.0)
                        .set_border_color(data.border_color);
                }
                systems.gfx.add_rect(rect, render_layer)
            }
            CheckType::SetImage(data) => {
                let mut img =
                    Image::new(Some(data.res), &mut systems.renderer, 0);
                img.pos = Vec3::new(
                    pos.x + data.pos.x,
                    pos.y + data.pos.y,
                    z_order.sub_f32(z_step.0, z_step.1),
                );
                img.hw = data.size;
                img.uv =
                    Vec4::new(data.uv.x, data.uv.y, data.size.x, data.size.y);
                systems.gfx.add_image(img, render_layer)
            }
        };
        systems.gfx.set_visible(check_image, false);

        let mut adjust_x = 0.0;
        let text_type = if let Some(data) = text_data {
            let data_copy = data.clone();
            let tpos = Vec3::new(
                pos.x + box_size.x + data.offset_pos.x,
                pos.y + data.offset_pos.y,
                z_order,
            );
            let txt = create_label(
                systems,
                tpos,
                data.label_size,
                Bounds::new(
                    tpos.x,
                    tpos.y,
                    tpos.x + data.label_size.x,
                    tpos.y + data.label_size.y,
                ),
                data.color,
            );
            let txt_index = systems.gfx.add_text(txt, data.render_layer);
            systems
                .gfx
                .set_text(&mut systems.renderer, txt_index, &data.text);
            systems.gfx.set_visible(txt_index, visible);
            adjust_x = data.offset_pos.x + data.label_size.x;
            Some((txt_index, data_copy))
        } else {
            None
        };

        Checkbox {
            visible,
            image,
            check_image,
            box_type,
            check_type,
            text_type,
            in_hover: false,
            in_click: false,
            value: false,
            base_pos,
            adjust_pos,
            z_order,
            z_step,
            box_size,
            adjust_x,
            tooltip,
        }
    }

    pub fn unload(&mut self, systems: &mut DrawSetting) {
        systems.gfx.remove_gfx(self.image);
        systems.gfx.remove_gfx(self.check_image);
        if let Some(data) = &mut self.text_type {
            systems.gfx.remove_gfx(data.0);
        }
    }

    pub fn set_visible(&mut self, systems: &mut DrawSetting, visible: bool) {
        if self.visible == visible {
            return;
        }
        self.visible = visible;
        systems.gfx.set_visible(self.image, visible);
        if visible {
            systems.gfx.set_visible(self.check_image, self.value);
        } else {
            systems.gfx.set_visible(self.check_image, false);
        }
        if let Some(data) = &mut self.text_type {
            systems.gfx.set_visible(data.0, visible);
        }
    }

    pub fn set_z_order(&mut self, systems: &mut DrawSetting, z_order: f32) {
        self.z_order = z_order;
        let pos = systems.gfx.get_pos(self.image);
        systems
            .gfx
            .set_pos(self.image, Vec3::new(pos.x, pos.y, self.z_order));
        let pos = systems.gfx.get_pos(self.check_image);
        systems.gfx.set_pos(
            self.check_image,
            Vec3::new(
                pos.x,
                pos.y,
                self.z_order.sub_f32(self.z_step.0, self.z_step.1),
            ),
        );
        if let Some(data) = &mut self.text_type {
            let pos = systems.gfx.get_pos(data.0);
            systems
                .gfx
                .set_pos(data.0, Vec3::new(pos.x, pos.y, self.z_order));
        }
    }

    pub fn set_pos(&mut self, systems: &mut DrawSetting, new_pos: Vec2) {
        self.base_pos = new_pos;

        let pos = Vec3::new(
            self.base_pos.x + self.adjust_pos.x,
            self.base_pos.y + self.adjust_pos.y,
            self.z_order,
        );
        systems.gfx.set_pos(self.image, pos);

        let contenttype = self.check_type.clone();
        let extra_pos = match contenttype {
            CheckType::SetRect(data) => data.pos,
            CheckType::SetImage(data) => data.pos,
        };
        let pos = Vec3::new(
            self.base_pos.x + self.adjust_pos.x + extra_pos.x,
            self.base_pos.y + self.adjust_pos.y + extra_pos.y,
            self.z_order,
        );
        systems.gfx.set_pos(self.check_image, pos);

        if let Some(data) = &mut self.text_type {
            let pos = Vec3::new(
                self.base_pos.x
                    + self.adjust_pos.x
                    + self.box_size.x
                    + data.1.offset_pos.x,
                self.base_pos.y + self.adjust_pos.y + data.1.offset_pos.y,
                self.z_order,
            );
            systems.gfx.set_pos(data.0, pos);
            systems.gfx.set_bound(
                data.0,
                Bounds::new(
                    pos.x,
                    pos.y,
                    pos.x + data.1.label_size.x,
                    pos.y + data.1.label_size.y,
                ),
            );
        }
    }

    pub fn set_hover(&mut self, systems: &mut DrawSetting, state: bool) {
        if self.in_hover == state || !self.visible {
            return;
        }
        self.in_hover = state;
        if !self.in_click {
            if self.in_hover {
                self.apply_hover(systems);
            } else {
                self.apply_normal(systems);
            }
        }
    }

    pub fn set_click(&mut self, systems: &mut DrawSetting, state: bool) {
        if self.in_click == state || !self.visible {
            return;
        }
        self.in_click = state;
        if self.in_click {
            self.value = !self.value;
            systems.gfx.set_visible(self.check_image, self.value);
        }

        if self.in_click {
            self.apply_click(systems);
        } else if self.in_hover {
            self.apply_hover(systems);
        } else {
            self.apply_normal(systems);
        }
    }

    fn apply_click(&mut self, systems: &mut DrawSetting) {
        let buttontype = self.box_type.clone();
        match buttontype {
            CheckboxType::Rect(data) => {
                if let CheckboxChangeType::ColorChange(color) =
                    data.click_change
                {
                    systems.gfx.set_color(self.image, color);
                }
            }
            CheckboxType::Image(data) => {
                if let CheckboxChangeType::ImageFrame(frame) = data.click_change
                {
                    systems.gfx.set_uv(
                        self.image,
                        Vec4::new(
                            0.0,
                            self.box_size.y * frame as f32,
                            self.box_size.x,
                            self.box_size.y,
                        ),
                    );
                }
            }
        }

        if let Some(data) = &mut self.text_type {
            let contenttype = data.1.clone();
            if let CheckboxChangeType::ColorChange(color) =
                contenttype.click_change
            {
                systems.gfx.set_color(data.0, color);
            }
        }
    }

    fn apply_hover(&mut self, systems: &mut DrawSetting) {
        let buttontype = self.box_type.clone();
        match buttontype {
            CheckboxType::Rect(data) => {
                if let CheckboxChangeType::ColorChange(color) =
                    data.hover_change
                {
                    systems.gfx.set_color(self.image, color);
                }
            }
            CheckboxType::Image(data) => {
                if let CheckboxChangeType::ImageFrame(frame) = data.hover_change
                {
                    systems.gfx.set_uv(
                        self.image,
                        Vec4::new(
                            0.0,
                            self.box_size.y * frame as f32,
                            self.box_size.x,
                            self.box_size.y,
                        ),
                    );
                }
            }
        }

        if let Some(data) = &mut self.text_type {
            let contenttype = data.1.clone();
            if let CheckboxChangeType::ColorChange(color) =
                contenttype.hover_change
            {
                systems.gfx.set_color(data.0, color);
            }
        }
    }

    fn apply_normal(&mut self, systems: &mut DrawSetting) {
        let buttontype = self.box_type.clone();
        match buttontype {
            CheckboxType::Rect(data) => {
                systems.gfx.set_color(self.image, data.rect_color);
            }
            CheckboxType::Image(_) => {
                systems.gfx.set_uv(
                    self.image,
                    Vec4::new(0.0, 0.0, self.box_size.x, self.box_size.y),
                );
            }
        }

        if let Some(data) = &mut self.text_type {
            systems.gfx.set_color(data.0, data.1.color);
        }
    }
}
