use crate::{
    logic::*, send_accepttrade, send_addtradeitem, send_declinetrade,
    send_deposititem, send_dropitem, send_removetradeitem, send_sellitem,
    send_switchinvslot, send_switchstorageslot, send_withdrawitem, socket,
    values::*, widget::*, MouseInputType, Result, Socket, SystemHolder,
};
use graphics::{cosmic_text::Attrs, *};
use input::Key;
use winit::keyboard::NamedKey;

#[derive(PartialEq, Eq)]
pub enum AlertType {
    Inform,
    Confirm,
    Input,
}

pub enum AlertIndex {
    None,
    Drop(u16),
    Sell(u16),
    AddTradeTradeItem(u16),
    RemoveTradeItem(u16),
    MergeInv(u16, u16),
    MergeStorage(u16, u16),
    Deposit(u16, u16),
    Withdraw(u16, u16),
    TradeRequest,
}

pub struct AlertTextbox {
    bg: usize,
    textbox: Textbox,
    selected: bool,
    numeric_only: bool,
}

pub struct Alert {
    window: Vec<usize>,
    text: Vec<usize>,
    button: Vec<Button>,
    alert_type: AlertType,
    input_box: Option<AlertTextbox>,
    pub visible: bool,
    did_button_click: bool,
    custom_index: AlertIndex,
}

impl Alert {
    pub fn new() -> Self {
        Alert {
            window: Vec::new(),
            button: Vec::new(),
            alert_type: AlertType::Inform,
            input_box: None,
            text: Vec::new(),
            visible: false,
            did_button_click: false,
            custom_index: AlertIndex::None,
        }
    }

