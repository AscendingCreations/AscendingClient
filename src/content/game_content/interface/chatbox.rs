use cosmic_text::{Attrs, Metrics};
use graphics::*;

use crate::{
    data_types::*, is_within_area, logic::*, send_command, send_message,
    widget::*, Interface, Result, Socket, SystemHolder,
};

const MAX_CHAT_LINE: usize = 8;
const VISIBLE_SIZE: f32 = 160.0;
const MAX_CHAT: usize = 100;

#[derive(Debug, Clone)]
pub struct Chat {
    text: usize,
    msg: String,
    size: Vec2,
    adjust_y: f32,
    channel: MessageChannel,
}

#[derive(Clone, Copy, Debug)]
pub struct ChatTab {
    bg: usize,
    text: usize,
    base_pos: Vec2,
    adjust_pos: Vec2,
    size: Vec2,
    in_hover: bool,
    is_selected: bool,
}

impl ChatTab {
    fn new(
        systems: &mut SystemHolder,
        base_pos: Vec2,
        adjust_pos: Vec2,
        size: Vec2,
        z_order: [f32; 2],
        msg: String,
    ) -> Self {
        let pos = base_pos + adjust_pos;

        let mut bg_rect = Rect::new(&mut systems.renderer, 0);
        bg_rect
            .set_position(Vec3::new(pos.x, pos.y, z_order[0]))
            .set_size(size)
            .set_border_width(1.0)
            .set_color(Color::rgba(100, 100, 100, 255))
            .set_border_color(Color::rgba(40, 40, 40, 255));
        let bg = systems.gfx.add_rect(bg_rect, 0, "ChatTab BG".into());
        systems.gfx.set_visible(bg, true);

        let text_data = create_label(
            systems,
            Vec3::new(pos.x, pos.y, z_order[1]),
            Vec2::new(size.x, 20.0),
            Bounds::new(pos.x, pos.y, pos.x + size.x, pos.y + 20.0),
            Color::rgba(255, 255, 255, 255),
        );
        let text = systems.gfx.add_text(text_data, 1, "ChatTab Text".into());
        systems.gfx.set_text(&mut systems.renderer, text, &msg);
        systems.gfx.center_text(text);
        systems.gfx.set_visible(text, true);

        ChatTab {
            bg,
            text,
            base_pos,
            adjust_pos,
            size,
            in_hover: false,
            is_selected: false,
        }
    }

    fn in_area(&mut self, screen_pos: Vec2) -> bool {
        is_within_area(screen_pos, self.base_pos + self.adjust_pos, self.size)
    }

    fn set_hover(&mut self, systems: &mut SystemHolder, in_hover: bool) {
        if self.in_hover == in_hover {
            return;
        }
        self.in_hover = in_hover;
        if !self.is_selected {
            if self.in_hover {
                systems
                    .gfx
                    .set_color(self.bg, Color::rgba(150, 150, 150, 255));
            } else {
                systems
                    .gfx
                    .set_color(self.bg, Color::rgba(100, 100, 100, 255));
            }
        }
    }

    fn set_select(&mut self, systems: &mut SystemHolder, is_selected: bool) {
        if self.is_selected == is_selected {
            return;
        }
        self.is_selected = is_selected;
        if self.is_selected {
            systems.gfx.set_color(self.bg, Color::rgba(65, 65, 65, 255));
        } else if self.in_hover {
            systems
                .gfx
                .set_color(self.bg, Color::rgba(150, 150, 150, 255));
        } else {
            systems
                .gfx
                .set_color(self.bg, Color::rgba(100, 100, 100, 255));
        }
    }

    fn unload(&mut self, systems: &mut SystemHolder) {
        systems.gfx.remove_gfx(&mut systems.renderer, self.bg);
        systems.gfx.remove_gfx(&mut systems.renderer, self.text);
    }

    fn set_z_order(&mut self, systems: &mut SystemHolder, z_order: [f32; 2]) {
        let pos = systems.gfx.get_pos(self.bg);
        systems
            .gfx
            .set_pos(self.bg, Vec3::new(pos.x, pos.y, z_order[0]));

        let pos = systems.gfx.get_pos(self.text);
        systems
            .gfx
            .set_pos(self.text, Vec3::new(pos.x, pos.y, z_order[1]));
    }

