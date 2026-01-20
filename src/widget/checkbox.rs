use std::default;

use cosmic_text::{Attrs, Metrics};
use graphics::*;

use crate::{GfxType, SystemHolder, logic::*, widget::*};

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

#[derive(Clone, Default)]
pub enum CheckType {
    #[default]
    Empty,
    SetRect(CheckRect),
    SetImage(CheckImage),
}

#[derive(Clone, Default)]
pub enum CheckboxType {
    #[default]
    Empty,
    Rect(CheckboxRect),
    Image(CheckboxImage),
}

#[derive(Default)]
pub struct Checkbox {
    visible: bool,
    image: GfxType,
    check_image: GfxType,
    box_type: CheckboxType,
    check_type: CheckType,
    text_type: Option<(GfxType, CheckboxText)>,

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
        systems: &mut SystemHolder,
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
        let pos = base_pos + (adjust_pos * systems.scale as f32);
        let image = match &box_type {
            CheckboxType::Rect(data) => {
                let mut rect = Rect::new(
                    &mut systems.renderer,
                    Vec3::new(pos.x, pos.y, z_order),
                    box_size * systems.scale as f32,
                    data.rect_color,
                    0,
                );
                rect.set_radius(data.border_radius);

                if data.got_border {
                    rect.set_border_width(1.0)
                        .set_border_color(data.border_color);
                }

                systems.gfx.add_rect(
                    rect,
                    render_layer,
                    "Checkbox Image",
                    visible,
                    CameraView::SubView1,
                )
            }
            CheckboxType::Image(data) => {
                let img = Image::new(
                    Some(data.res),
                    &mut systems.renderer,
                    Vec3::new(pos.x, pos.y, z_order),
                    box_size * systems.scale as f32,
                    Vec4::new(
                        0.0,
                        0.0,
                        box_size.x * systems.scale as f32,
                        box_size.y * systems.scale as f32,
                    ),
                    0,
                );

                systems.gfx.add_image(
                    img,
                    render_layer,
                    "Checkbox Image",
                    visible,
                    CameraView::SubView1,
                )
            }
            _ => GfxType::default(),
        };
        let check_image = match &check_type {
            CheckType::SetRect(data) => {
                let mut rect = Rect::new(
                    &mut systems.renderer,
                    Vec3::new(
                        pos.x + (data.pos.x * systems.scale as f32),
                        pos.y + (data.pos.y * systems.scale as f32),
                        z_order.sub_f32(z_step.0, z_step.1),
                    ),
                    data.size * systems.scale as f32,
                    data.rect_color,
                    0,
                );
                rect.set_radius(data.border_radius);

                if data.got_border {
                    rect.set_border_width(1.0)
                        .set_border_color(data.border_color);
                }

                systems.gfx.add_rect(
                    rect,
                    render_layer,
                    "Checkbox Check",
                    false,
                    CameraView::SubView1,
                )
            }
            CheckType::SetImage(data) => {
                let img = Image::new(
                    Some(data.res),
                    &mut systems.renderer,
                    Vec3::new(
                        pos.x + (data.pos.x * systems.scale as f32),
                        pos.y + (data.pos.y * systems.scale as f32),
                        z_order.sub_f32(z_step.0, z_step.1),
                    ),
                    data.size * systems.scale as f32,
                    Vec4::new(data.uv.x, data.uv.y, data.size.x, data.size.y),
                    0,
                );

                systems.gfx.add_image(
                    img,
                    render_layer,
                    "Checkbox Check",
                    false,
                    CameraView::SubView1,
                )
            }
            _ => GfxType::default(),
        };
        let mut adjust_x = 0.0;
        let text_type = if let Some(data) = &text_data {
            let tpos = Vec3::new(
                pos.x
                    + ((box_size.x + data.offset_pos.x) * systems.scale as f32),
                pos.y + (data.offset_pos.y * systems.scale as f32),
                z_order,
            );
            let txt = create_label(
                systems,
                tpos,
                data.label_size * systems.scale as f32,
                Some(Bounds::new(
                    tpos.x,
                    tpos.y,
                    tpos.x + (data.label_size.x * systems.scale as f32),
                    tpos.y + (data.label_size.y * systems.scale as f32),
                )),
                data.color,
            );
            let txt_index = systems.gfx.add_text(
                txt,
                data.render_layer,
                "Checkbox Text",
                visible,
                CameraView::SubView1,
            );
            systems
                .gfx
                .set_text(&mut systems.renderer, &txt_index, &data.text);
            adjust_x = data.offset_pos.x + data.label_size.x;
            Some(txt_index)
        } else {
            None
        };

        Checkbox {
            visible,
            image,
            check_image,
            box_type,
            check_type,
            text_type: text_type.zip(text_data),
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

    pub fn unload(&mut self, systems: &mut SystemHolder) {
        systems.gfx.remove_gfx(&mut systems.renderer, &self.image);
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, &self.check_image);

        if let Some(data) = &mut self.text_type {
            systems.gfx.remove_gfx(&mut systems.renderer, &data.0);
        }
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        if self.visible != visible {
            self.visible = visible;
            systems.gfx.set_visible(&self.image, visible);

            if visible {
                systems.gfx.set_visible(&self.check_image, self.value);
            } else {
                systems.gfx.set_visible(&self.check_image, false);
            }

            if let Some(data) = &mut self.text_type {
                systems.gfx.set_visible(&data.0, visible);
            }
        }
    }

