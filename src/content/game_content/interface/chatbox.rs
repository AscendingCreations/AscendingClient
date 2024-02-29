use graphics::*;

use crate::{
    gfx_order::*, is_within_area, next_down, widget::*, DrawSetting
};

const MAX_CHAT_LINE: usize = 8;

pub struct Chatbox {
    window: usize,
    textbox_bg: usize,
    chatarea_bg: usize,
    pub textbox: Textbox,
    button: [Button; 3],
    pub did_button_click: bool,
    pub scrollbar: Scrollbar,

    pub pos: Vec2,
    pub size: Vec2,
    pub z_order: f32,
    in_hold: bool,
    hold_pos: Vec2,
}

impl Chatbox {
    pub fn new(systems: &mut DrawSetting) -> Self {
        let w_pos = Vec3::new(10.0, 10.0, ORDER_GUI_WINDOW);
        let w_size = Vec2::new(350.0, 200.0);

        let mut window_rect = Rect::new(&mut systems.renderer, 0);
        window_rect.set_position(w_pos)
            .set_size(w_size)
            .set_color(Color::rgba(120, 120, 120, 255))
            .set_border_width(1.0)
            .set_border_color(Color::rgba(40, 40, 40, 255));
        let window = systems.gfx.add_rect(window_rect, 0);

        let mut textbox_rect = Rect::new(&mut systems.renderer, 0);
        let textbox_zpos = next_down(w_pos.z);
        textbox_rect.set_position(Vec3::new(w_pos.x + 5.0, w_pos.y + 5.0, textbox_zpos))
            .set_size(Vec2::new(w_size.x - 75.0, 24.0))
            .set_color(Color::rgba(80, 80, 80, 255));
        let textbox_bg = systems.gfx.add_rect(textbox_rect, 0);

        let mut chatarea_rect = Rect::new(&mut systems.renderer, 0);
        chatarea_rect.set_position(Vec3::new(w_pos.x + 5.0, w_pos.y + 34.0, next_down(w_pos.z)))
            .set_size(Vec2::new(w_size.x - 39.0, w_size.y - 39.0))
            .set_color(Color::rgba(160, 160, 160, 255));
        let chatarea_bg = systems.gfx.add_rect(chatarea_rect, 0);

        let textbox = Textbox::new(
            systems,
            Vec3::new(w_pos.x + 7.0, w_pos.y + 7.0, next_down(textbox_zpos)),
            Vec2::new(w_size.x - 79.0, 20.0),
            Color::rgba(200, 200, 200, 255),
            1,
            255,
            Color::rgba(120, 120, 120, 255),
            false,
            true,
        );

        let button = [
            Button::new(
                systems,
                ButtonType::Rect(
                    ButtonRect {
                        rect_color: Color::rgba(80, 80, 80, 255),
                        got_border: false,
                        border_color: Color::rgba(0,0,0,0),
                        border_radius: 0.0,
                        hover_change: ButtonChangeType::ColorChange(Color::rgba(150, 150, 150, 255)),
                        click_change: ButtonChangeType::ColorChange(Color::rgba(80, 80, 80, 255))
                    }
                ),
                ButtonContentType::Image(
                    ButtonContentImg {
                        res: systems.resource.vertical_arrow.allocation,
                        pos: Vec2::new(0.0, 0.0),
                        uv: Vec2::new(0.0, 0.0),
                        size: Vec2::new(24.0, 24.0),
                        hover_change: ButtonChangeType::None,
                        click_change: ButtonChangeType::None,
                    }
                ),
                Vec2::new(w_pos.x, w_pos.y),
                Vec2::new(w_size.x - 29.0, w_size.y - 29.0),
                next_down(w_pos.z),
                Vec2::new(24.0, 24.0),
                0,
                true
            ),
            Button::new(
                systems,
                ButtonType::Rect(
                    ButtonRect {
                        rect_color: Color::rgba(80, 80, 80, 255),
                        got_border: false,
                        border_color: Color::rgba(0,0,0,0),
                        border_radius: 0.0,
                        hover_change: ButtonChangeType::ColorChange(Color::rgba(150, 150, 150, 255)),
                        click_change: ButtonChangeType::ColorChange(Color::rgba(80, 80, 80, 255))
                    }
                ),
                ButtonContentType::Image(
                    ButtonContentImg {
                        res: systems.resource.vertical_arrow.allocation,
                        pos: Vec2::new(0.0, 0.0),
                        uv: Vec2::new(24.0, 0.0),
                        size: Vec2::new(24.0, 24.0),
                        hover_change: ButtonChangeType::None,
                        click_change: ButtonChangeType::None,
                    }
                ),
                Vec2::new(w_pos.x, w_pos.y),
                Vec2::new(w_size.x - 29.0, 34.0),
                next_down(w_pos.z),
                Vec2::new(24.0, 24.0),
                0,
                true
            ),
            Button::new(
                systems,
                ButtonType::Rect(
                    ButtonRect {
                        rect_color: Color::rgba(80, 80, 80, 255),
                        got_border: false,
                        border_color: Color::rgba(0,0,0,0),
                        border_radius: 0.0,
                        hover_change: ButtonChangeType::ColorChange(Color::rgba(150, 150, 150, 255)),
                        click_change: ButtonChangeType::ColorChange(Color::rgba(80, 80, 80, 255))
                    }
                ),
                ButtonContentType::Text(
                    ButtonContentText {
                        text: "Send".to_string(),
                        pos: Vec2::new(0.0, 2.0),
                        color: Color::rgba(255, 255, 255, 255),
                        render_layer: 1,
                        hover_change: ButtonChangeType::ColorChange(Color::rgba(255, 255, 255, 255)),
                        click_change: ButtonChangeType::ColorChange(Color::rgba(255, 255, 255, 255)),
                    }
                ),
                Vec2::new(w_pos.x, w_pos.y),
                Vec2::new(w_size.x - 65.0, 5.0),
                next_down(w_pos.z),
                Vec2::new(60.0, 24.0),
                0,
                true
            ),
        ];

        let scrollbar = Scrollbar::new(
            systems,
            Vec2::new(w_pos.x, w_pos.y),
            Vec2::new(w_size.x - 28.0, 63.0),
            w_size.y - 97.0,
            22.0,
            true,
            next_down(w_pos.z),
            ScrollbarRect {
                color: Color::rgba(190, 190, 190, 255),
                render_layer: 0,
                got_border: true,
                border_color: Color::rgba(50, 50, 50, 255),
                hover_color: Color::rgba(240, 240, 240, 255),
                hold_color: Color::rgba(50, 50, 50, 255),
                radius: 5.0,
            },
            Some(ScrollbarBackground {
                color: Color::rgba(80, 80, 80, 255),
                render_layer: 0,
                got_border: false,
                border_color: Color::rgba(0, 0, 0, 0),
                radius: 0.0,
            }),
            3,
            20.0,
            true,
        );

        Chatbox {
            window,
            textbox_bg,
            chatarea_bg,
            textbox,
            button,
            did_button_click: false,
            scrollbar,
            pos: Vec2::new(w_pos.x, w_pos.y),
            size: w_size,
            z_order: w_pos.z,
            in_hold: false,
            hold_pos: Vec2::new(0.0, 0.0),
        }
    }

