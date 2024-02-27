use graphics::*;
use cosmic_text::{Attrs, Metrics};

use crate::{
    widget::*,
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
pub struct ButtonRect {
    pub rect_color: Color,
    pub got_border: bool,
    pub border_color: Color,
    pub border_radius: f32,
    pub hover_change: ButtonChangeType,
    pub click_change: ButtonChangeType,
}

#[derive(Clone)]
pub struct ButtonImage {
    pub res: usize,
    pub hover_change: ButtonChangeType,
    pub click_change: ButtonChangeType,
}

#[derive(Clone)]
pub struct ButtonContentImg {
    pub res: usize,
    pub pos: Vec3,
    pub uv: Vec2,
    pub size: Vec2,
    pub hover_change: ButtonChangeType,
    pub click_change: ButtonChangeType,
}

#[derive(Clone)]
pub struct ButtonContentText {
    pub text: String,
    pub pos: Vec3,
    pub color: Color,
    pub render_layer: usize,
    pub hover_change: ButtonChangeType,
    pub click_change: ButtonChangeType,
}

#[derive(Clone)]
pub enum ButtonType {
    None,
    Rect(ButtonRect),
    Image(ButtonImage),
}

#[derive(Clone)]
pub enum ButtonContentType {
    None,
    Image(ButtonContentImg),
    Text(ButtonContentText),
}

pub struct Button {
    visible: bool,
    index: Option<usize>,
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
        visible: bool,
    ) -> Self {
        let buttontype = button_type.clone();
        let index = match buttontype {
            ButtonType::Rect(data) => {
                let mut rect = Rect::new(&mut systems.renderer, 0);
                rect.set_position(pos)
                    .set_size(size)
                    .set_color(data.rect_color)
                    .set_radius(data.border_radius);
                if data.got_border {
                    rect.set_border_width(1.0)
                        .set_border_color(data.border_color);
                }
                let rect_index = systems.gfx.add_rect(rect, render_layer);
                systems.gfx.set_visible(rect_index, visible);
                Some(rect_index)
            }
            ButtonType::Image(data) => {
                let mut image = Image::new(Some(data.res), &mut systems.renderer, 0);
                image.pos = pos;
                image.hw = size;
                image.uv = Vec4::new(0.0, 0.0, size.x, size.y);
                let image_index = systems.gfx.add_image(image, render_layer);
                systems.gfx.set_visible(image_index, visible);
                Some(image_index)
            }
            _ => None,
        };

        let contenttype = content_type.clone();
        let content = match contenttype {
            ButtonContentType::None => None,
            ButtonContentType::Image(data) => {
                let mut image = Image::new(Some(data.res), &mut systems.renderer, 0);
                image.pos = Vec3::new(pos.x + data.pos.x, pos.y + data.pos.y, data.pos.z);
                image.hw = data.size;
                image.uv = Vec4::new(data.uv.x, data.uv.y, data.size.x, data.size.y);
                let image_index = systems.gfx.add_image(image, render_layer);
                systems.gfx.set_visible(image_index, visible);
                Some(image_index)
            }
            ButtonContentType::Text(data) => {
                let text_pos = Vec2::new(pos.x + data.pos.x, pos.y + data.pos.y);
                let text = create_label(
                    systems, 
                    Vec3::new(text_pos.x, text_pos.y, data.pos.z), 
                    Vec2::new(size.x, 20.0), 
                    Bounds::new(text_pos.x, text_pos.y, text_pos.x + size.x, text_pos.y + 20.0), 
                    data.color);
                let index = systems.gfx.add_text(text, data.render_layer);
                systems.gfx.set_text(&mut systems.renderer, index, &data.text);
                systems.gfx.center_text(index);
                systems.gfx.set_visible(index, visible);
                Some(index)
            }
        };

        Button {
            visible,
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
        if let Some(index) = self.index {
            systems.gfx.remove_gfx(index);
        }
        if let Some(content_index) = self.content {
            systems.gfx.remove_gfx(content_index);
        }
    }

    pub fn set_visible(&mut self, systems: &mut DrawSetting, visible: bool) {
        if self.visible == visible {
            return;
        }
        self.visible = visible;
        if let Some(index) = self.index {
            systems.gfx.set_visible(index, visible);
        }
        if let Some(content_index) = self.content {
            systems.gfx.set_visible(content_index, visible);
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
        if let Some(index) = self.index {
            let buttontype = self.button_type.clone();
            match buttontype {
                ButtonType::Rect(data) => {
                    match data.click_change {
                        ButtonChangeType::AdjustY(adjusty) => {
                            systems.gfx.set_pos(index, 
                                Vec3::new(self.pos.x, self.pos.y + adjusty as f32, self.pos.z));
                        }
                        ButtonChangeType::ColorChange(color) => { systems.gfx.set_color(index, color); }
                        _ => {}
                    }
                }
                ButtonType::Image(data) => {
                    match data.click_change {
                        ButtonChangeType::AdjustY(adjusty) => {
                            systems.gfx.set_pos(index, 
                                Vec3::new(self.pos.x, self.pos.y + adjusty as f32, self.pos.z));
                        }
                        ButtonChangeType::ImageFrame(frame) => {
                            systems.gfx.set_uv(index, 
                                Vec4::new(0.0, self.size.y * frame as f32, self.size.x, self.size.y));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if let Some(content_data) = self.content {
            let contenttype = self.content_type.clone();
            match contenttype {
                ButtonContentType::Text(data) => {
                    match data.click_change {
                        ButtonChangeType::AdjustY(adjusty) => {
                            systems.gfx.set_pos(content_data, 
                                Vec3::new(self.pos.x + data.pos.x, self.pos.y + data.pos.y + adjusty as f32, data.pos.z));
                            systems.gfx.center_text(content_data);
                        }
                        ButtonChangeType::ColorChange(color) => { systems.gfx.set_color(content_data, color); }
                        _ => {}
                    }
                }
                ButtonContentType::Image(data) => {
                    match data.click_change {
                        ButtonChangeType::AdjustY(adjusty) => {
                            systems.gfx.set_pos(content_data, 
                                Vec3::new(self.pos.x + data.pos.x, self.pos.y + data.pos.y + adjusty as f32, data.pos.z));
                        }
                        ButtonChangeType::ImageFrame(frame) => {
                            systems.gfx.set_uv(content_data, 
                                Vec4::new(data.uv.x, data.uv.y + data.size.y * frame as f32, data.size.x, data.size.y));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    fn apply_hover(&mut self, systems: &mut DrawSetting) {
        if let Some(index) = self.index {
            let buttontype = self.button_type.clone();
            match buttontype {
                ButtonType::Rect(data) => {
                    match data.hover_change {
                        ButtonChangeType::AdjustY(adjusty) => {
                            systems.gfx.set_pos(index, 
                                Vec3::new(self.pos.x, self.pos.y + adjusty as f32, self.pos.z));
                        }
                        ButtonChangeType::ColorChange(color) => { systems.gfx.set_color(index, color); }
                        _ => {}
                    }
                }
                ButtonType::Image(data) => {
                    match data.hover_change {
                        ButtonChangeType::AdjustY(adjusty) => {
                            systems.gfx.set_pos(index, 
                                Vec3::new(self.pos.x, self.pos.y + adjusty as f32, self.pos.z));
                        }
                        ButtonChangeType::ImageFrame(frame) => {
                            systems.gfx.set_uv(index, 
                                Vec4::new(0.0, self.size.y * frame as f32, self.size.x, self.size.y));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if let Some(content_data) = self.content {
            let contenttype = self.content_type.clone();
            match contenttype {
                ButtonContentType::Text(data) => {
                    match data.hover_change {
                        ButtonChangeType::AdjustY(adjusty) => {
                            systems.gfx.set_pos(content_data, 
                                Vec3::new(self.pos.x + data.pos.x, self.pos.y + data.pos.y + adjusty as f32, data.pos.z));
                            systems.gfx.center_text(content_data);
                        }
                        ButtonChangeType::ColorChange(color) => {
                            systems.gfx.set_color(content_data, color);
                        }
                        _ => {}
                    }
                }
                ButtonContentType::Image(data) => {
                    match data.hover_change {
                        ButtonChangeType::AdjustY(adjusty) => {
                            systems.gfx.set_pos(content_data, 
                                Vec3::new(self.pos.x + data.pos.x, self.pos.y + data.pos.y + adjusty as f32, data.pos.z));
                        }
                        ButtonChangeType::ImageFrame(frame) => {
                            systems.gfx.set_uv(content_data, 
                                Vec4::new(data.uv.x, data.uv.y + data.size.y * frame as f32, data.size.x, data.size.y));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    fn apply_normal(&mut self, systems: &mut DrawSetting) {
        if let Some(index) = self.index {
            let buttontype = self.button_type.clone();
            systems.gfx.set_pos(index, self.pos);
            match buttontype {
                ButtonType::Rect(data) => {
                    systems.gfx.set_color(index, data.rect_color);
                }
                ButtonType::Image(_) => {
                    systems.gfx.set_uv(index, 
                        Vec4::new(0.0, 0.0, self.size.x, self.size.y));
                }
                _ => {}
            }
        }

        if let Some(content_data) = self.content {
            let contenttype = self.content_type.clone();
            match contenttype {
                ButtonContentType::Text(data) => {
                    systems.gfx.set_pos(content_data,
                        Vec3::new(self.pos.x + data.pos.x, self.pos.y + data.pos.y, data.pos.z));
                    systems.gfx.set_color(content_data, data.color);
                    systems.gfx.center_text(content_data);
                }
                ButtonContentType::Image(data) => {
                    systems.gfx.set_pos(content_data,
                        Vec3::new(self.pos.x + data.pos.x, self.pos.y + data.pos.y, data.pos.z));
                    systems.gfx.set_uv(content_data, 
                        Vec4::new(data.uv.x, data.uv.y, data.size.x, data.size.y));
                }
                _ => {}
            }
        }
    }
}