use crate::{logic::*, values::*, widget::*, MouseInputType, SystemHolder};
use graphics::{cosmic_text::Attrs, *};

pub enum AlertType {
    Inform,
    Confirm,
}

pub struct Alert {
    window: Vec<usize>,
    text: Vec<usize>,
    button: Vec<Button>,
    alert_type: AlertType,
    pub visible: bool,
    did_button_click: bool,
    alert_custom_index: usize,
}

impl Alert {
    pub fn new() -> Self {
        Alert {
            window: Vec::new(),
            button: Vec::new(),
            alert_type: AlertType::Inform,
            text: Vec::new(),
            visible: false,
            did_button_click: false,
            alert_custom_index: 0,
        }
    }

    pub fn show_alert(
        &mut self,
        systems: &mut SystemHolder,
        alert_type: AlertType,
        msg: String,
        header: String,
        max_text_width: usize,
        custom_index: Option<usize>,
    ) {
        if self.visible {
            self.window.iter().for_each(|gfx_index| {
                systems.gfx.remove_gfx(*gfx_index);
            });
            self.text.iter().for_each(|gfx_index| {
                systems.gfx.remove_gfx(*gfx_index);
            });
            self.button.iter_mut().for_each(|button| {
                button.unload(systems);
            });
        }

        self.window.clear();
        self.text.clear();
        self.button.clear();

        if let Some(index) = custom_index {
            self.alert_custom_index = index
        }

        let limit_width = match alert_type {
            AlertType::Inform => 80.0,
            AlertType::Confirm => 150.0,
        };

        let mut text = create_empty_label(systems);
        text.set_buffer_size(&mut systems.renderer, max_text_width as i32, 128)
            .set_wrap(&mut systems.renderer, cosmic_text::Wrap::Word);
        text.set_text(
            &mut systems.renderer,
            &msg,
            Attrs::new(),
            Shaping::Advanced,
        );
        let text_size = text.measure().floor();

        let mut header_text = create_empty_label(systems);
        header_text.set_text(
            &mut systems.renderer,
            &header,
            Attrs::new(),
            Shaping::Advanced,
        );
        let header_text_size = header_text.measure().floor();

        let text_width = header_text_size.x.max(text_size.x);

        let center = get_screen_center(&systems.size);
        let w_size = Vec2::new(
            (text_width + 20.0).max(limit_width),
            (text_size.y + 90.0).max(110.0),
        );
        let w_pos = Vec3::new(
            (center.x - (w_size.x * 0.5)).floor(),
            (center.y - (w_size.y * 0.5)).floor(),
            ORDER_ALERT,
        );

        let pos = Vec2::new(
            w_pos.x + ((w_size.x - text_size.x) * 0.5).floor(),
            w_pos.y + 43.0,
        );
        text.set_position(Vec3::new(pos.x, pos.y, ORDER_ALERT_TEXT))
            .set_bounds(Some(Bounds::new(
                pos.x,
                pos.y,
                pos.x + text_size.x,
                pos.y + text_size.y + 10.0,
            )));
        text.size = Vec2::new(text_size.x, text_size.y + 10.0);
        text.changed = true;

        let pos = Vec2::new(w_pos.x + 10.0, w_pos.y + w_size.y - 25.0);
        header_text
            .set_position(Vec3::new(pos.x, pos.y, ORDER_ALERT_TEXT))
            .set_bounds(Some(Bounds::new(
                pos.x,
                pos.y,
                pos.x + header_text_size.x,
                pos.y + 20.0,
            )));
        header_text.size =
            Vec2::new(header_text_size.x, header_text_size.y + 4.0);
        header_text.changed = true;

        let mut bg = Rect::new(&mut systems.renderer, 0);
        bg.set_position(Vec3::new(0.0, 0.0, ORDER_ALERT_BG))
            .set_size(Vec2::new(systems.size.width, systems.size.height))
            .set_color(Color::rgba(10, 10, 10, 140));

        let mut window = Rect::new(&mut systems.renderer, 0);
        window
            .set_position(w_pos - Vec3::new(1.0, 1.0, 0.0))
            .set_size(w_size + Vec2::new(2.0, 2.0))
            .set_border_width(1.0)
            .set_border_color(Color::rgba(40, 40, 40, 255))
            .set_color(Color::rgba(160, 160, 160, 255));

        let mut header = Rect::new(&mut systems.renderer, 0);
        header
            .set_position(Vec3::new(
                w_pos.x,
                w_pos.y + w_size.y - 30.0,
                ORDER_ALERT_HEADER,
            ))
            .set_size(Vec2::new(w_size.x, 30.0))
            .set_color(Color::rgba(100, 100, 100, 255));

        self.window.push(systems.gfx.add_rect(bg, 3));
        self.window.push(systems.gfx.add_rect(window, 4));
        self.window.push(systems.gfx.add_rect(header, 4));
        self.text.push(systems.gfx.add_text(text, 5));
        self.text.push(systems.gfx.add_text(header_text, 5));

        let button_detail = ButtonRect {
            rect_color: Color::rgba(70, 70, 70, 255),
            got_border: true,
            border_color: Color::rgba(40, 40, 40, 255),
            border_radius: 0.0,
            hover_change: ButtonChangeType::ColorChange(Color::rgba(
                150, 150, 150, 255,
            )),
            click_change: ButtonChangeType::ColorChange(Color::rgba(
                200, 200, 200, 255,
            )),
        };

        match alert_type {
            AlertType::Inform => {
                let pos = Vec2::new(((w_size.x - 60.0) * 0.5).floor(), 10.0);
                self.button.push(Button::new(
                    systems,
                    ButtonType::Rect(button_detail.clone()),
                    ButtonContentType::Text(ButtonContentText {
                        text: "Okay".into(),
                        pos: Vec2::new(0.0, 5.0),
                        color: Color::rgba(255, 255, 255, 255),
                        render_layer: 5,
                        hover_change: ButtonChangeType::None,
                        click_change: ButtonChangeType::None,
                    }),
                    Vec2::new(w_pos.x, w_pos.y),
                    pos,
                    ORDER_ALERT_BUTTON,
                    (0.01, 2),
                    Vec2::new(60.0, 30.0),
                    4,
                    true,
                    None,
                ));
            }
            AlertType::Confirm => {
                let pos = Vec2::new(((w_size.x - 130.0) * 0.5).floor(), 10.0);
                self.button.push(Button::new(
                    systems,
                    ButtonType::Rect(button_detail.clone()),
                    ButtonContentType::Text(ButtonContentText {
                        text: "Yes".into(),
                        pos: Vec2::new(0.0, 5.0),
                        color: Color::rgba(255, 255, 255, 255),
                        render_layer: 5,
                        hover_change: ButtonChangeType::None,
                        click_change: ButtonChangeType::None,
                    }),
                    Vec2::new(w_pos.x, w_pos.y),
                    pos,
                    ORDER_ALERT_BUTTON,
                    (0.01, 2),
                    Vec2::new(60.0, 30.0),
                    4,
                    true,
                    None,
                ));
                self.button.push(Button::new(
                    systems,
                    ButtonType::Rect(button_detail.clone()),
                    ButtonContentType::Text(ButtonContentText {
                        text: "No".into(),
                        pos: Vec2::new(0.0, 5.0),
                        color: Color::rgba(255, 255, 255, 255),
                        render_layer: 5,
                        hover_change: ButtonChangeType::None,
                        click_change: ButtonChangeType::None,
                    }),
                    Vec2::new(w_pos.x, w_pos.y),
                    pos + Vec2::new(70.0, 0.0),
                    ORDER_ALERT_BUTTON,
                    (0.01, 2),
                    Vec2::new(60.0, 30.0),
                    4,
                    true,
                    None,
                ));
            }
        }

        self.visible = true;
    }