    pub fn unload(&mut self, systems: &mut DrawSetting) {
        systems.gfx.remove_gfx(self.window);
        systems.gfx.remove_gfx(self.textbox_bg);
        systems.gfx.remove_gfx(self.chatarea_bg);
        self.textbox.unload(systems);
        self.button.iter_mut().for_each(|button| {
            button.unload(systems);
        });
        self.scrollbar.unload(systems);
    }

    pub fn can_hold(&mut self, screen_pos: Vec2) -> bool {
        if self.scrollbar.in_scroll(screen_pos) {
            return false;
        }
        for button in self.button.iter() {
            let target_pos = button.base_pos + button.adjust_pos;
            if is_within_area(screen_pos, target_pos, button.size) {
                return false;
            }
        }
        if is_within_area(screen_pos, 
            Vec2::new(self.textbox.pos.x, self.textbox.pos.y), 
            self.textbox.size) {
            return false;
        }
        if !is_within_area(screen_pos, self.pos, self.size) {
            return false;
        }
        true
    }

    pub fn in_window(&mut self, screen_pos: Vec2) -> bool {
        is_within_area(screen_pos, self.pos, self.size)
    }

    pub fn hold_window(&mut self, screen_pos: Vec2) {
        if self.in_hold {
            return;
        }
        self.in_hold = true;
        self.hold_pos = screen_pos - self.pos;
    }

