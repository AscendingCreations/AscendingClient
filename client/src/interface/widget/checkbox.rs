use graphics::*;
use cosmic_text::{Attrs, Metrics};

use crate::{
    interface::*,
    DrawSetting,
};

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
    pub pos: Vec3,
    pub size: Vec2,
}

#[derive(Clone)]
pub struct CheckImage {
    pub res: usize,
    pub pos: Vec3,
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
    image: usize,
    check_image: usize,
    box_type: CheckboxType,
    check_type: CheckType,
    text_type: Option<(usize, CheckboxText)>,

    in_hover: bool,
    in_click: bool,
    pub value: bool,

    pub pos: Vec3,
    pub box_size: Vec2,
    pub adjust_x: f32,
}

impl Checkbox {
    pub fn new(
        systems: &mut DrawSetting,
        box_type: CheckboxType,
        check_type: CheckType,
        pos: Vec3,
        box_size: Vec2,
        render_layer: usize,
        text_data: Option<CheckboxText>,
    ) -> Self {
        let boxtype = box_type.clone();
        let checktype = check_type.clone();

        let image = match boxtype {
            CheckboxType::Rect(data) => {
                let mut rect = Rect::new(&mut systems.renderer, 0);
                rect.set_color(data.rect_color)
                    .set_position(pos)
                    .set_size(box_size)
                    .set_radius(data.border_radius);
                if data.got_border {
                    rect.set_border_width(1.0)
                        .set_border_color(data.border_color);
                }
                systems.gfx.add_rect(rect, render_layer)
            }
            CheckboxType::Image(data) => {
                let mut img = Image::new(Some(data.res), &mut systems.renderer, 0);
                img.pos = pos;
                img.hw = box_size;
                img.uv = Vec4::new(0.0, 0.0, box_size.x, box_size.y);
                systems.gfx.add_image(img, render_layer)
            }
        };

        let check_image = match checktype {
            CheckType::SetRect(data) => {
                let mut rect = Rect::new(&mut systems.renderer, 0);
                rect.set_position(Vec3::new(pos.x + data.pos.x, pos.y + data.pos.y, data.pos.z))
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
                let mut img = Image::new(Some(data.res), &mut systems.renderer, 0);
                img.pos = Vec3::new(pos.x + data.pos.x, pos.y + data.pos.y, data.pos.z);
                img.hw = data.size;
                img.uv = Vec4::new(data.uv.x, data.uv.y, data.size.x, data.size.y);
                systems.gfx.add_image(img, render_layer)
            }
        };
        systems.gfx.set_visible(check_image, false);

        let mut adjust_x = 0.0;
        let text_type = if let Some(data) = text_data {
            let data_copy = data.clone();
            let tpos = Vec3::new(pos.x + box_size.x + data.offset_pos.x, pos.y + data.offset_pos.y, pos.z);
            let txt = create_label(systems, 
                tpos,
                data.label_size, 
                Bounds::new(tpos.x, tpos.y, tpos.x + data.label_size.x, tpos.y + data.label_size.y),
                data.color);
            let txt_index = systems.gfx.add_text(txt, data.render_layer);
            systems.gfx.set_text(&mut systems.renderer, txt_index, &data.text);
            adjust_x = data.offset_pos.x + data.label_size.x;
            Some((txt_index, data_copy))
        } else {
            None
        };

        Checkbox {
            image,
            check_image,
            box_type,
            check_type,
            text_type,
            in_hover: false,
            in_click: false,
            value: false,
            pos,
            box_size,
            adjust_x,
        }
    }

    pub fn unload(&mut self, systems: &mut DrawSetting) {
        systems.gfx.remove_gfx(self.image);
        systems.gfx.remove_gfx(self.check_image);
        if let Some(data) = &mut self.text_type {
            systems.gfx.remove_gfx(data.0);
        }
    }

    pub fn set_hover(&mut self, systems: &mut DrawSetting, state: bool) {
        if self.in_hover == state {
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
        if self.in_click == state {
            return;
        }
        self.in_click = state;
        if self.in_click {
            self.value = !self.value;
            systems.gfx.set_visible(self.check_image, self.value);
        }
        
        if self.in_click {
            self.apply_click(systems);
        } else {
            if self.in_hover {
                self.apply_hover(systems);
            } else {
                self.apply_normal(systems);
            }
        }
    }

    fn apply_click(&mut self, systems: &mut DrawSetting) {
        let buttontype = self.box_type.clone();
        match buttontype {
            CheckboxType::Rect(data) => {
                match data.click_change {
                    CheckboxChangeType::ColorChange(color) => { systems.gfx.set_color(self.image, color); }
                    _ => {}
                }
            }
            CheckboxType::Image(data) => {
                match data.click_change {
                    CheckboxChangeType::ImageFrame(frame) => {
                        systems.gfx.set_uv(self.image, 
                            Vec4::new(0.0, self.box_size.y * frame as f32, self.box_size.x, self.box_size.y));
                    }
                    _ => {}
                }
            }
        }

        if let Some(data) = &mut self.text_type {
            let contenttype = data.1.clone();
            match contenttype.click_change {
                CheckboxChangeType::ColorChange(color) => { systems.gfx.set_color(data.0, color); }
                _ => {}
            }
        }
    }

    fn apply_hover(&mut self, systems: &mut DrawSetting) {
        let buttontype = self.box_type.clone();
        match buttontype {
            CheckboxType::Rect(data) => {
                match data.hover_change {
                    CheckboxChangeType::ColorChange(color) => { systems.gfx.set_color(self.image, color); }
                    _ => {}
                }
            }
            CheckboxType::Image(data) => {
                match data.hover_change {
                    CheckboxChangeType::ImageFrame(frame) => {
                        systems.gfx.set_uv(self.image, 
                            Vec4::new(0.0, self.box_size.y * frame as f32, self.box_size.x, self.box_size.y));
                    }
                    _ => {}
                }
            }
        }

        if let Some(data) = &mut self.text_type {
            let contenttype = data.1.clone();
            match contenttype.hover_change {
                CheckboxChangeType::ColorChange(color) => { systems.gfx.set_color(data.0, color); }
                _ => {}
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
                systems.gfx.set_uv(self.image, 
                    Vec4::new(0.0, 0.0, self.box_size.x, self.box_size.y));
            }
        }

        if let Some(data) = &mut self.text_type {
            systems.gfx.set_color(data.0, data.1.color);
        }
    }
}