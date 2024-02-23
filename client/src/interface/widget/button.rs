use graphics::*;
use cosmic_text::{Attrs, Metrics};

use crate::{
    interface::*,
    DrawSetting,
};

#[derive(Clone)]
pub enum ButtonChangeType {
    None,
    ImageFrame(usize),
    ColorChange(Color),
    AdjustY(usize),
}

#[derive(Clone)]
pub enum ButtonType {
    Rect(Color, // Default Rect Color
        bool, // Got Border
        Color, // Border Color
        ButtonChangeType, // Hover Change
        ButtonChangeType // Click Change
    ),
    Image(usize,
        ButtonChangeType, // Hover Change
        ButtonChangeType // Click Change
    ),
}

#[derive(Clone)]
pub enum ButtonContentType {
    None,
    Image(usize, // Resource
        Vec3, // Pos to Button
        Vec2, // Size
        ButtonChangeType, // Hover Change
        ButtonChangeType // Click Change
    ),
    Text(String, // Msg
        Vec3, // Pos to Button
        Color, // Default Text Color
        usize, // Text Render Layer
        ButtonChangeType, // Hover Change
        ButtonChangeType // Click Change
    ),
}

pub struct Button {
    index: usize,
    content: Option<usize>,
    in_hover: bool,
    in_click: bool,

    button_type: ButtonType,
    content_type: ButtonContentType,

    pub pos: Vec3,
    pub size: Vec2,
}

impl Button {
    pub fn new(
        systems: &mut DrawSetting,
        button_type: ButtonType,
        content_type: ButtonContentType,
        pos: Vec3,
        size: Vec2,
        render_layer: usize,
    ) -> Self {
        let buttontype = button_type.clone();
        let index = match buttontype {
            ButtonType::Rect(color, got_border, border_color, _, _) => {
                let mut rect = Rect::new(&mut systems.renderer, 0);
                rect.set_position(pos)
                    .set_size(size)
                    .set_color(color);
                if got_border {
                    rect.set_border_width(1.0)
                        .set_border_color(border_color);
                }
                systems.gfx.add_rect(rect, render_layer)
            }
            ButtonType::Image(res, _, _) => {
                let mut image = Image::new(Some(res), &mut systems.renderer, 0);
                image.pos = pos;
                image.hw = size;
                image.uv = Vec4::new(0.0, 0.0, size.x, size.y);
                systems.gfx.add_image(image, render_layer)
            }
        };

        let contenttype = content_type.clone();
        let content = match contenttype {
            ButtonContentType::None => None,
            ButtonContentType::Image(res, cpos, csize, _, _) => {
                let mut image = Image::new(Some(res), &mut systems.renderer, 0);
                image.pos = Vec3::new(pos.x + cpos.x, pos.y + cpos.y, cpos.z);
                image.hw = csize;
                image.uv = Vec4::new(0.0, 0.0, csize.x, csize.y);
                Some(systems.gfx.add_image(image, render_layer))
            }
            ButtonContentType::Text(msg, tpos, tcolor, text_layer, _, _) => {
                let text_pos = Vec2::new(pos.x + tpos.x, pos.y + tpos.y);
                let text = create_label(
                    systems, 
                    Vec3::new(text_pos.x, text_pos.y, tpos.z), 
                    Vec2::new(size.x, 20.0), 
                    Bounds::new(text_pos.x, text_pos.y, text_pos.x + size.x, text_pos.y + 20.0), 
                    tcolor);
                let index = systems.gfx.add_text(text, text_layer);
                systems.gfx.set_text(&mut systems.renderer, index, &msg);
                systems.gfx.center_text(index);
                Some(index)
            }
        };

        Button {
            index,
            content,
            in_hover: false,
            in_click: false,
            button_type,
            content_type,
            pos,
            size,
        }
    }