    pub fn release_window(&mut self) {
        self.in_hold = false;
    }

    pub fn set_z_order(&mut self, systems: &mut DrawSetting, z_order: f32) {
        if self.z_order == z_order {
            return;
        }
        self.z_order = z_order;
        let z_order_result = self.z_order * 0.01;

        let set_pos_z = ORDER_GUI_WINDOW - z_order_result;
        let pos = systems.gfx.get_pos(self.window);
        systems.gfx.set_pos(self.window, Vec3::new(pos.x, pos.y, set_pos_z));
        let pos = systems.gfx.get_pos(self.textbox_bg);
        let textbox_zpos = next_down(set_pos_z);
        systems.gfx.set_pos(self.textbox_bg, Vec3::new(pos.x, pos.y, textbox_zpos));
        let pos = systems.gfx.get_pos(self.chatarea_bg);
        systems.gfx.set_pos(self.chatarea_bg, Vec3::new(pos.x, pos.y, next_down(set_pos_z)));
        self.textbox.set_z_order(systems, next_down(textbox_zpos));
        self.button.iter_mut().for_each(|button| {
            button.set_z_order(systems, next_down(set_pos_z));
        });
        self.scrollbar.set_z_order(systems, next_down(set_pos_z));
    }

    pub fn move_window(&mut self, systems: &mut DrawSetting, screen_pos: Vec2) {
        if !self.in_hold {
            return;
        }
        self.pos = screen_pos - self.hold_pos;

        let pos = systems.gfx.get_pos(self.window);
        systems.gfx.set_pos(self.window, Vec3::new(self.pos.x, self.pos.y, pos.z));
        let pos = systems.gfx.get_pos(self.textbox_bg);
        systems.gfx.set_pos(self.textbox_bg, Vec3::new(self.pos.x + 5.0, self.pos.y + 5.0, pos.z));
        let pos = systems.gfx.get_pos(self.chatarea_bg);
        systems.gfx.set_pos(self.chatarea_bg, Vec3::new(self.pos.x + 5.0, self.pos.y + 34.0, pos.z));
        self.textbox.set_pos(systems, Vec2::new(self.pos.x + 7.0, self.pos.y + 7.0));
        self.button.iter_mut().for_each(|button| {
            button.set_pos(systems, self.pos);
        });
        self.scrollbar.set_pos(systems, self.pos);
    }

    pub fn hover_buttons(
        &mut self,
        systems: &mut DrawSetting,
        screen_pos: Vec2,
    ) {
        for button in self.button.iter_mut() {
            if is_within_area(screen_pos, 
                Vec2::new(button.base_pos.x + button.adjust_pos.x, 
                    button.base_pos.y + button.adjust_pos.y), button.size) {
                button.set_hover(systems, true);
            } else {
                button.set_hover(systems, false);
            }
        }
    }
    
    pub fn click_buttons(
        &mut self,
        systems: &mut DrawSetting,
        screen_pos: Vec2,
    ) -> Option<usize> {
        let mut button_found = None;
        for (index, button) in self.button.iter_mut().enumerate() {
            if is_within_area(screen_pos, 
                Vec2::new(button.base_pos.x + button.adjust_pos.x, 
                    button.base_pos.y + button.adjust_pos.y), button.size) {
                button.set_click(systems, true);
                button_found = Some(index)
            }
        }
        button_found
    }
    
    pub fn reset_buttons(
        &mut self,
        systems: &mut DrawSetting,
    ) {
        if !self.did_button_click {
            return;
        }
        self.did_button_click = false;
    
        self.button.iter_mut().for_each(|button| {
            button.set_click(systems, false);
        });
    }
}