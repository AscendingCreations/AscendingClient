use cosmic_text::{Attrs, Metrics};
use graphics::*;

use crate::{logic::*, widget::*, SystemHolder};

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
    pub pos: Vec2,
    pub uv: Vec2,
    pub size: Vec2,
    pub hover_change: ButtonChangeType,
    pub click_change: ButtonChangeType,
}

#[derive(Clone)]
pub struct ButtonContentText {
    pub text: String,
    pub pos: Vec2,
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

    pub base_pos: Vec2,
    pub adjust_pos: Vec2,
    pub z_order: f32,
    pub size: Vec2,
    z_step: (f32, i32),
    pub tooltip: Option<String>,
}

impl Button {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        systems: &mut SystemHolder,
        button_type: ButtonType,
        content_type: ButtonContentType,
        base_pos: Vec2,
        adjust_pos: Vec2,
        z_order: f32,
        z_step: (f32, i32),
        size: Vec2,
        render_layer: usize,
        visible: bool,
        tooltip: Option<String>,
    ) -> Self {
        let pos = base_pos + (adjust_pos * systems.scale as f32).floor();

        let buttontype = button_type.clone();
        let index = match buttontype {
            ButtonType::Rect(data) => {
                let mut rect = Rect::new(&mut systems.renderer, 0);
                rect.set_position(Vec3::new(pos.x, pos.y, z_order))
                    .set_size((size * systems.scale as f32).floor())
                    .set_color(data.rect_color)
                    .set_radius(data.border_radius);
                if data.got_border {
                    rect.set_border_width(1.0)
                        .set_border_color(data.border_color);
                }
                let rect_index = systems.gfx.add_rect(
                    rect,
                    render_layer,
                    "Button Image".into(),
                );
                systems.gfx.set_visible(rect_index, visible);
                Some(rect_index)
            }
            ButtonType::Image(data) => {
                let mut image =
                    Image::new(Some(data.res), &mut systems.renderer, 0);
                image.pos = Vec3::new(pos.x, pos.y, z_order);
                image.hw = (size * systems.scale as f32).floor();
                image.uv = Vec4::new(0.0, 0.0, size.x, size.y);
                let image_index = systems.gfx.add_image(
                    image,
                    render_layer,
                    "Button Image".into(),
                );
                systems.gfx.set_visible(image_index, visible);
                Some(image_index)
            }
            _ => None,
        };

        let contenttype = content_type.clone();
        let content = match contenttype {
            ButtonContentType::None => None,
            ButtonContentType::Image(data) => {
                let mut image =
                    Image::new(Some(data.res), &mut systems.renderer, 0);
                let spos = Vec3::new(
                    pos.x,
                    pos.y,
                    z_order.sub_f32(z_step.0, z_step.1),
                );
                image.pos = Vec3::new(
                    spos.x + (data.pos.x * systems.scale as f32).floor(),
                    spos.y + (data.pos.y * systems.scale as f32).floor(),
                    spos.z,
                );
                image.hw = (data.size * systems.scale as f32).floor();
                image.uv =
                    Vec4::new(data.uv.x, data.uv.y, data.size.x, data.size.y);
                let image_index = systems.gfx.add_image(
                    image,
                    render_layer,
                    "Button Content".into(),
                );
                systems.gfx.set_visible(image_index, visible);
                Some(image_index)
            }
            ButtonContentType::Text(data) => {
                let spos = Vec3::new(
                    pos.x,
                    pos.y,
                    z_order.sub_f32(z_step.0, z_step.1),
                );
                let text_pos = Vec2::new(
                    spos.x + (data.pos.x * systems.scale as f32).floor(),
                    spos.y + (data.pos.y * systems.scale as f32).floor(),
                );
                let text = create_label(
                    systems,
                    Vec3::new(text_pos.x, text_pos.y, spos.z),
                    (Vec2::new(size.x, 20.0) * systems.scale as f32).floor(),
                    Bounds::new(
                        text_pos.x,
                        text_pos.y,
                        text_pos.x + (size.x * systems.scale as f32).floor(),
                        text_pos.y + (20.0 * systems.scale as f32).floor(),
                    ),
                    data.color,
                );
                let index = systems.gfx.add_text(
                    text,
                    data.render_layer,
                    "Button Content".into(),
                );
                systems
                    .gfx
                    .set_text(&mut systems.renderer, index, &data.text);
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
            base_pos,
            adjust_pos,
            z_order,
            z_step,
            size,
            tooltip,
        }
    }

    pub fn unload(&mut self, systems: &mut SystemHolder) {
        if let Some(index) = self.index {
            systems.gfx.remove_gfx(&mut systems.renderer, index);
        }
        if let Some(content_index) = self.content {
            systems.gfx.remove_gfx(&mut systems.renderer, content_index);
        }
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        if self.visible == visible {
            return;
        }
        if !visible {
            self.set_click(systems, false);
            self.set_hover(systems, false);
        }
        self.visible = visible;
        if let Some(index) = self.index {
            systems.gfx.set_visible(index, visible);
        }
        if let Some(index) = self.content {
            systems.gfx.set_visible(index, visible);
        }
    }

    pub fn set_z_order(&mut self, systems: &mut SystemHolder, z_order: f32) {
        self.z_order = z_order;
        if let Some(index) = self.index {
            let pos = systems.gfx.get_pos(index);
            systems
                .gfx
                .set_pos(index, Vec3::new(pos.x, pos.y, self.z_order));
        }
        if let Some(content_index) = self.content {
            let pos = systems.gfx.get_pos(content_index);
            systems.gfx.set_pos(
                content_index,
                Vec3::new(
                    pos.x,
                    pos.y,
                    self.z_order.sub_f32(self.z_step.0, self.z_step.1),
                ),
            );
        }
    }

    pub fn set_pos(&mut self, systems: &mut SystemHolder, new_pos: Vec2) {
        self.base_pos = new_pos;
        if let Some(index) = self.index {
            let pos = Vec3::new(
                self.base_pos.x
                    + (self.adjust_pos.x * systems.scale as f32).floor(),
                self.base_pos.y
                    + (self.adjust_pos.y * systems.scale as f32).floor(),
                self.z_order,
            );
            systems.gfx.set_pos(index, pos);
        }
        if let Some(content_index) = self.content {
            let contenttype = self.content_type.clone();
            match contenttype {
                ButtonContentType::Image(data) => {
                    let pos = Vec3::new(
                        self.base_pos.x
                            + ((self.adjust_pos.x + data.pos.x)
                                * systems.scale as f32)
                                .floor(),
                        self.base_pos.y
                            + ((self.adjust_pos.y + data.pos.y)
                                * systems.scale as f32)
                                .floor(),
                        self.z_order.sub_f32(self.z_step.0, self.z_step.1),
                    );
                    systems.gfx.set_pos(content_index, pos);
                }
                ButtonContentType::Text(data) => {
                    let pos = Vec3::new(
                        self.base_pos.x
                            + ((self.adjust_pos.x + data.pos.x)
                                * systems.scale as f32)
                                .floor(),
                        self.base_pos.y
                            + ((self.adjust_pos.y + data.pos.y)
                                * systems.scale as f32)
                                .floor(),
                        self.z_order.sub_f32(self.z_step.0, self.z_step.1),
                    );
                    systems.gfx.set_pos(content_index, pos);
                    systems.gfx.set_bound(
                        content_index,
                        Bounds::new(
                            pos.x,
                            pos.y,
                            pos.x + self.size.x,
                            pos.y + self.size.y,
                        ),
                    );
                    systems.gfx.center_text(content_index);
                }
                _ => {}
            };
        }
    }

    pub fn set_hover(&mut self, systems: &mut SystemHolder, state: bool) {
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

    pub fn set_click(&mut self, systems: &mut SystemHolder, state: bool) {
        if self.in_click == state || !self.visible {
            return;
        }
        self.in_click = state;

        if self.in_click {
            self.apply_click(systems);
        } else if self.in_hover {
            self.apply_hover(systems);
        } else {
            self.apply_normal(systems);
        }
    }

    pub fn change_text(&mut self, systems: &mut SystemHolder, msg: String) {
        if let Some(content_data) = self.content {
            if let ButtonContentType::Text(data) = &mut self.content_type {
                systems
                    .gfx
                    .set_text(&mut systems.renderer, content_data, &msg);
                data.text = msg;
                systems.gfx.center_text(content_data);
            }
        }
    }

    fn apply_click(&mut self, systems: &mut SystemHolder) {
        let pos =
            self.base_pos + (self.adjust_pos * systems.scale as f32).floor();
        if let Some(index) = self.index {
            let buttontype = self.button_type.clone();
            match buttontype {
                ButtonType::Rect(data) => match data.click_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            index,
                            Vec3::new(
                                pos.x,
                                pos.y
                                    + (adjusty as f32 * systems.scale as f32)
                                        .floor(),
                                self.z_order,
                            ),
                        );
                    }
                    ButtonChangeType::ColorChange(color) => {
                        systems.gfx.set_color(index, color);
                    }
                    _ => {}
                },
                ButtonType::Image(data) => match data.click_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            index,
                            Vec3::new(
                                pos.x,
                                pos.y
                                    + (adjusty as f32 * systems.scale as f32)
                                        .floor(),
                                self.z_order,
                            ),
                        );
                    }
                    ButtonChangeType::ImageFrame(frame) => {
                        systems.gfx.set_uv(
                            index,
                            Vec4::new(
                                0.0,
                                self.size.y * frame as f32,
                                self.size.x,
                                self.size.y,
                            ),
                        );
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if let Some(content_data) = self.content {
            let contenttype = self.content_type.clone();
            match contenttype {
                ButtonContentType::Text(data) => match data.click_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            content_data,
                            Vec3::new(
                                pos.x
                                    + (data.pos.x * systems.scale as f32)
                                        .floor(),
                                pos.y
                                    + ((data.pos.y + adjusty as f32)
                                        * systems.scale as f32)
                                        .floor(),
                                self.z_order
                                    .sub_f32(self.z_step.0, self.z_step.1),
                            ),
                        );
                        systems.gfx.center_text(content_data);
                    }
                    ButtonChangeType::ColorChange(color) => {
                        systems.gfx.set_color(content_data, color);
                    }
                    _ => {}
                },
                ButtonContentType::Image(data) => match data.click_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            content_data,
                            Vec3::new(
                                pos.x
                                    + (data.pos.x * systems.scale as f32)
                                        .floor(),
                                pos.y
                                    + ((data.pos.y + adjusty as f32)
                                        * systems.scale as f32)
                                        .floor(),
                                self.z_order
                                    .sub_f32(self.z_step.0, self.z_step.1),
                            ),
                        );
                    }
                    ButtonChangeType::ImageFrame(frame) => {
                        systems.gfx.set_uv(
                            content_data,
                            Vec4::new(
                                data.uv.x,
                                data.uv.y + data.size.y * frame as f32,
                                data.size.x,
                                data.size.y,
                            ),
                        );
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn apply_hover(&mut self, systems: &mut SystemHolder) {
        let pos =
            self.base_pos + (self.adjust_pos * systems.scale as f32).floor();
        if let Some(index) = self.index {
            let buttontype = self.button_type.clone();
            match buttontype {
                ButtonType::Rect(data) => match data.hover_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            index,
                            Vec3::new(
                                pos.x,
                                pos.y
                                    + (adjusty as f32 * systems.scale as f32)
                                        .floor(),
                                self.z_order,
                            ),
                        );
                    }
                    ButtonChangeType::ColorChange(color) => {
                        systems.gfx.set_color(index, color);
                    }
                    _ => {}
                },
                ButtonType::Image(data) => match data.hover_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            index,
                            Vec3::new(
                                pos.x,
                                pos.y
                                    + (adjusty as f32 * systems.scale as f32)
                                        .floor(),
                                self.z_order,
                            ),
                        );
                    }
                    ButtonChangeType::ImageFrame(frame) => {
                        systems.gfx.set_uv(
                            index,
                            Vec4::new(
                                0.0,
                                self.size.y * frame as f32,
                                self.size.x,
                                self.size.y,
                            ),
                        );
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if let Some(content_data) = self.content {
            let contenttype = self.content_type.clone();
            match contenttype {
                ButtonContentType::Text(data) => match data.hover_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            content_data,
                            Vec3::new(
                                pos.x
                                    + (data.pos.x * systems.scale as f32)
                                        .floor(),
                                pos.y
                                    + ((data.pos.y + adjusty as f32)
                                        * systems.scale as f32)
                                        .floor(),
                                self.z_order
                                    .sub_f32(self.z_step.0, self.z_step.1),
                            ),
                        );
                        systems.gfx.center_text(content_data);
                    }
                    ButtonChangeType::ColorChange(color) => {
                        systems.gfx.set_color(content_data, color);
                    }
                    _ => {}
                },
                ButtonContentType::Image(data) => match data.hover_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            content_data,
                            Vec3::new(
                                pos.x
                                    + (data.pos.x * systems.scale as f32)
                                        .floor(),
                                pos.y
                                    + ((data.pos.y + adjusty as f32)
                                        * systems.scale as f32)
                                        .floor(),
                                self.z_order
                                    .sub_f32(self.z_step.0, self.z_step.1),
                            ),
                        );
                    }
                    ButtonChangeType::ImageFrame(frame) => {
                        systems.gfx.set_uv(
                            content_data,
                            Vec4::new(
                                data.uv.x,
                                data.uv.y + data.size.y * frame as f32,
                                data.size.x,
                                data.size.y,
                            ),
                        );
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn apply_normal(&mut self, systems: &mut SystemHolder) {
        let pos =
            self.base_pos + (self.adjust_pos * systems.scale as f32).floor();
        if let Some(index) = self.index {
            let buttontype = self.button_type.clone();
            systems
                .gfx
                .set_pos(index, Vec3::new(pos.x, pos.y, self.z_order));
            match buttontype {
                ButtonType::Rect(data) => {
                    systems.gfx.set_color(index, data.rect_color);
                }
                ButtonType::Image(_) => {
                    systems.gfx.set_uv(
                        index,
                        Vec4::new(0.0, 0.0, self.size.x, self.size.y),
                    );
                }
                _ => {}
            }
        }

        if let Some(content_data) = self.content {
            let contenttype = self.content_type.clone();
            match contenttype {
                ButtonContentType::Text(data) => {
                    systems.gfx.set_pos(
                        content_data,
                        Vec3::new(
                            pos.x + (data.pos.x * systems.scale as f32).floor(),
                            pos.y + (data.pos.y * systems.scale as f32).floor(),
                            self.z_order.sub_f32(self.z_step.0, self.z_step.1),
                        ),
                    );
                    systems.gfx.set_color(content_data, data.color);
                    systems.gfx.center_text(content_data);
                }
                ButtonContentType::Image(data) => {
                    systems.gfx.set_pos(
                        content_data,
                        Vec3::new(
                            pos.x + (data.pos.x * systems.scale as f32).floor(),
                            pos.y + (data.pos.y * systems.scale as f32).floor(),
                            self.z_order.sub_f32(self.z_step.0, self.z_step.1),
                        ),
                    );
                    systems.gfx.set_uv(
                        content_data,
                        Vec4::new(
                            data.uv.x,
                            data.uv.y,
                            data.size.x,
                            data.size.y,
                        ),
                    );
                }
                _ => {}
            }
        }
    }
}