    pub fn unload(&mut self, systems: &mut DrawSetting) {
        systems.gfx.remove_gfx(self.index);
        if let Some(content_index) = self.content {
            systems.gfx.remove_gfx(content_index);
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
        let buttontype = self.button_type.clone();

        match buttontype {
            ButtonType::Rect(_, _, _, _, click_change) => {
                match click_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(self.index, 
                            Vec3::new(self.pos.x, self.pos.y + adjusty as f32, self.pos.z));
                    }
                    ButtonChangeType::ColorChange(color) => { systems.gfx.set_color(self.index, color); }
                    _ => {}
                }
            }
            ButtonType::Image(_, _, click_change) => {
                match click_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(self.index, 
                            Vec3::new(self.pos.x, self.pos.y + adjusty as f32, self.pos.z));
                    }
                    ButtonChangeType::ImageFrame(frame) => {
                        systems.gfx.set_uv(self.index, 
                            Vec4::new(0.0, self.size.y * frame as f32, self.size.x, self.size.y));
                    }
                    _ => {}
                }
            }
        }

        let contenttype = self.content_type.clone();
        if let Some(content_data) = self.content {
            match contenttype {
                ButtonContentType::Text(_, pos, _, _, _, click_change) => {
                    match click_change {
                        ButtonChangeType::AdjustY(adjusty) => {
                            systems.gfx.set_pos(content_data, 
                                Vec3::new(self.pos.x + pos.x, self.pos.y + pos.y + adjusty as f32, pos.z));
                        }
                        ButtonChangeType::ColorChange(color) => { systems.gfx.set_color(content_data, color); }
                        _ => {}
                    }
                }
                ButtonContentType::Image(_, pos, size, _, click_change) => {
                    match click_change {
                        ButtonChangeType::AdjustY(adjusty) => {
                            systems.gfx.set_pos(content_data, 
                                Vec3::new(self.pos.x + pos.x, self.pos.y + pos.y + adjusty as f32, pos.z));
                        }
                        ButtonChangeType::ImageFrame(frame) => {
                            systems.gfx.set_uv(content_data, 
                                Vec4::new(0.0, size.y * frame as f32, size.x, size.y));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    fn apply_hover(&mut self, systems: &mut DrawSetting) {
        let buttontype = self.button_type.clone();
        match buttontype {
            ButtonType::Rect(_, _, _, hover_change, _) => {
                match hover_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(self.index, 
                            Vec3::new(self.pos.x, self.pos.y + adjusty as f32, self.pos.z));
                    }
                    ButtonChangeType::ColorChange(color) => { systems.gfx.set_color(self.index, color); }
                    _ => {}
                }
            }
            ButtonType::Image(_, hover_change, _) => {
                match hover_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(self.index, 
                            Vec3::new(self.pos.x, self.pos.y + adjusty as f32, self.pos.z));
                    }
                    ButtonChangeType::ImageFrame(frame) => {
                        systems.gfx.set_uv(self.index, 
                            Vec4::new(0.0, self.size.y * frame as f32, self.size.x, self.size.y));
                    }
                    _ => {}
                }
            }
        }

        let contenttype = self.content_type.clone();
        if let Some(content_data) = self.content {
            match contenttype {
                ButtonContentType::Text(_, pos, _, _, hover_change, _) => {
                    match hover_change {
                        ButtonChangeType::AdjustY(adjusty) => {
                            systems.gfx.set_pos(content_data, 
                                Vec3::new(self.pos.x + pos.x, self.pos.y + pos.y + adjusty as f32, pos.z));
                        }
                        ButtonChangeType::ColorChange(color) => {
                            systems.gfx.set_color(content_data, color);
                        }
                        _ => {}
                    }
                }
                ButtonContentType::Image(_, pos, size, hover_change, _) => {
                    match hover_change {
                        ButtonChangeType::AdjustY(adjusty) => {
                            systems.gfx.set_pos(content_data, 
                                Vec3::new(self.pos.x + pos.x, self.pos.y + pos.y + adjusty as f32, pos.z));
                        }
                        ButtonChangeType::ImageFrame(frame) => {
                            systems.gfx.set_uv(content_data, 
                                Vec4::new(0.0, size.y * frame as f32, size.x, size.y));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    fn apply_normal(&mut self, systems: &mut DrawSetting) {
        let buttontype = self.button_type.clone();
        systems.gfx.set_pos(self.index, self.pos);
        match buttontype {
            ButtonType::Rect(color, _, _, _, _) => {
                systems.gfx.set_color(self.index, color);
            }
            ButtonType::Image(_, _, _) => {
                systems.gfx.set_uv(self.index, 
                    Vec4::new(0.0, 0.0, self.size.x, self.size.y));
            }
        }

        let contenttype = self.content_type.clone();
        if let Some(content_data) = self.content {
            match contenttype {
                ButtonContentType::Text(_, pos, color, _, _, _) => {
                    systems.gfx.set_pos(content_data,
                        Vec3::new(self.pos.x + pos.x, self.pos.y + pos.y, pos.z));
                    systems.gfx.set_color(content_data, color);
                    systems.gfx.center_text(content_data);
                }
                ButtonContentType::Image(_, pos, size, _, _) => {
                    systems.gfx.set_pos(content_data,
                        Vec3::new(self.pos.x + pos.x, self.pos.y + pos.y, pos.z));
                    systems.gfx.set_uv(content_data, 
                        Vec4::new(0.0, 0.0, size.x, size.y));
                }
                _ => {}
            }
        }
    }
}