    fn move_pos(&mut self, systems: &mut SystemHolder, newpos: Vec2) {
        self.base_pos = newpos;
        let set_pos = newpos + self.adjust_pos;

        let pos = systems.gfx.get_pos(self.bg);
        systems
            .gfx
            .set_pos(self.bg, Vec3::new(set_pos.x, set_pos.y, pos.z));

        let pos = systems.gfx.get_pos(self.text);
        systems
            .gfx
            .set_pos(self.text, Vec3::new(set_pos.x, set_pos.y, pos.z));
        systems.gfx.set_bound(
            self.text,
            Bounds::new(
                set_pos.x,
                set_pos.y,
                set_pos.x + self.size.x,
                set_pos.y + 20.0,
            ),
        );
        systems.gfx.center_text(self.text);
    }
}

pub struct Chatbox {
    window: usize,
    textbox_bg: usize,
    chatarea_bg: usize,
    pub textbox: Textbox,
    button: [Button; 3],
    pub did_button_click: bool,
    pub scrollbar: Scrollbar,
    pub chat_tab: [ChatTab; 3],
    msg_selection: usize,
    msg_select_index: Option<usize>,
    pub selected_tab: usize,

    chat: Vec<Chat>,
    chat_areasize: Vec2,
    chat_zorder: f32,
    chat_bounds: Bounds,
    chat_line_size: f32,
    chat_scroll_value: usize,

    pub pos: Vec2,
    pub size: Vec2,
    pub z_order: f32,
    pub order_index: usize,
    in_hold: bool,
    hold_pos: Vec2,

    min_bound: Vec2,
    max_bound: Vec2,
}