    pub fn hide_alert(&mut self, systems: &mut SystemHolder) {
        if !self.visible {
            return;
        }
        self.visible = false;
        self.window.iter().for_each(|gfx_index| {
            systems.gfx.remove_gfx(*gfx_index);
        });
        self.text.iter().for_each(|gfx_index| {
            systems.gfx.remove_gfx(*gfx_index);
        });
        self.button.iter_mut().for_each(|button| {
            button.unload(systems);
        });
    }

    pub fn hover_buttons(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
    ) {
        for button in self.button.iter_mut() {
            if is_within_area(
                screen_pos,
                Vec2::new(
                    button.base_pos.x + button.adjust_pos.x,
                    button.base_pos.y + button.adjust_pos.y,
                ),
                button.size,
            ) {
                button.set_hover(systems, true);
            } else {
                button.set_hover(systems, false);
            }
        }
    }

    pub fn click_buttons(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
    ) -> Option<usize> {
        let mut button_found = None;
        for (index, button) in self.button.iter_mut().enumerate() {
            if is_within_area(
                screen_pos,
                Vec2::new(
                    button.base_pos.x + button.adjust_pos.x,
                    button.base_pos.y + button.adjust_pos.y,
                ),
                button.size,
            ) {
                button.set_click(systems, true);
                button_found = Some(index)
            }
        }
        button_found
    }

    pub fn reset_buttons(&mut self, systems: &mut SystemHolder) {
        if !self.did_button_click {
            return;
        }
        self.did_button_click = false;

        self.button.iter_mut().for_each(|button| {
            button.set_click(systems, false);
        });
    }

    pub fn alert_mouse_input(
        &mut self,
        systems: &mut SystemHolder,
        input_type: MouseInputType,
        screen_pos: Vec2,
    ) {
        if !self.visible {
            return;
        }
        match input_type {
            MouseInputType::MouseMove => {
                self.hover_buttons(systems, screen_pos);
            }
            MouseInputType::MouseLeftDown => {
                let button_index = self.click_buttons(systems, screen_pos);
                if let Some(index) = button_index {
                    self.did_button_click = true;
                    self.select_option(systems, index);
                }
            }
            MouseInputType::MouseRelease => {
                self.reset_buttons(systems);
            }
            _ => {}
        }
    }

    pub fn select_option(&mut self, systems: &mut SystemHolder, index: usize) {
        match self.alert_type {
            AlertType::Inform =>
            {
                #[allow(clippy::match_single_binding)]
                match self.alert_custom_index {
                    _ => self.hide_alert(systems),
                }
            }
            AlertType::Confirm => {
                match index {
                    #[allow(clippy::match_single_binding)]
                    0 => match self.alert_custom_index {
                        _ => self.hide_alert(systems),
                    }, // Yes
                    #[allow(clippy::match_single_binding)]
                    _ => match self.alert_custom_index {
                        _ => self.hide_alert(systems),
                    }, // No
                }
            }
        }
    }
}
