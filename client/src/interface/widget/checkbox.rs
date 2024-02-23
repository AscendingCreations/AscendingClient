use graphics::*;
use cosmic_text::{Attrs, Metrics};

use crate::{
    interface::*,
    DrawSetting,
};

#[derive(Clone)]
pub enum CheckType {
    RectColor(Color),
    SetImage(usize, // Resource Image
        Vec3, // Position
        Vec2, // Size
        Vec2, // UV
    ),
}

#[derive(Clone)]
pub enum CheckboxType {
    Rect(Color, // Default Color
        bool, // Got Border
        Color, // Border Color
        CheckType),
    Image(usize, // Resource image
        CheckType),
}

pub struct CheckboxText {
    text: String,
    render_layer: usize,
    label_size: Vec2,
    offset: usize,
    color: Color,
}

pub struct Checkbox {
    image: usize,
    check_image: Option<usize>,
    text: Option<usize>,

    checkbox_type: CheckboxType,
}

impl Checkbox {
    pub fn new(
        systems: &mut DrawSetting,
        checkbox_type: CheckboxType,
        pos: Vec3,
        box_size: Vec2,
        render_layer: usize,
        text_data: Option<CheckboxText>,
    ) -> Self {
        let checkboxtype = checkbox_type.clone();
        let checktype: CheckType;
        let image = match checkboxtype {
            CheckboxType::Rect(default_color, got_border, border_color, ctype) => {
                let mut rect = Rect::new(&mut systems.renderer, 0);
                rect.set_color(default_color)
                    .set_position(pos)
                    .set_size(box_size);
                if got_border {
                    rect.set_border_width(1.0)
                        .set_border_color(border_color);
                }
                checktype = ctype;
                systems.gfx.add_rect(rect, render_layer)
            }
            CheckboxType::Image(res, ctype) => {
                let mut img = Image::new(Some(res), &mut systems.renderer, 0);
                img.pos = pos;
                img.hw = box_size;
                img.uv = Vec4::new(0.0, 0.0, box_size.x, box_size.y);
                checktype = ctype;
                systems.gfx.add_image(img, render_layer)
            }
        };
        let check_image = match checktype {
            CheckType::SetImage(res, cpos, csize, cuv) => {
                let mut img = Image::new(Some(res), &mut systems.renderer, 0);
                img.pos = Vec3::new(pos.x + cpos.x, pos.y + cpos.y, cpos.z);
                img.hw = csize;
                img.uv = Vec4::new(cuv.x, cuv.y, csize.x, csize.y);
                Some(systems.gfx.add_image(img, render_layer))
            }
            _ => None,
        };

        let text = if let Some(data) = text_data {
            let tpos = Vec3::new(pos.x + box_size.x + data.offset as f32, pos.y, pos.z);
            let txt = create_label(systems, 
                tpos,
                data.label_size, 
                Bounds::new(tpos.x, tpos.y, tpos.x + data.label_size.x, tpos.y + data.label_size.y),
                data.color);
            let txt_index = systems.gfx.add_text(txt, data.render_layer);
            systems.gfx.set_text(&mut systems.renderer, txt_index, &data.text);
            Some(txt_index)
        } else {
            None
        };

        Checkbox {
            image,
            check_image,
            text,
            checkbox_type,
        }
    }

    pub fn unload(&mut self, systems: &mut DrawSetting) {
        systems.gfx.remove_gfx(self.image);
        if let Some(index) = self.check_image {
            systems.gfx.remove_gfx(index);
        }
        if let Some(index) = self.text {
            systems.gfx.remove_gfx(index);
        }
    }
}