impl Chatbox {
    pub fn new(systems: &mut SystemHolder) -> Self {
        let w_pos = Vec3::new(10.0, 10.0, ORDER_GUI_WINDOW);
        let w_size = Vec2::new(350.0, 200.0);

        let detail_1 = w_pos.z.sub_f32(0.001, 3);
        let detail_2 = w_pos.z.sub_f32(0.002, 3);
        let detail_3 = w_pos.z.sub_f32(0.003, 3);

        let mut window_rect = Rect::new(&mut systems.renderer, 0);
        window_rect
            .set_position(w_pos)
            .set_size(w_size)
            .set_color(Color::rgba(120, 120, 120, 255))
            .set_border_width(1.0)
            .set_border_color(Color::rgba(40, 40, 40, 255));
        let window = systems.gfx.add_rect(window_rect, 0, "Chatbox BG".into());

        let mut textbox_rect = Rect::new(&mut systems.renderer, 0);
        let textbox_zpos = detail_1;
        textbox_rect
            .set_position(Vec3::new(w_pos.x + 5.0, w_pos.y + 5.0, textbox_zpos))
            .set_size(Vec2::new(w_size.x - 75.0, 24.0))
            .set_color(Color::rgba(80, 80, 80, 255));
        let textbox_bg =
            systems
                .gfx
                .add_rect(textbox_rect, 0, "Chatbox Textbox BG".into());

        let mut chatarea_rect = Rect::new(&mut systems.renderer, 0);
        let chatarea_zorder = detail_1;
        let chat_area_pos = Vec2::new(w_pos.x + 5.0, w_pos.y + 34.0);
        let chat_areasize = Vec2::new(w_size.x - 39.0, w_size.y - 39.0);
        chatarea_rect
            .set_position(Vec3::new(
                chat_area_pos.x,
                chat_area_pos.y,
                chatarea_zorder,
            ))
            .set_size(chat_areasize)
            .set_color(Color::rgba(140, 140, 140, 255));
        let chatarea_bg =
            systems
                .gfx
                .add_rect(chatarea_rect, 0, "Chatbox Chat Area".into());
        let chat_zorder = detail_3;
        let chat_bounds = Bounds::new(
            chat_area_pos.x,
            chat_area_pos.y,
            chat_area_pos.x + chat_areasize.x,
            chat_area_pos.y + chat_areasize.y,
        );

        let textbox = Textbox::new(
            systems,
            Vec3::new(w_pos.x, w_pos.y, detail_2),
            Vec2::new(7.0, 7.0),
            (0.0001, 5),
            Vec2::new(w_size.x - 79.0, 20.0),
            Color::rgba(200, 200, 200, 255),
            1,
            255,
            Color::rgba(120, 120, 120, 255),
            Color::rgba(10, 10, 150, 255),
            false,
            true,
            None,
            vec![],
        );

        let button = [
            Button::new(
                systems,
                ButtonType::Rect(ButtonRect {
                    rect_color: Color::rgba(80, 80, 80, 255),
                    got_border: false,
                    border_color: Color::rgba(0, 0, 0, 0),
                    border_radius: 0.0,
                    hover_change: ButtonChangeType::ColorChange(Color::rgba(
                        150, 150, 150, 255,
                    )),
                    click_change: ButtonChangeType::ColorChange(Color::rgba(
                        80, 80, 80, 255,
                    )),
                }),
                ButtonContentType::Image(ButtonContentImg {
                    res: systems.resource.vertical_arrow.allocation,
                    pos: Vec2::new(0.0, 0.0),
                    uv: Vec2::new(0.0, 0.0),
                    size: Vec2::new(24.0, 24.0),
                    hover_change: ButtonChangeType::None,
                    click_change: ButtonChangeType::None,
                }),
                Vec2::new(w_pos.x, w_pos.y),
                Vec2::new(w_size.x - 29.0, w_size.y - 29.0),
                detail_1,
                (0.0001, 5),
                Vec2::new(24.0, 24.0),
                0,
                true,
                None,
            ),
            Button::new(
                systems,
                ButtonType::Rect(ButtonRect {
                    rect_color: Color::rgba(80, 80, 80, 255),
                    got_border: false,
                    border_color: Color::rgba(0, 0, 0, 0),
                    border_radius: 0.0,
                    hover_change: ButtonChangeType::ColorChange(Color::rgba(
                        150, 150, 150, 255,
                    )),
                    click_change: ButtonChangeType::ColorChange(Color::rgba(
                        80, 80, 80, 255,
                    )),
                }),
                ButtonContentType::Image(ButtonContentImg {
                    res: systems.resource.vertical_arrow.allocation,
                    pos: Vec2::new(0.0, 0.0),
                    uv: Vec2::new(24.0, 0.0),
                    size: Vec2::new(24.0, 24.0),
                    hover_change: ButtonChangeType::None,
                    click_change: ButtonChangeType::None,
                }),
                Vec2::new(w_pos.x, w_pos.y),
                Vec2::new(w_size.x - 29.0, 34.0),
                detail_1,
                (0.0001, 5),
                Vec2::new(24.0, 24.0),
                0,
                true,
                None,
            ),
            Button::new(
                systems,
                ButtonType::Rect(ButtonRect {
                    rect_color: Color::rgba(80, 80, 80, 255),
                    got_border: false,
                    border_color: Color::rgba(0, 0, 0, 0),
                    border_radius: 0.0,
                    hover_change: ButtonChangeType::ColorChange(Color::rgba(
                        150, 150, 150, 255,
                    )),
                    click_change: ButtonChangeType::ColorChange(Color::rgba(
                        80, 80, 80, 255,
                    )),
                }),
                ButtonContentType::Text(ButtonContentText {
                    text: "Send".to_string(),
                    pos: Vec2::new(0.0, 2.0),
                    color: Color::rgba(255, 255, 255, 255),
                    render_layer: 1,
                    hover_change: ButtonChangeType::ColorChange(Color::rgba(
                        255, 255, 255, 255,
                    )),
                    click_change: ButtonChangeType::ColorChange(Color::rgba(
                        255, 255, 255, 255,
                    )),
                }),
                Vec2::new(w_pos.x, w_pos.y),
                Vec2::new(w_size.x - 65.0, 5.0),
                detail_1,
                (0.0001, 5),
                Vec2::new(60.0, 24.0),
                0,
                true,
                None,
            ),
        ];

        let scrollbar = Scrollbar::new(
            systems,
            Vec2::new(w_pos.x, w_pos.y),
            Vec2::new(w_size.x - 28.0, 63.0),
            w_size.y - 97.0,
            22.0,
            true,
            detail_1,
            (0.0001, 5),
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
            0,
            20.0,
            true,
            true,
            None,
        );

        let mut chat_tab = [
            ChatTab::new(
                systems,
                Vec2::new(w_pos.x, w_pos.y),
                Vec2::new(0.0, w_size.y - 1.0),
                Vec2::new(70.0, 24.0),
                [w_pos.z, detail_1],
                "All".into(),
            ),
            ChatTab::new(
                systems,
                Vec2::new(w_pos.x, w_pos.y),
                Vec2::new(69.0, w_size.y - 1.0),
                Vec2::new(70.0, 24.0),
                [w_pos.z, detail_1],
                "Map".into(),
            ),
            ChatTab::new(
                systems,
                Vec2::new(w_pos.x, w_pos.y),
                Vec2::new(138.0, w_size.y - 1.0),
                Vec2::new(70.0, 24.0),
                [w_pos.z, detail_1],
                "Global".into(),
            ),
        ];
        chat_tab[0].set_select(systems, false);

        let mut selection_rect = Rect::new(&mut systems.renderer, 0);
        selection_rect
            .set_position(Vec3::new(0.0, 0.0, detail_3))
            .set_size(Vec2::new(0.0, 0.0))
            .set_color(Color::rgba(60, 60, 60, 255));
        let msg_selection =
            systems
                .gfx
                .add_rect(selection_rect, 0, "Chatbox Selection".into());
        systems.gfx.set_visible(msg_selection, false);

        Chatbox {
            window,
            textbox_bg,
            chatarea_bg,
            textbox,
            button,
            did_button_click: false,
            scrollbar,
            chat: Vec::new(),
            chat_areasize,
            chat_zorder,
            chat_bounds,
            chat_line_size: 0.0,
            chat_scroll_value: 0,
            pos: Vec2::new(w_pos.x, w_pos.y),
            size: w_size,
            z_order: w_pos.z,
            order_index: 0,
            in_hold: false,
            hold_pos: Vec2::new(0.0, 0.0),
            chat_tab,
            selected_tab: 0,
            msg_selection,
            msg_select_index: None,

            min_bound: Vec2::new(
                systems.size.width - w_size.x,
                systems.size.height - w_size.y,
            ),
            max_bound: Vec2::new(0.0, 0.0),
        }
    }