    pub fn set_z_order(&mut self, systems: &mut SystemHolder, z_order: f32) {
        self.z_order = z_order;
        let pos = systems.gfx.get_pos(&self.image);

        systems
            .gfx
            .set_pos(&self.image, Vec3::new(pos.x, pos.y, self.z_order));

        let pos = systems.gfx.get_pos(&self.check_image);

        systems.gfx.set_pos(
            &self.check_image,
            Vec3::new(
                pos.x,
                pos.y,
                self.z_order.sub_f32(self.z_step.0, self.z_step.1),
            ),
        );

        if let Some(data) = &mut self.text_type {
            let pos = systems.gfx.get_pos(&data.0);
            systems
                .gfx
                .set_pos(&data.0, Vec3::new(pos.x, pos.y, self.z_order));
        }
    }

    pub fn set_pos(&mut self, systems: &mut SystemHolder, new_pos: Vec2) {
        self.base_pos = new_pos;

        let pos = Vec3::new(
            self.base_pos.x + (self.adjust_pos.x * systems.scale as f32),
            self.base_pos.y + (self.adjust_pos.y * systems.scale as f32),
            self.z_order,
        );

        systems.gfx.set_pos(&self.image, pos);

        let extra_pos = match &self.check_type {
            CheckType::SetRect(data) => data.pos * systems.scale as f32,
            CheckType::SetImage(data) => data.pos * systems.scale as f32,
            _ => Vec2::ZERO,
        };
        let pos = Vec3::new(
            self.base_pos.x
                + (self.adjust_pos.x * systems.scale as f32)
                + extra_pos.x,
            self.base_pos.y
                + (self.adjust_pos.y * systems.scale as f32)
                + extra_pos.y,
            self.z_order,
        );

        systems.gfx.set_pos(&self.check_image, pos);

        if let Some(data) = &mut self.text_type {
            let pos = Vec3::new(
                self.base_pos.x
                    + ((self.adjust_pos.x
                        + self.box_size.x
                        + data.1.offset_pos.x)
                        * systems.scale as f32),
                self.base_pos.y
                    + ((self.adjust_pos.y + data.1.offset_pos.y)
                        * systems.scale as f32),
                self.z_order,
            );
            systems.gfx.set_pos(&data.0, pos);
            systems.gfx.set_bound(
                &data.0,
                Some(Bounds::new(
                    pos.x,
                    pos.y,
                    pos.x + (data.1.label_size.x * systems.scale as f32),
                    pos.y + (data.1.label_size.y * systems.scale as f32),
                )),
            );
        }
    }

    pub fn set_hover(&mut self, systems: &mut SystemHolder, state: bool) {
        if self.in_hover != state && self.visible {
            self.in_hover = state;

            if !self.in_click {
                if self.in_hover {
                    self.apply_hover(systems);
                } else {
                    self.apply_normal(systems);
                }
            }
        }
    }

    pub fn set_click(&mut self, systems: &mut SystemHolder, state: bool) {
        if self.in_click != state && self.visible {
            self.in_click = state;

            if self.in_click {
                self.set_value(systems, !self.value);
            }

            if self.in_click {
                self.apply_click(systems);
            } else if self.in_hover {
                self.apply_hover(systems);
            } else {
                self.apply_normal(systems);
            }
        }
    }

    pub fn set_value(&mut self, systems: &mut SystemHolder, value: bool) {
        if self.value != value {
            self.value = value;

            if self.visible {
                systems.gfx.set_visible(&self.check_image, self.value);
            }
        }
    }

    fn apply_click(&mut self, systems: &mut SystemHolder) {
        use crate::{CheckboxChangeType::*, CheckboxType::*};

        match &self.box_type {
            Rect(data) => {
                if let ColorChange(color) = data.click_change {
                    systems.gfx.set_color(&self.image, color);
                }
            }
            Image(data) => {
                if let ImageFrame(frame) = data.click_change {
                    systems.gfx.set_uv(
                        &self.image,
                        Vec4::new(
                            0.0,
                            self.box_size.y * frame as f32,
                            self.box_size.x,
                            self.box_size.y,
                        ),
                    );
                }
            }
            _ => {}
        }

        if let Some(data) = &mut self.text_type
            && let ColorChange(color) = data.1.click_change
        {
            systems.gfx.set_color(&data.0, color);
        }
    }

    fn apply_hover(&mut self, systems: &mut SystemHolder) {
        use crate::{CheckboxChangeType::*, CheckboxType::*};

        match &self.box_type {
            Rect(data) => {
                if let ColorChange(color) = data.hover_change {
                    systems.gfx.set_color(&self.image, color);
                }
            }
            Image(data) => {
                if let ImageFrame(frame) = data.hover_change {
                    systems.gfx.set_uv(
                        &self.image,
                        Vec4::new(
                            0.0,
                            self.box_size.y * frame as f32,
                            self.box_size.x,
                            self.box_size.y,
                        ),
                    );
                }
            }
            _ => {}
        }

        if let Some(data) = &mut self.text_type
            && let ColorChange(color) = data.1.hover_change
        {
            systems.gfx.set_color(&data.0, color);
        }
    }

    fn apply_normal(&mut self, systems: &mut SystemHolder) {
        match &self.box_type {
            CheckboxType::Rect(data) => {
                systems.gfx.set_color(&self.image, data.rect_color);
            }
            CheckboxType::Image(_) => {
                systems.gfx.set_uv(
                    &self.image,
                    Vec4::new(0.0, 0.0, self.box_size.x, self.box_size.y),
                );
            }
            _ => {}
        }

        if let Some(data) = &mut self.text_type {
            systems.gfx.set_color(&data.0, data.1.color);
        }
    }
}