    pub fn show_alert(
        &mut self,
        systems: &mut SystemHolder,
        alert_type: AlertType,
        msg: String,
        header: String,
        max_text_width: usize,
        custom_index: AlertIndex,
        numeric_only: bool,
    ) {
        if self.visible {
            self.window.iter().for_each(|gfx_index| {
                systems.gfx.remove_gfx(&mut systems.renderer, *gfx_index);
            });
            self.text.iter().for_each(|gfx_index| {
                systems.gfx.remove_gfx(&mut systems.renderer, *gfx_index);
            });
            self.button.iter_mut().for_each(|button| {
                button.unload(systems);
            });
            if let Some(textbox) = &mut self.input_box {
                systems.gfx.remove_gfx(&mut systems.renderer, textbox.bg);
                textbox.textbox.unload(systems);
            }
        }

        self.window.clear();
        self.text.clear();
        self.button.clear();
        self.input_box = None;
        self.custom_index = custom_index;

        let limit_width = match alert_type {
            AlertType::Inform => 80.0,
            AlertType::Confirm => 150.0,
            AlertType::Input => 170.0,
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

        let (pos, bounds) = if alert_type == AlertType::Input {
            let s_pos = Vec2::new(w_pos.x, w_pos.y + w_size.y - 30.0);
            (
                s_pos,
                Bounds::new(
                    s_pos.x,
                    s_pos.y,
                    s_pos.x + w_size.x,
                    s_pos.y + 20.0,
                ),
            )
        } else {
            let s_pos = Vec2::new(w_pos.x + 10.0, w_pos.y + w_size.y - 25.0);
            (
                s_pos,
                Bounds::new(
                    s_pos.x,
                    s_pos.y,
                    s_pos.x + header_text_size.x,
                    s_pos.y + 20.0,
                ),
            )
        };
        header_text
            .set_position(Vec3::new(pos.x, pos.y, ORDER_ALERT_TEXT))
            .set_bounds(Some(bounds));
        header_text.size =
            Vec2::new(header_text_size.x, header_text_size.y + 4.0);
        header_text.changed = true;
        let header_text_index = systems.gfx.add_text(header_text, 5);
        if alert_type == AlertType::Input {
            systems.gfx.center_text(header_text_index);
        }
        self.text.push(header_text_index);

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

        self.window.push(systems.gfx.add_rect(bg, 3));
        self.window.push(systems.gfx.add_rect(window, 4));

        if alert_type != AlertType::Input {
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
            self.text.push(systems.gfx.add_text(text, 5));

            let mut header = Rect::new(&mut systems.renderer, 0);
            header
                .set_position(Vec3::new(
                    w_pos.x,
                    w_pos.y + w_size.y - 30.0,
                    ORDER_ALERT_HEADER,
                ))
                .set_size(Vec2::new(w_size.x, 30.0))
                .set_color(Color::rgba(100, 100, 100, 255));
            self.window.push(systems.gfx.add_rect(header, 4));
        }

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
            AlertType::Input => {
                let textbox_pos = Vec2::new(
                    w_pos.x + ((w_size.x - 100.0) * 0.5).floor(),
                    w_pos.y + 50.0,
                );

                let mut textbox_bg = Rect::new(&mut systems.renderer, 0);
                textbox_bg
                    .set_size(Vec2::new(104.0, 24.0))
                    .set_color(Color::rgba(120, 120, 120, 255))
                    .set_border_width(1.0)
                    .set_border_color(Color::rgba(40, 40, 40, 255))
                    .set_position(Vec3::new(
                        textbox_pos.x,
                        textbox_pos.y,
                        ORDER_ALERT_TEXTBOX_BG,
                    ));
                let textbox = Textbox::new(
                    systems,
                    Vec3::new(
                        textbox_pos.x + 2.0,
                        textbox_pos.y + 2.0,
                        ORDER_ALERT_TEXTBOX,
                    ),
                    (0.001, 3),
                    Vec2::new(100.0, 20.0),
                    Color::rgba(200, 200, 200, 255),
                    5,
                    10,
                    Color::rgba(80, 80, 80, 255),
                    false,
                    true,
                    None,
                );
                self.input_box = Some(AlertTextbox {
                    bg: systems.gfx.add_rect(textbox_bg, 4),
                    textbox,
                    selected: false,
                    numeric_only,
                });

                let pos = Vec2::new(((w_size.x - 150.0) * 0.5).floor(), 10.0);
                self.button.push(Button::new(
                    systems,
                    ButtonType::Rect(button_detail.clone()),
                    ButtonContentType::Text(ButtonContentText {
                        text: "Confirm".into(),
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
                    Vec2::new(70.0, 30.0),
                    4,
                    true,
                    None,
                ));
                self.button.push(Button::new(
                    systems,
                    ButtonType::Rect(button_detail.clone()),
                    ButtonContentType::Text(ButtonContentText {
                        text: "Cancel".into(),
                        pos: Vec2::new(0.0, 5.0),
                        color: Color::rgba(255, 255, 255, 255),
                        render_layer: 5,
                        hover_change: ButtonChangeType::None,
                        click_change: ButtonChangeType::None,
                    }),
                    Vec2::new(w_pos.x, w_pos.y),
                    pos + Vec2::new(80.0, 0.0),
                    ORDER_ALERT_BUTTON,
                    (0.01, 2),
                    Vec2::new(70.0, 30.0),
                    4,
                    true,
                    None,
                ));
            }
        }

        self.alert_type = alert_type;

        self.visible = true;
    }

    pub fn hide_alert(&mut self, systems: &mut SystemHolder) {
        if !self.visible {
            return;
        }
        self.visible = false;
        self.window.iter().for_each(|gfx_index| {
            systems.gfx.remove_gfx(&mut systems.renderer, *gfx_index);
        });
        self.text.iter().for_each(|gfx_index| {
            systems.gfx.remove_gfx(&mut systems.renderer, *gfx_index);
        });
        self.button.iter_mut().for_each(|button| {
            button.unload(systems);
        });
        if let Some(textbox) = &mut self.input_box {
            systems.gfx.remove_gfx(&mut systems.renderer, textbox.bg);
            textbox.textbox.unload(systems);
        }
        self.input_box = None;
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
        socket: &mut Socket,
        input_type: MouseInputType,
        tooltip: &mut Tooltip,
        screen_pos: Vec2,
    ) -> Result<()> {
        if !self.visible {
            return Ok(());
        }
        match input_type {
            MouseInputType::MouseMove => {
                self.hover_buttons(systems, screen_pos);
                self.hover_textbox(systems, tooltip, screen_pos);
            }
            MouseInputType::MouseLeftDown => {
                let button_index = self.click_buttons(systems, screen_pos);
                if let Some(index) = button_index {
                    self.did_button_click = true;
                    self.select_option(systems, socket, index)?;
                }
                self.click_textbox(systems, screen_pos);
            }
            MouseInputType::MouseRelease => {
                self.reset_buttons(systems);
            }
            _ => {}
        }
        Ok(())
    }

    pub fn alert_key_input(
        &mut self,
        systems: &mut SystemHolder,
        key: &Key,
        pressed: bool,
    ) {
        if let Some(textbox) = &mut self.input_box {
            if textbox.selected {
                textbox.textbox.enter_text(
                    systems,
                    key,
                    pressed,
                    textbox.numeric_only,
                );
            }
        }
    }

    pub fn select_option(
        &mut self,
        systems: &mut SystemHolder,
        socket: &mut Socket,
        index: usize,
    ) -> Result<()> {
        match self.alert_type {
            AlertType::Inform =>
            {
                #[allow(clippy::match_single_binding)]
                match self.custom_index {
                    _ => self.hide_alert(systems),
                }
            }
            AlertType::Confirm => {
                match index {
                    #[allow(clippy::match_single_binding)]
                    0 => match self.custom_index {
                        AlertIndex::TradeRequest => {
                            send_accepttrade(socket)?;
                            self.hide_alert(systems);
                        }
                        _ => self.hide_alert(systems),
                    }, // Yes
                    #[allow(clippy::match_single_binding)]
                    _ => match self.custom_index {
                        AlertIndex::TradeRequest => {
                            send_declinetrade(socket)?;
                            self.hide_alert(systems);
                        }
                        _ => self.hide_alert(systems),
                    }, // No
                }
            }
            AlertType::Input => {
                if let Some(textbox) = &self.input_box {
                    let input_text = textbox.textbox.text.clone();
                    match index {
                        #[allow(clippy::match_single_binding)]
                        0 => match self.custom_index {
                            AlertIndex::Drop(slot) => {
                                let amount = input_text
                                    .parse::<u16>()
                                    .unwrap_or_default();
                                send_dropitem(socket, slot, amount)?;
                                self.hide_alert(systems);
                            }
                            AlertIndex::Sell(slot) => {
                                let amount = input_text
                                    .parse::<u16>()
                                    .unwrap_or_default();
                                send_sellitem(socket, slot, amount)?;
                                self.hide_alert(systems);
                            }
                            AlertIndex::AddTradeTradeItem(slot) => {
                                let amount = input_text
                                    .parse::<u16>()
                                    .unwrap_or_default();
                                send_addtradeitem(socket, slot, amount)?;
                                self.hide_alert(systems);
                            }
                            AlertIndex::RemoveTradeItem(slot) => {
                                let amount = input_text
                                    .parse::<u64>()
                                    .unwrap_or_default();
                                send_removetradeitem(socket, slot, amount)?;
                                self.hide_alert(systems);
                            }
                            AlertIndex::MergeInv(inv_slot, new_slot) => {
                                let amount = input_text
                                    .parse::<u16>()
                                    .unwrap_or_default();
                                send_switchinvslot(
                                    socket, inv_slot, new_slot, amount,
                                )?;
                                self.hide_alert(systems);
                            }
                            AlertIndex::MergeStorage(inv_slot, new_slot) => {
                                let amount = input_text
                                    .parse::<u16>()
                                    .unwrap_or_default();
                                send_switchstorageslot(
                                    socket, inv_slot, new_slot, amount,
                                )?;
                                self.hide_alert(systems);
                            }
                            AlertIndex::Deposit(inv_slot, bank_slot) => {
                                let amount = input_text
                                    .parse::<u16>()
                                    .unwrap_or_default();
                                send_deposititem(
                                    socket, inv_slot, bank_slot, amount,
                                )?;
                                self.hide_alert(systems);
                            }
                            AlertIndex::Withdraw(inv_slot, bank_slot) => {
                                let amount = input_text
                                    .parse::<u16>()
                                    .unwrap_or_default();
                                send_withdrawitem(
                                    socket, inv_slot, bank_slot, amount,
                                )?;
                                self.hide_alert(systems);
                            }
                            _ => self.hide_alert(systems),
                        }, // Yes
                        #[allow(clippy::match_single_binding)]
                        _ => match self.custom_index {
                            _ => self.hide_alert(systems),
                        }, // No
                    }
                }
            }
        }
        Ok(())
    }

    pub fn hover_textbox(
        &mut self,
        systems: &mut SystemHolder,
        tooltip: &mut Tooltip,
        screen_pos: Vec2,
    ) {
        if let Some(textbox) = &mut self.input_box {
            if is_within_area(
                screen_pos,
                Vec2::new(textbox.textbox.pos.x, textbox.textbox.pos.y),
                Vec2::new(textbox.textbox.size.x, textbox.textbox.size.y),
            ) {
                if let Some(msg) = &textbox.textbox.tooltip {
                    tooltip.init_tooltip(systems, screen_pos, msg.clone());
                }
            }
        }
    }

    pub fn click_textbox(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
    ) {
        if let Some(textbox) = &mut self.input_box {
            if is_within_area(
                screen_pos,
                Vec2::new(textbox.textbox.pos.x, textbox.textbox.pos.y),
                textbox.textbox.size,
            ) {
                textbox.textbox.set_select(systems, true);
                textbox.selected = true;
            } else {
                textbox.textbox.set_select(systems, false);
                textbox.selected = false;
            }
        }
    }
}