    pub fn unload(&mut self, systems: &mut SystemHolder) {
        systems.gfx.remove_gfx(&mut systems.renderer, self.window);
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, self.textbox_bg);
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, self.chatarea_bg);
        self.textbox.unload(systems);
        self.button.iter_mut().for_each(|button| {
            button.unload(systems);
        });
        self.chat.iter().for_each(|chat| {
            systems.gfx.remove_gfx(&mut systems.renderer, chat.text);
        });
        self.scrollbar.unload(systems);
        self.chat_tab.iter_mut().for_each(|tab| {
            tab.unload(systems);
        });
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, self.msg_selection);
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
        if is_within_area(
            screen_pos,
            Vec2::new(self.textbox.base_pos.x, self.textbox.base_pos.y)
                + self.textbox.adjust_pos,
            self.textbox.size,
        ) {
            return false;
        }
        if !is_within_area(screen_pos, self.pos, self.size) {
            return false;
        }
        true
    }

    pub fn in_window(&mut self, screen_pos: Vec2) -> bool {
        let chatbox_size = self.size + Vec2::new(0.0, 24.0);
        is_within_area(screen_pos, self.pos, chatbox_size)
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

    pub fn set_z_order(
        &mut self,
        systems: &mut SystemHolder,
        z_order: f32,
        order_index: usize,
    ) {
        if self.z_order == z_order {
            return;
        }
        self.z_order = z_order;
        self.order_index = order_index;

        let detail_origin = ORDER_GUI_WINDOW.sub_f32(self.z_order, 3);
        let detail_1 = detail_origin.sub_f32(0.001, 3);
        let detail_2 = detail_origin.sub_f32(0.002, 3);
        let detail_3 = detail_origin.sub_f32(0.002, 3);

        let pos = systems.gfx.get_pos(self.window);
        systems
            .gfx
            .set_pos(self.window, Vec3::new(pos.x, pos.y, detail_origin));
        let pos = systems.gfx.get_pos(self.textbox_bg);
        systems
            .gfx
            .set_pos(self.textbox_bg, Vec3::new(pos.x, pos.y, detail_1));
        let pos = systems.gfx.get_pos(self.chatarea_bg);
        self.chat_zorder = detail_3;
        systems
            .gfx
            .set_pos(self.chatarea_bg, Vec3::new(pos.x, pos.y, detail_1));
        self.textbox.set_z_order(systems, detail_2);
        self.button.iter_mut().for_each(|button| {
            button.set_z_order(systems, detail_1);
        });
        self.scrollbar.set_z_order(systems, detail_1);

        for chat in self.chat.iter() {
            let pos = systems.gfx.get_pos(chat.text);
            systems
                .gfx
                .set_pos(chat.text, Vec3::new(pos.x, pos.y, self.chat_zorder));
        }

        let pos = systems.gfx.get_pos(self.msg_selection);
        systems
            .gfx
            .set_pos(self.msg_selection, Vec3::new(pos.x, pos.y, detail_2));

        self.chat_tab.iter_mut().for_each(|tab| {
            tab.set_z_order(systems, [detail_origin, detail_1]);
        })
    }

    pub fn move_window(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
    ) {
        if !self.in_hold {
            return;
        }
        self.pos = (screen_pos - self.hold_pos)
            .max(self.max_bound)
            .min(self.min_bound);

        let pos = systems.gfx.get_pos(self.window);
        systems
            .gfx
            .set_pos(self.window, Vec3::new(self.pos.x, self.pos.y, pos.z));
        let pos = systems.gfx.get_pos(self.textbox_bg);
        systems.gfx.set_pos(
            self.textbox_bg,
            Vec3::new(self.pos.x + 5.0, self.pos.y + 5.0, pos.z),
        );
        let pos = systems.gfx.get_pos(self.chatarea_bg);
        let chat_area_pos = Vec2::new(self.pos.x + 5.0, self.pos.y + 34.0);
        self.chat_bounds = Bounds::new(
            chat_area_pos.x,
            chat_area_pos.y,
            chat_area_pos.x + self.chat_areasize.x,
            chat_area_pos.y + self.chat_areasize.y,
        );
        systems.gfx.set_pos(
            self.chatarea_bg,
            Vec3::new(chat_area_pos.x, chat_area_pos.y, pos.z),
        );
        self.textbox.set_pos(systems, self.pos);
        self.button.iter_mut().for_each(|button| {
            button.set_pos(systems, self.pos);
        });
        self.scrollbar.set_pos(systems, self.pos);

        let scroll_y = self.chat_scroll_value * 16;
        for data in self.chat.iter_mut() {
            let start_pos = Vec2::new(
                self.chat_bounds.left,
                self.chat_bounds.bottom - self.chat_areasize.y,
            );
            systems.gfx.set_pos(
                data.text,
                Vec3::new(
                    start_pos.x,
                    (start_pos.y + 2.0 + data.adjust_y) - scroll_y as f32,
                    self.chat_zorder,
                ),
            );
            systems.gfx.set_bound(data.text, self.chat_bounds);
        }

        let pos = systems.gfx.get_pos(self.msg_selection);
        systems
            .gfx
            .set_pos(self.msg_selection, Vec3::new(0.0, 0.0, pos.z));
        systems.gfx.set_visible(self.msg_selection, false);

        self.chat_tab.iter_mut().for_each(|tab| {
            tab.move_pos(systems, self.pos);
        })
    }

    pub fn hover_scrollbar(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
    ) {
        if self.order_index != 0 {
            return;
        }

        if self.scrollbar.in_scroll(screen_pos) {
            self.scrollbar.set_hover(systems, true);
        } else {
            self.scrollbar.set_hover(systems, false);
        }
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

        for tab in self.chat_tab.iter_mut() {
            let in_area = tab.in_area(screen_pos);
            tab.set_hover(systems, in_area);
        }
    }

    pub fn hover_msg(&mut self, systems: &mut SystemHolder, screen_pos: Vec2) {
        let mut got_index = None;
        for (index, chat) in self.chat.iter().enumerate() {
            if can_channel_show(chat.channel, self.selected_tab)
                && is_within_area(
                    screen_pos,
                    Vec2::new(self.chat_bounds.left, self.chat_bounds.bottom),
                    self.chat_areasize,
                )
            {
                let scroll_y = self.chat_scroll_value * 16;
                let start_pos = Vec2::new(
                    self.chat_bounds.left,
                    self.chat_bounds.bottom - chat.size.y,
                );
                let pos = Vec2::new(
                    start_pos.x,
                    (start_pos.y + 2.0 + chat.adjust_y) - scroll_y as f32,
                );
                if is_within_area(screen_pos, pos, chat.size) {
                    got_index = Some((index, pos));
                    break;
                }
            }
        }

        if let Some((index, mut pos)) = got_index {
            let new_y = pos.y.max(self.chat_bounds.bottom);
            let adjust_size_y =
                if new_y != pos.y { new_y - pos.y } else { 0.0 };
            pos.y = pos.y.max(self.chat_bounds.bottom);

            let mut size = self.chat[index].size;
            size.y -= adjust_size_y;
            size.y = size
                .y
                .min((self.chat_bounds.bottom + self.chat_areasize.y) - pos.y);

            systems.gfx.set_visible(self.msg_selection, true);

            let curpos = systems.gfx.get_pos(self.msg_selection);
            systems
                .gfx
                .set_pos(self.msg_selection, Vec3::new(pos.x, pos.y, curpos.z));
            systems.gfx.set_size(self.msg_selection, size);

            self.msg_select_index = Some(index);
        } else {
            systems.gfx.set_visible(self.msg_selection, false);

            self.msg_select_index = None;
        }
    }

    pub fn get_selected_msg(&mut self) -> Option<String> {
        if let Some(index) = self.msg_select_index {
            if let Some(chatdata) = self.chat.get(index) {
                return Some(chatdata.msg.clone());
            }
        }
        None
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

    pub fn select_chat_tab(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
    ) {
        let mut selected_tab = self.selected_tab;
        for (index, tab) in self.chat_tab.iter_mut().enumerate() {
            if tab.in_area(screen_pos) {
                selected_tab = index;
                break;
            }
        }
        if selected_tab != self.selected_tab {
            self.chat_tab[self.selected_tab].set_select(systems, false);
            self.chat_tab[selected_tab].set_select(systems, true);
            self.selected_tab = selected_tab;
            self.switch_tab(systems);
        }
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

    pub fn set_chat_scrollbar(
        &mut self,
        systems: &mut SystemHolder,
        force: bool,
    ) {
        if self.scrollbar.value == self.chat_scroll_value {
            return;
        }
        if !force && !self.scrollbar.in_hold {
            return;
        }
        self.chat_scroll_value = self.scrollbar.value;
        let scroll_y = self.chat_scroll_value * 16;

        for data in self.chat.iter_mut() {
            let start_pos = Vec2::new(
                self.chat_bounds.left,
                self.chat_bounds.bottom - self.chat_areasize.y,
            );
            systems.gfx.set_pos(
                data.text,
                Vec3::new(
                    start_pos.x,
                    (start_pos.y + 2.0 + data.adjust_y) - scroll_y as f32,
                    self.chat_zorder,
                ),
            );
        }
    }

    pub fn add_chat(
        &mut self,
        systems: &mut SystemHolder,
        msg: (String, Color),
        header_msg: Option<(String, Color)>,
        channel: MessageChannel,
    ) {
        let mut text_data = create_label(
            systems,
            Vec3::new(0.0, 0.0, 0.0),
            self.chat_areasize,
            self.chat_bounds,
            Color::rgba(255, 255, 255, 255),
        );
        text_data
            .set_buffer_size(
                &mut systems.renderer,
                self.chat_areasize.x as i32,
                self.chat_areasize.y as i32,
            )
            .set_wrap(&mut systems.renderer, cosmic_text::Wrap::Word);

        let text = systems.gfx.add_text(text_data, 1, "Chatbox Text".into());
        let msg_color = Attrs::new().color(msg.1);

        let msg = if let Some(header) = header_msg {
            let header_color = Attrs::new().color(header.1);
            systems.gfx.set_rich_text(
                &mut systems.renderer,
                text,
                [
                    (header.0.as_str(), header_color),
                    (msg.0.as_str(), msg_color),
                ],
            );
            format!("{}{}", header.0, msg.0)
        } else {
            systems.gfx.set_rich_text(
                &mut systems.renderer,
                text,
                [(msg.0.as_str(), msg_color)],
            );
            msg.0
        };
        let size = systems.gfx.get_measure(text);

        let chat = Chat {
            text,
            msg,
            size,
            adjust_y: size.y,
            channel,
        };

        systems.gfx.set_visible(
            chat.text,
            can_channel_show(channel, self.selected_tab),
        );

        if self.chat.len() >= MAX_CHAT {
            if let Some(chat) = self.chat.pop() {
                if can_channel_show(chat.channel, self.selected_tab) {
                    self.chat_line_size -= chat.size.y
                }
            }
        }

        if can_channel_show(channel, self.selected_tab) {
            let start_pos = Vec2::new(
                self.chat_bounds.left,
                self.chat_bounds.bottom - self.chat_areasize.y,
            );
            systems.gfx.set_pos(
                text,
                Vec3::new(
                    start_pos.x,
                    start_pos.y + 2.0 + size.y,
                    self.chat_zorder,
                ),
            );

            for data in self.chat.iter_mut() {
                if can_channel_show(data.channel, self.selected_tab) {
                    data.adjust_y += size.y;
                    systems.gfx.set_pos(
                        data.text,
                        Vec3::new(
                            start_pos.x,
                            start_pos.y + 2.0 + data.adjust_y,
                            self.chat_zorder,
                        ),
                    );
                }
            }
        }

        self.chat.insert(0, chat);

        if can_channel_show(channel, self.selected_tab) {
            self.chat_line_size += size.y;
            let leftover = self.chat_line_size - VISIBLE_SIZE;
            if leftover > 0.0 {
                self.scrollbar
                    .set_max_value(systems, (leftover / 16.0).floor() as usize);
                self.scrollbar.set_value(systems, 0);
            }
        }
    }

    pub fn switch_tab(&mut self, systems: &mut SystemHolder) {
        let start_pos = Vec2::new(
            self.chat_bounds.left,
            self.chat_bounds.bottom - self.chat_areasize.y,
        );

        let mut chat_line_size = 0.0;
        let mut add_y = 0.0;
        for data in self.chat.iter_mut() {
            if can_channel_show(data.channel, self.selected_tab) {
                systems.gfx.set_visible(data.text, true);

                data.adjust_y = data.size.y + add_y;

                systems.gfx.set_pos(
                    data.text,
                    Vec3::new(
                        start_pos.x,
                        start_pos.y + 2.0 + data.adjust_y,
                        self.chat_zorder,
                    ),
                );

                chat_line_size += data.size.y;
                add_y += data.size.y;
            } else {
                systems.gfx.set_visible(data.text, false);
            }
        }
        self.chat_line_size = chat_line_size;
        let leftover = self.chat_line_size - VISIBLE_SIZE;
        if leftover > 0.0 {
            self.scrollbar
                .set_max_value(systems, (leftover / 16.0).floor() as usize);
            self.scrollbar.set_value(systems, 0);
        } else if self.scrollbar.max_value > 0 {
            self.scrollbar.set_value(systems, 0);
            self.scrollbar.set_max_value(systems, 0);
        }
        self.chat_scroll_value = self.scrollbar.value;
    }
}

pub fn can_channel_show(channel: MessageChannel, selected_tab: usize) -> bool {
    match channel {
        MessageChannel::Global => selected_tab == 2 || selected_tab == 0,
        MessageChannel::Map => selected_tab == 1 || selected_tab == 0,
        _ => selected_tab == 0,
    }
}

pub fn send_chat(
    interface: &mut Interface,
    systems: &mut SystemHolder,
    socket: &mut Socket,
) -> Result<()> {
    let input_string = interface.chatbox.textbox.text.clone();
    if input_string.is_empty() {
        return Ok(());
    }

    if let Some(char) = input_string.chars().next() {
        match char {
            '@' => {
                let msg = &input_string[1..];
                if let Some(index) = msg.find(' ') {
                    let (name, message) = msg.split_at(index);
                    send_message(
                        socket,
                        crate::MessageChannel::Private,
                        message.into(),
                        name.into(),
                    )?;
                } else {
                    interface.chatbox.add_chat(
                        systems,
                        ("Invalid Command".into(), COLOR_WHITE),
                        None,
                        crate::MessageChannel::Map,
                    );
                }
            }
            '/' => {
                let msg = &input_string[1..];
                match msg {
                    "trade" => {
                        send_command(socket, crate::Command::Trade)?;
                    }
                    _ => {
                        interface.chatbox.add_chat(
                            systems,
                            ("Invalid Command".into(), COLOR_WHITE),
                            None,
                            crate::MessageChannel::Map,
                        );
                    }
                }
            }
            _ => {
                let channel = match interface.chatbox.selected_tab {
                    2 => crate::MessageChannel::Global,
                    _ => crate::MessageChannel::Map,
                };
                send_message(socket, channel, input_string, String::new())?;
            }
        }
    }

    interface.chatbox.textbox.set_text(systems, String::new());

    Ok(())
}
