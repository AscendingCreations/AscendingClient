use std::collections::VecDeque;

use cosmic_text::{Attrs, Metrics};
use graphics::*;

use input::Key;
use winit::keyboard::NamedKey;

use crate::{
    Alert, AlertIndex, AlertType, COLOR_WHITE, GameContent, GfxType,
    MouseInputType, Result, SystemHolder, TradeStatus, World,
    interface::chatbox::*, is_within_area, send_buyitem, send_closeshop,
    send_closestorage, send_closetrade, send_command, send_message,
    send_removetradeitem, send_submittrade, send_unequip,
    send_updatetrademoney, send_useitem, socket, systems::Poller, widget::*,
};

pub mod chatbox;
mod inventory;
mod item_description;
mod profile;
mod screen;
mod setting;
mod shop;
mod storage;
mod trade;

pub use chatbox::*;
use inventory::*;
use item_description::*;
pub use profile::*;
use screen::*;
use setting::*;
use shop::*;
use storage::*;
use trade::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Window {
    Inventory,
    Profile,
    Setting,
    Chatbox,
    Storage,
    Shop,
    Trade,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SelectedTextbox {
    None,
    Chatbox,
    Trade,
}

pub struct Interface {
    pub ping_text: GfxType,
    pub average_ping: GfxType,
    pub frame_loop: GfxType,
    pub average_ping_collection: VecDeque<u64>,
    pub frame_loop_collection: VecDeque<u64>,
    pub menu_button: [Button; 3],
    pub vitalbar: VitalBar,
    did_button_click: bool,
    pub inventory: Inventory,
    pub storage: Storage,
    pub shop: Shop,
    pub trade: Trade,
    pub profile: Profile,
    pub item_desc: ItemDescription,
    setting: Setting,
    pub chatbox: Chatbox,
    window_order: Vec<(Window, usize)>,
    drag_window: Option<Window>,
    pub selected_textbox: SelectedTextbox,
}

impl Interface {
    pub fn new(systems: &mut SystemHolder) -> Self {
        let menu_button = create_menu_button(systems);
        let size = (Vec2::new(150.0, 20.0) * systems.scale as f32).floor();
        let statistic_pos = Vec3::new(
            systems.size.width - size.x,
            systems.size.height - size.y - 30.0 * systems.scale as f32,
            0.0,
        );
        let ping = create_label(
            systems,
            statistic_pos,
            size,
            Bounds::new(
                statistic_pos.x,
                statistic_pos.y,
                statistic_pos.x + size.x,
                statistic_pos.y + size.y,
            ),
            Color::rgba(200, 200, 200, 255),
        );
        let ping_text = systems.gfx.add_text(
            ping,
            5,
            "Ping".to_string(),
            systems.config.show_ping,
        );

        systems
            .gfx
            .set_text(&mut systems.renderer, &ping_text, "Ping: 0");

        let pos = Vec3::new(
            statistic_pos.x,
            statistic_pos.y - 25.0 * systems.scale as f32,
            statistic_pos.z,
        );
        let averageping = create_label(
            systems,
            pos,
            size,
            Bounds::new(pos.x, pos.y, pos.x + size.x, pos.y + size.y),
            Color::rgba(200, 200, 200, 255),
        );
        let average_ping = systems.gfx.add_text(
            averageping,
            5,
            "Av. Ping".to_string(),
            systems.config.show_average_ping,
        );

        systems.gfx.set_text(
            &mut systems.renderer,
            &average_ping,
            "Av. Ping: 0",
        );

        let pos = Vec3::new(
            statistic_pos.x,
            statistic_pos.y - 50.0 * systems.scale as f32,
            statistic_pos.z,
        );
        let framejitter = create_label(
            systems,
            pos,
            size,
            Bounds::new(pos.x, pos.y, pos.x + size.x, pos.y + size.y),
            Color::rgba(200, 200, 200, 255),
        );
        let frame_loop = systems.gfx.add_text(
            framejitter,
            5,
            "Av. Ping".to_string(),
            systems.config.show_frame_loop,
        );

        systems.gfx.set_text(
            &mut systems.renderer,
            &frame_loop,
            "Frame Jitter: 0",
        );

        let mut interface = Interface {
            menu_button,
            ping_text,
            average_ping,
            frame_loop,
            average_ping_collection: VecDeque::with_capacity(20),
            frame_loop_collection: VecDeque::with_capacity(20),
            vitalbar: VitalBar::new(systems),
            did_button_click: false,
            inventory: Inventory::new(systems),
            storage: Storage::new(systems),
            shop: Shop::new(systems),
            trade: Trade::new(systems),
            profile: Profile::new(systems),
            setting: Setting::new(systems),
            chatbox: Chatbox::new(systems),
            item_desc: ItemDescription::new(systems),
            window_order: Vec::with_capacity(7),
            drag_window: None,
            selected_textbox: SelectedTextbox::None,
        };

        interface.add_window_order();
        interface
    }

    pub fn add_window_order(&mut self) {
        self.window_order.push((Window::Chatbox, 0));
        self.window_order.push((Window::Inventory, 1));
        self.window_order.push((Window::Profile, 2));
        self.window_order.push((Window::Setting, 3));
        self.window_order.push((Window::Storage, 4));
        self.window_order.push((Window::Shop, 5));
        self.window_order.push((Window::Trade, 6));
        self.window_order.sort_by(|a, b| a.1.cmp(&b.1));
    }

    pub fn recreate(&mut self, systems: &mut SystemHolder) {
        self.menu_button = create_menu_button(systems);
        self.vitalbar = VitalBar::new(systems);
        self.inventory = Inventory::new(systems);
        self.profile = Profile::new(systems);
        self.setting = Setting::new(systems);
        self.chatbox = Chatbox::new(systems);
        self.storage = Storage::new(systems);
        self.shop = Shop::new(systems);
        self.trade = Trade::new(systems);
        self.item_desc = ItemDescription::new(systems);
        self.add_window_order();
        self.did_button_click = false;
        self.drag_window = None;
        self.selected_textbox = SelectedTextbox::None;
        systems
            .gfx
            .set_visible(&self.ping_text, systems.config.show_ping);
        systems
            .gfx
            .set_visible(&self.average_ping, systems.config.show_average_ping);
        systems
            .gfx
            .set_visible(&self.frame_loop, systems.config.show_frame_loop);
    }

    pub fn unload(&mut self, systems: &mut SystemHolder) {
        self.menu_button.iter_mut().for_each(|button| {
            button.unload(systems);
        });
        self.vitalbar.unload(systems);
        self.inventory.unload(systems);
        self.profile.unload(systems);
        self.setting.unload(systems);
        self.chatbox.unload(systems);
        self.storage.unload(systems);
        self.shop.unload(systems);
        self.trade.unload(systems);
        self.window_order.clear();
        self.item_desc.unload(systems);
        systems.gfx.set_visible(&self.ping_text, false);
        systems.gfx.set_visible(&self.average_ping, false);
        systems.gfx.set_visible(&self.frame_loop, false);
    }

    pub fn mouse_input(
        interface: &mut Interface,
        _world: &mut World,
        systems: &mut SystemHolder,
        socket: &mut Poller,
        alert: &mut Alert,
        input_type: MouseInputType,
        tooltip: &mut Tooltip,
        screen_pos: Vec2,
    ) -> Result<bool> {
        let mut result = false;

        match input_type {
            MouseInputType::MouseMove => {
                let mut can_hover: bool = true;

                for window in interface.window_order.iter() {
                    if can_hover {
                        match window.0 {
                            Window::Chatbox => {
                                interface
                                    .chatbox
                                    .hover_buttons(systems, screen_pos);
                                interface
                                    .chatbox
                                    .hover_msg(systems, screen_pos);
                                interface
                                    .chatbox
                                    .hover_scrollbar(systems, screen_pos);
                                can_hover = !interface
                                    .chatbox
                                    .in_window(screen_pos, systems);
                            }
                            Window::Inventory => {
                                interface
                                    .inventory
                                    .hover_buttons(systems, screen_pos);
                                interface.inventory.hover_data(
                                    systems,
                                    screen_pos,
                                    &mut interface.item_desc,
                                );
                                can_hover =
                                    !interface.inventory.in_window(screen_pos);
                            }
                            Window::Profile => {
                                interface
                                    .profile
                                    .hover_buttons(systems, screen_pos);
                                interface.profile.hover_data(
                                    systems,
                                    screen_pos,
                                    &mut interface.item_desc,
                                );
                                can_hover =
                                    !interface.profile.in_window(screen_pos);
                            }
                            Window::Setting => {
                                interface
                                    .setting
                                    .hover_buttons(systems, screen_pos);
                                interface
                                    .setting
                                    .hover_scrollbar(systems, screen_pos);
                                interface.setting.hover_checkbox(
                                    systems, tooltip, screen_pos,
                                );
                                can_hover =
                                    !interface.setting.in_window(screen_pos);
                            }
                            Window::Shop => {
                                interface
                                    .shop
                                    .hover_buttons(systems, screen_pos);
                                interface
                                    .shop
                                    .hover_scrollbar(systems, screen_pos);
                                interface.shop.hover_data(
                                    systems,
                                    screen_pos,
                                    &mut interface.item_desc,
                                );
                                can_hover =
                                    !interface.shop.in_window(screen_pos);
                            }
                            Window::Storage => {
                                interface
                                    .storage
                                    .hover_buttons(systems, screen_pos);
                                interface.storage.hover_data(
                                    systems,
                                    screen_pos,
                                    &mut interface.item_desc,
                                );
                                can_hover =
                                    !interface.storage.in_window(screen_pos);
                            }
                            Window::Trade => {
                                interface
                                    .trade
                                    .hover_buttons(systems, screen_pos);
                                interface.trade.hover_data(
                                    systems,
                                    screen_pos,
                                    &mut interface.item_desc,
                                );
                                can_hover =
                                    !interface.trade.in_window(screen_pos);
                            }
                        }
                    }
                }

                if can_hover {
                    Interface::hover_buttons(interface, systems, screen_pos);
                }
            }
            MouseInputType::MouseDoubleLeftDown => {
                if interface.inventory.visible
                    && interface.inventory.order_index == 0
                    && let Some(slot) = interface
                        .inventory
                        .find_inv_slot(systems, screen_pos, false)
                {
                    send_useitem(socket, slot as u16)?;
                }

                if interface.profile.visible
                    && interface.profile.order_index == 0
                    && let Some(slot) = interface
                        .profile
                        .find_eq_slot(systems, screen_pos, false)
                {
                    send_unequip(socket, slot as u16)?;
                }

                if interface.chatbox.order_index == 0
                    && let Some(text) = interface.chatbox.get_selected_msg()
                {
                    set_clipboard_text(text);
                }
            }
            MouseInputType::MouseLeftDown => {
                result = interface
                    .click_window_buttons(systems, socket, screen_pos, alert)?;

                if !result {
                    let button_index = Interface::click_buttons(
                        interface, systems, screen_pos,
                    );
                    if let Some(index) = button_index {
                        interface.did_button_click = true;
                        trigger_button(interface, systems, index);
                        result = true;
                    }
                }
            }
            MouseInputType::MouseLeftDownMove => {
                if interface.item_desc.visible {
                    interface.item_desc.set_visible(systems, false);
                }

                if let Some(slot) = interface.inventory.hold_slot {
                    interface
                        .inventory
                        .move_inv_slot(systems, slot, screen_pos);

                    let window =
                        find_window(systems, interface, screen_pos, None);
                    if let Some(result_window) = window {
                        match result_window {
                            Window::Storage
                            | Window::Inventory
                            | Window::Shop
                            | Window::Trade => {
                                hold_interface(
                                    interface,
                                    systems,
                                    result_window,
                                    screen_pos,
                                    false,
                                    true,
                                );
                            }
                            _ => {}
                        }
                    }

                    return Ok(true);
                }
                if let Some(slot) = interface.storage.hold_slot {
                    interface
                        .storage
                        .move_storage_slot(systems, slot, screen_pos);

                    let window =
                        find_window(systems, interface, screen_pos, None);

                    if let Some(result_window) = window {
                        match result_window {
                            Window::Storage
                            | Window::Inventory
                            | Window::Trade => {
                                hold_interface(
                                    interface,
                                    systems,
                                    result_window,
                                    screen_pos,
                                    false,
                                    true,
                                );
                            }
                            _ => {}
                        }
                    }

                    return Ok(true);
                }

                if let Some(window) = &interface.drag_window {
                    match window {
                        Window::Inventory => {
                            interface.inventory.move_window(systems, screen_pos)
                        }
                        Window::Profile => {
                            interface.profile.move_window(systems, screen_pos)
                        }
                        Window::Setting => {
                            interface.setting.move_window(systems, screen_pos)
                        }
                        Window::Chatbox => {
                            interface.chatbox.move_window(systems, screen_pos)
                        }
                        Window::Storage => {
                            interface.storage.move_window(systems, screen_pos)
                        }
                        Window::Shop => {
                            interface.shop.move_window(systems, screen_pos)
                        }
                        Window::Trade => {
                            interface.trade.move_window(systems, screen_pos)
                        }
                    }
                    result = true;
                } else {
                    if interface.setting.visible {
                        interface
                            .setting
                            .sfx_scroll
                            .set_move_scroll(systems, screen_pos);
                        interface
                            .setting
                            .bgm_scroll
                            .set_move_scroll(systems, screen_pos);

                        if interface.setting.bgm_scroll.in_hold {
                            interface.setting.update_bgm_value(
                                systems,
                                interface.setting.bgm_scroll.value,
                            );
                            let volume = interface.setting.bgm_scroll.value
                                as f32
                                * 0.01;
                            systems.audio.set_music_volume(volume);
                            result = true;
                        } else if interface.setting.sfx_scroll.in_hold {
                            interface.setting.update_sfx_value(
                                systems,
                                interface.setting.sfx_scroll.value,
                            );
                            let volume = interface.setting.sfx_scroll.value
                                as f32
                                * 0.01;

                            systems.audio.set_effect_volume(volume);
                            result = true;
                        }
                    }

                    interface
                        .chatbox
                        .scrollbar
                        .set_move_scroll(systems, screen_pos);
                    interface.chatbox.set_chat_scrollbar(systems, false);

                    if interface.shop.visible {
                        interface
                            .shop
                            .item_scroll
                            .set_move_scroll(systems, screen_pos);
                        interface.shop.set_shop_scroll_value(systems);

                        if interface.shop.item_scroll.in_hold {
                            result = true;
                        }
                    }

                    if interface.chatbox.scrollbar.in_hold {
                        result = true;
                    }

                    interface.hold_move_textbox(systems, screen_pos);
                }
            }
            MouseInputType::MouseRelease => {
                if let Some(slot) = interface.inventory.hold_slot {
                    release_inv_slot(
                        interface, socket, systems, alert, slot, screen_pos,
                    )?;
                    interface.inventory.hold_slot = None;
                    return Ok(true);
                }

                if let Some(slot) = interface.storage.hold_slot {
                    release_storage_slot(
                        interface, socket, systems, alert, slot, screen_pos,
                    )?;
                    interface.storage.hold_slot = None;
                    return Ok(true);
                }

                interface.reset_buttons(systems);
                interface.release_textbox();

                if let Some(window) = &interface.drag_window {
                    match window {
                        Window::Inventory => {
                            interface.inventory.release_window()
                        }
                        Window::Profile => interface.profile.release_window(),
                        Window::Setting => interface.setting.release_window(),
                        Window::Chatbox => interface.chatbox.release_window(),
                        Window::Storage => interface.storage.release_window(),
                        Window::Shop => interface.shop.release_window(),
                        Window::Trade => interface.trade.release_window(),
                    }
                }

                interface.drag_window = None;

                if interface.setting.visible {
                    if interface.setting.bgm_scroll.in_hold {
                        systems.config.bgm_volume =
                            interface.setting.bgm_scroll.value as u8;
                        systems.config.save_config("settings.toml");
                    } else if interface.setting.sfx_scroll.in_hold {
                        systems.config.sfx_volume =
                            interface.setting.sfx_scroll.value as u8;
                        systems.config.save_config("settings.toml");
                    }

                    interface
                        .setting
                        .sfx_scroll
                        .set_hold(systems, false, screen_pos);
                    interface
                        .setting
                        .bgm_scroll
                        .set_hold(systems, false, screen_pos);
                }

                interface
                    .chatbox
                    .scrollbar
                    .set_hold(systems, false, screen_pos);

                if interface.shop.visible {
                    interface
                        .shop
                        .item_scroll
                        .set_hold(systems, false, screen_pos);
                }

                interface.chatbox.reset_buttons(systems);
                interface.profile.reset_buttons(systems);
                interface.setting.reset_buttons(systems);
                interface.inventory.reset_buttons(systems);
                interface.storage.reset_buttons(systems);
                interface.shop.reset_buttons(systems);
                interface.trade.reset_buttons(systems);
                interface.setting.reset_checkbox(systems);
            }
        }

        Ok(result)
    }

    pub fn key_input(
        game_content: &mut GameContent,
        _world: &mut World,
        systems: &mut SystemHolder,
        socket: &mut Poller,
        key: &Key,
        pressed: bool,
    ) -> Result<()> {
        if pressed
            && !game_content.interface.trade.visible
            && let Key::Named(NamedKey::Enter) = key
        {
            if game_content.interface.selected_textbox
                == SelectedTextbox::Chatbox
            {
                game_content.interface.selected_textbox = SelectedTextbox::None;
                game_content
                    .interface
                    .chatbox
                    .textbox
                    .set_select(systems, false);
                trigger_chatbox_button(
                    &mut game_content.interface,
                    systems,
                    socket,
                    2,
                )?;
            } else {
                game_content.interface.selected_textbox =
                    SelectedTextbox::Chatbox;
                game_content
                    .interface
                    .chatbox
                    .textbox
                    .set_select(systems, true);
            }
        }

        match game_content.interface.selected_textbox {
            SelectedTextbox::Chatbox => {
                game_content
                    .interface
                    .chatbox
                    .textbox
                    .enter_text(systems, key, pressed, false);
            }
            SelectedTextbox::Trade => {
                game_content
                    .interface
                    .trade
                    .money_input
                    .enter_text(systems, key, pressed, true);
            }
            _ => {}
        }
        Ok(())
    }

    pub fn hover_buttons(
        interface: &mut Interface,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
    ) {
        for button in interface.menu_button.iter_mut() {
            if is_within_area(
                screen_pos,
                Vec2::new(
                    button.base_pos.x
                        + (button.adjust_pos.x * systems.scale as f32).floor(),
                    button.base_pos.y
                        + (button.adjust_pos.y * systems.scale as f32).floor(),
                ),
                (button.size * systems.scale as f32).floor(),
            ) {
                button.set_hover(systems, true);
            } else {
                button.set_hover(systems, false);
            }
        }
    }

    pub fn click_window_buttons(
        &mut self,
        systems: &mut SystemHolder,
        socket: &mut Poller,
        screen_pos: Vec2,
        alert: &mut Alert,
    ) -> Result<bool> {
        let mut did_click = false;

        for index in 0..self.window_order.len() {
            if !did_click {
                match self.window_order[index].0 {
                    Window::Chatbox => {
                        if self.chatbox.in_window(screen_pos, systems) {
                            if let Some(index) =
                                self.chatbox.click_buttons(systems, screen_pos)
                            {
                                self.chatbox.did_button_click = true;
                                trigger_chatbox_button(
                                    self, systems, socket, index,
                                )?;
                            }

                            self.chatbox.select_chat_tab(systems, screen_pos);
                            self.click_textbox(
                                systems,
                                socket,
                                screen_pos,
                                SelectedTextbox::Chatbox,
                            )?;

                            if self.chatbox.scrollbar.in_scroll(screen_pos) {
                                self.chatbox
                                    .scrollbar
                                    .set_hold(systems, true, screen_pos);
                            }

                            hold_interface(
                                self,
                                systems,
                                Window::Chatbox,
                                screen_pos,
                                true,
                                false,
                            );
                            did_click = true;
                        }
                    }
                    Window::Inventory => {
                        if self.inventory.in_window(screen_pos) {
                            if let Some(index) = self
                                .inventory
                                .click_buttons(systems, screen_pos)
                            {
                                if index == 0 {
                                    close_interface(
                                        self,
                                        systems,
                                        Window::Inventory,
                                    );
                                    return Ok(true);
                                }

                                self.inventory.did_button_click = true;
                            }

                            hold_interface(
                                self,
                                systems,
                                Window::Inventory,
                                screen_pos,
                                true,
                                false,
                            );
                            did_click = true;
                        }
                    }
                    Window::Profile => {
                        if self.profile.in_window(screen_pos) {
                            if let Some(index) =
                                self.profile.click_buttons(systems, screen_pos)
                            {
                                if index == 0 {
                                    close_interface(
                                        self,
                                        systems,
                                        Window::Profile,
                                    );
                                    return Ok(true);
                                }

                                self.profile.did_button_click = true;
                            }

                            hold_interface(
                                self,
                                systems,
                                Window::Profile,
                                screen_pos,
                                true,
                                false,
                            );
                            did_click = true;
                        }
                    }
                    Window::Setting => {
                        if self.setting.in_window(screen_pos) {
                            if let Some(index) =
                                self.setting.click_buttons(systems, screen_pos)
                            {
                                if index == 0 {
                                    close_interface(
                                        self,
                                        systems,
                                        Window::Setting,
                                    );
                                    return Ok(true);
                                }

                                self.setting.did_button_click = true;
                            }

                            if self.setting.sfx_scroll.in_scroll(screen_pos) {
                                self.setting
                                    .sfx_scroll
                                    .set_hold(systems, true, screen_pos);
                            }

                            if self.setting.bgm_scroll.in_scroll(screen_pos) {
                                self.setting
                                    .bgm_scroll
                                    .set_hold(systems, true, screen_pos);
                            }

                            if let Some(index) =
                                self.setting.click_checkbox(systems, screen_pos)
                            {
                                self.setting.did_checkbox_click = true;
                                self.setting.trigger_checkbox(
                                    systems,
                                    index,
                                    &self.ping_text,
                                    &self.average_ping,
                                    &self.frame_loop,
                                );
                            }

                            hold_interface(
                                self,
                                systems,
                                Window::Setting,
                                screen_pos,
                                true,
                                false,
                            );
                            did_click = true;
                        }
                    }
                    Window::Shop => {
                        if self.shop.in_window(screen_pos) {
                            if let Some(index) =
                                self.shop.click_buttons(systems, screen_pos)
                            {
                                match index {
                                    0 => {
                                        close_interface(
                                            self,
                                            systems,
                                            Window::Shop,
                                        );
                                        send_closeshop(socket)?;
                                        return Ok(true);
                                    } // Close
                                    1 => {
                                        // Scroll Up
                                        if self.shop.item_scroll.max_value == 0
                                        {
                                            return Ok(true);
                                        }

                                        let scrollbar_value = self
                                            .shop
                                            .item_scroll
                                            .value
                                            .saturating_sub(1);

                                        self.shop.item_scroll.set_value(
                                            systems,
                                            scrollbar_value,
                                        );
                                        self.shop
                                            .set_shop_scroll_value(systems);
                                    }
                                    2 => {
                                        // Scroll Down
                                        if self.shop.item_scroll.max_value == 0
                                        {
                                            return Ok(true);
                                        }

                                        let scrollbar_value = self
                                            .shop
                                            .item_scroll
                                            .value
                                            .saturating_add(1)
                                            .min(
                                                self.shop.item_scroll.max_value,
                                            );

                                        self.shop.item_scroll.set_value(
                                            systems,
                                            scrollbar_value,
                                        );
                                        self.shop
                                            .set_shop_scroll_value(systems);
                                    }
                                    3..=7 => {
                                        let button_index =
                                            self.shop.shop_start_pos
                                                + index.saturating_sub(3);

                                        send_buyitem(
                                            socket,
                                            button_index as u16,
                                        )?;
                                    }
                                    _ => {}
                                }

                                self.shop.did_button_click = true;
                            }

                            if self.shop.item_scroll.in_scroll(screen_pos) {
                                self.shop
                                    .item_scroll
                                    .set_hold(systems, true, screen_pos);
                            }

                            hold_interface(
                                self,
                                systems,
                                Window::Shop,
                                screen_pos,
                                true,
                                false,
                            );
                            did_click = true;
                        }
                    }
                    Window::Storage => {
                        if self.storage.in_window(screen_pos) {
                            if let Some(index) =
                                self.storage.click_buttons(systems, screen_pos)
                            {
                                if index == 0 {
                                    close_interface(
                                        self,
                                        systems,
                                        Window::Storage,
                                    );
                                    send_closestorage(socket)?;
                                    return Ok(true);
                                }
                                self.storage.did_button_click = true;
                            }

                            hold_interface(
                                self,
                                systems,
                                Window::Storage,
                                screen_pos,
                                true,
                                false,
                            );
                            did_click = true;
                        }
                    }
                    Window::Trade => {
                        if self.trade.in_window(screen_pos) {
                            if let Some(index) =
                                self.trade.click_buttons(systems, screen_pos)
                            {
                                match index {
                                    0 | 2 => {
                                        close_interface(
                                            self,
                                            systems,
                                            Window::Trade,
                                        );
                                        send_closetrade(socket)?;
                                        return Ok(true);
                                    }
                                    1 => {
                                        if matches!(
                                            self.trade.trade_status,
                                            TradeStatus::None
                                                | TradeStatus::Accepted
                                        ) {
                                            send_submittrade(socket)?;
                                        }
                                    }
                                    _ => {}
                                }

                                self.trade.did_button_click = true;
                            }
                            self.click_textbox(
                                systems,
                                socket,
                                screen_pos,
                                SelectedTextbox::Trade,
                            )?;

                            if self.trade.trade_status == TradeStatus::None
                                && let Some(slot) = self
                                    .trade
                                    .find_mytrade_slot(systems, screen_pos)
                                && self.trade.my_items[slot].got_data
                            {
                                if self.trade.my_items[slot].count_data > 1 {
                                    alert.show_alert(
                                        systems,
                                        AlertType::Input,
                                        String::new(),
                                        "Enter the amount to remove".into(),
                                        250,
                                        AlertIndex::RemoveTradeItem(
                                            slot as u16,
                                        ),
                                        true,
                                    );
                                } else {
                                    send_removetradeitem(
                                        socket,
                                        slot as u16,
                                        1,
                                    )?;
                                }
                            }

                            hold_interface(
                                self,
                                systems,
                                Window::Trade,
                                screen_pos,
                                true,
                                false,
                            );
                            did_click = true;
                        }
                    }
                }
            }
        }

        Ok(did_click)
    }

    pub fn click_buttons(
        interface: &mut Interface,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
    ) -> Option<usize> {
        let mut button_found = None;

        for (index, button) in interface.menu_button.iter_mut().enumerate() {
            if is_within_area(
                screen_pos,
                Vec2::new(
                    button.base_pos.x
                        + (button.adjust_pos.x * systems.scale as f32).floor(),
                    button.base_pos.y
                        + (button.adjust_pos.y * systems.scale as f32).floor(),
                ),
                (button.size * systems.scale as f32).floor(),
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
        self.menu_button.iter_mut().for_each(|button| {
            button.set_click(systems, false);
        });
    }

    pub fn click_textbox(
        &mut self,
        systems: &mut SystemHolder,
        socket: &mut Poller,
        screen_pos: Vec2,
        chatbox_type: SelectedTextbox,
    ) -> Result<()> {
        match chatbox_type {
            SelectedTextbox::Chatbox => {
                if is_within_area(
                    screen_pos,
                    Vec2::new(
                        self.chatbox.textbox.base_pos.x
                            + (self.chatbox.textbox.adjust_pos.x
                                * systems.scale as f32)
                                .floor(),
                        self.chatbox.textbox.base_pos.y
                            + (self.chatbox.textbox.adjust_pos.y
                                * systems.scale as f32)
                                .floor(),
                    ),
                    (self.chatbox.textbox.size * systems.scale as f32).floor(),
                ) {
                    self.chatbox.textbox.set_select(systems, true);
                    self.chatbox.textbox.set_hold(true);
                    self.chatbox.textbox.select_text(systems, screen_pos);
                    self.selected_textbox = SelectedTextbox::Chatbox;
                    return Ok(());
                }
            }
            SelectedTextbox::Trade => {
                if self.trade.visible
                    & is_within_area(
                        screen_pos,
                        Vec2::new(
                            self.trade.money_input.base_pos.x
                                + (self.trade.money_input.adjust_pos.x
                                    * systems.scale as f32)
                                    .floor(),
                            self.trade.money_input.base_pos.y
                                + (self.trade.money_input.adjust_pos.y
                                    * systems.scale as f32)
                                    .floor(),
                        ),
                        self.trade.money_input.size,
                    )
                {
                    self.trade.money_input.set_select(systems, true);
                    self.trade.money_input.set_hold(true);
                    self.trade.money_input.select_text(systems, screen_pos);
                    self.selected_textbox = SelectedTextbox::Trade;
                    return Ok(());
                }
            }
            _ => {}
        }

        match self.selected_textbox {
            SelectedTextbox::Chatbox => {
                self.chatbox.textbox.set_select(systems, false)
            }
            SelectedTextbox::Trade => {
                self.trade.money_input.set_select(systems, false);
                if self.trade.trade_status == TradeStatus::None {
                    let input_text = self.trade.money_input.text.clone();
                    let amount = input_text.parse::<u64>().unwrap_or_default();
                    send_updatetrademoney(socket, amount)?;
                }
            }
            _ => {}
        }

        self.selected_textbox = SelectedTextbox::None;
        Ok(())
    }

    pub fn release_textbox(&mut self) {
        match self.selected_textbox {
            SelectedTextbox::Chatbox => {
                self.chatbox.textbox.set_hold(false);
            }
            SelectedTextbox::Trade => {
                self.trade.money_input.set_hold(false);
            }
            _ => {}
        }
    }

    pub fn hold_move_textbox(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
    ) {
        match self.selected_textbox {
            SelectedTextbox::Chatbox => {
                self.chatbox.textbox.hold_move(systems, screen_pos);
            }
            SelectedTextbox::Trade => {
                self.trade.money_input.hold_move(systems, screen_pos);
            }
            _ => {}
        }
    }
}

fn trigger_button(
    interface: &mut Interface,
    systems: &mut SystemHolder,
    index: usize,
) {
    match index {
        0 => {
            if interface.profile.visible {
                close_interface(interface, systems, Window::Profile);
            } else {
                open_interface(interface, systems, Window::Profile);
            }
        }
        1 => {
            if interface.inventory.visible {
                close_interface(interface, systems, Window::Inventory);
            } else {
                open_interface(interface, systems, Window::Inventory);
            }
        }
        2 => {
            if interface.setting.visible {
                close_interface(interface, systems, Window::Setting);
            } else {
                open_interface(interface, systems, Window::Setting);
            }
        }
        _ => {}
    }
}

fn trigger_chatbox_button(
    interface: &mut Interface,
    systems: &mut SystemHolder,
    socket: &mut Poller,
    index: usize,
) -> Result<()> {
    match index {
        0 => {
            // Scroll Up
            if interface.chatbox.scrollbar.max_value == 0 {
                return Ok(());
            }

            let scrollbar_value = interface
                .chatbox
                .scrollbar
                .value
                .saturating_add(1)
                .min(interface.chatbox.scrollbar.max_value);

            interface
                .chatbox
                .scrollbar
                .set_value(systems, scrollbar_value);
            interface.chatbox.set_chat_scrollbar(systems, true);
        }
        1 => {
            // Scroll Down
            if interface.chatbox.scrollbar.max_value == 0 {
                return Ok(());
            }

            let scrollbar_value =
                interface.chatbox.scrollbar.value.saturating_sub(1);

            interface
                .chatbox
                .scrollbar
                .set_value(systems, scrollbar_value);
            interface.chatbox.set_chat_scrollbar(systems, true);
        }
        2 => {
            send_chat(interface, systems, socket)?;
        }
        _ => {}
    }
    Ok(())
}

fn can_find_window(window: Window, exception: Option<Window>) -> bool {
    if let Some(x_window) = exception
        && window == x_window
    {
        return false;
    }

    true
}

fn find_window(
    systems: &mut SystemHolder,
    interface: &mut Interface,
    screen_pos: Vec2,
    exception: Option<Window>,
) -> Option<Window> {
    let mut max_z_order: f32 = 0.0;
    let mut selected_window = None;

    if interface.inventory.in_window(screen_pos)
        && can_find_window(Window::Inventory, exception)
    {
        max_z_order = interface.inventory.z_order;
        selected_window = Some(Window::Inventory);
    }

    if interface.profile.in_window(screen_pos)
        && can_find_window(Window::Profile, exception)
        && interface.profile.z_order > max_z_order
    {
        max_z_order = interface.profile.z_order;
        selected_window = Some(Window::Profile);
    }

    if interface.setting.in_window(screen_pos)
        && can_find_window(Window::Setting, exception)
        && interface.setting.z_order > max_z_order
    {
        max_z_order = interface.setting.z_order;
        selected_window = Some(Window::Setting);
    }

    if interface.chatbox.in_window(screen_pos, systems)
        && can_find_window(Window::Chatbox, exception)
        && interface.chatbox.z_order > max_z_order
    {
        max_z_order = interface.chatbox.z_order;
        selected_window = Some(Window::Chatbox);
    }

    if interface.storage.in_window(screen_pos)
        && can_find_window(Window::Storage, exception)
        && interface.storage.z_order > max_z_order
    {
        max_z_order = interface.storage.z_order;
        selected_window = Some(Window::Storage);
    }

    if interface.shop.in_window(screen_pos)
        && can_find_window(Window::Shop, exception)
        && interface.shop.z_order > max_z_order
    {
        max_z_order = interface.shop.z_order;
        selected_window = Some(Window::Shop);
    }

    if interface.trade.in_window(screen_pos)
        && can_find_window(Window::Trade, exception)
        && interface.trade.z_order > max_z_order
    {
        selected_window = Some(Window::Trade);
    }

    selected_window
}

pub fn open_interface(
    interface: &mut Interface,
    systems: &mut SystemHolder,
    window: Window,
) {
    match window {
        Window::Inventory => {
            if interface.inventory.visible {
                return;
            }

            interface.inventory.set_visible(systems, true);
        }
        Window::Profile => {
            if interface.profile.visible {
                return;
            }

            interface.profile.set_visible(systems, true);
        }
        Window::Setting => {
            if interface.setting.visible {
                return;
            }

            interface.setting.set_visible(systems, true);
        }
        Window::Storage => {
            if interface.storage.visible {
                return;
            }

            interface.storage.set_visible(systems, true);
        }
        Window::Shop => {
            if interface.shop.visible {
                return;
            }

            interface.shop.set_visible(systems, true);
        }
        Window::Trade => {
            if interface.trade.visible {
                return;
            }

            interface.trade.set_visible(systems, true);
        }
        _ => {}
    }

    interface_set_to_first(interface, systems, window);
}

pub fn close_interface(
    interface: &mut Interface,
    systems: &mut SystemHolder,
    window: Window,
) {
    match window {
        Window::Inventory => {
            if !interface.inventory.visible {
                return;
            }

            interface.inventory.set_visible(systems, false);
            interface.item_desc.set_visible(systems, false);
        }
        Window::Profile => {
            if !interface.profile.visible {
                return;
            }

            interface.profile.set_visible(systems, false);
            interface.item_desc.set_visible(systems, false);
        }
        Window::Setting => {
            if !interface.setting.visible {
                return;
            }

            interface.setting.set_visible(systems, false);
        }
        Window::Storage => {
            if !interface.storage.visible {
                return;
            }

            interface.storage.set_visible(systems, false);
            interface.item_desc.set_visible(systems, false);
        }
        Window::Shop => {
            if !interface.shop.visible {
                return;
            }

            interface.shop.set_visible(systems, false);
            interface.item_desc.set_visible(systems, false);
        }
        Window::Trade => {
            if !interface.trade.visible {
                return;
            }

            interface.trade.set_visible(systems, false);
            interface.item_desc.set_visible(systems, false);
        }
        _ => {}
    }

    interface_set_to_last(interface, systems, window);
}

fn hold_interface(
    interface: &mut Interface,
    systems: &mut SystemHolder,
    window: Window,
    screen_pos: Vec2,
    check_content: bool,
    hold_check: bool,
) {
    interface_set_to_first(interface, systems, window);

    match window {
        Window::Inventory => {
            if interface.inventory.can_hold(systems, screen_pos) && !hold_check
            {
                interface.inventory.hold_window(screen_pos);
            } else if let Some(slot) = interface
                .inventory
                .find_inv_slot(systems, screen_pos, false)
            {
                if check_content {
                    interface
                        .inventory
                        .hold_inv_slot(systems, slot, screen_pos);
                }

                return;
            } else {
                return;
            }
        }
        Window::Profile => {
            if !interface.profile.can_hold(systems, screen_pos) {
                return;
            }

            interface.profile.hold_window(screen_pos);
        }
        Window::Setting => {
            if !interface.setting.can_hold(screen_pos) {
                return;
            }

            interface.setting.hold_window(screen_pos);
        }
        Window::Chatbox => {
            if !interface.chatbox.can_hold(screen_pos) {
                return;
            }

            interface.chatbox.hold_window(screen_pos);
        }
        Window::Storage => {
            if interface.storage.can_hold(systems, screen_pos) && !hold_check {
                interface.storage.hold_window(screen_pos);
            } else if let Some(slot) = interface
                .storage
                .find_storage_slot(systems, screen_pos, false)
            {
                if check_content {
                    interface
                        .storage
                        .hold_storage_slot(systems, slot, screen_pos);
                }

                return;
            } else {
                return;
            }
        }
        Window::Shop => {
            if !interface.shop.can_hold(systems, screen_pos) {
                return;
            }

            interface.shop.hold_window(screen_pos);
        }
        Window::Trade => {
            if !interface.trade.can_hold(systems, screen_pos) {
                return;
            }

            interface.trade.hold_window(screen_pos);
        }
    }
    interface.drag_window = Some(window);
}

fn interface_set_to_first(
    interface: &mut Interface,
    systems: &mut SystemHolder,
    window: Window,
) {
    if let Some(index) = interface
        .window_order
        .iter()
        .position(|&wndw| wndw.0 == window)
    {
        if interface.window_order[index].1 == 0 {
            return;
        }

        for i in 0..index {
            interface.window_order[i].1 = i.saturating_add(1);
        }

        interface.window_order[index].1 = 0;
    }

    interface.window_order.sort_by(|a, b| a.1.cmp(&b.1));
    adjust_window_zorder(interface, systems);
}

fn interface_set_to_last(
    interface: &mut Interface,
    systems: &mut SystemHolder,
    window: Window,
) {
    let last_index = interface.window_order.len() - 1;

    if let Some(index) = interface
        .window_order
        .iter()
        .position(|&wndw| wndw.0 == window)
    {
        if interface.window_order[index].1 == last_index {
            return;
        }

        for i in index..(last_index + 1) {
            interface.window_order[i].1 = i.saturating_sub(1);
        }

        interface.window_order[index].1 = last_index;
    }

    interface.window_order.sort_by(|a, b| a.1.cmp(&b.1));
    adjust_window_zorder(interface, systems);
}

fn adjust_window_zorder(interface: &mut Interface, systems: &mut SystemHolder) {
    let mut order = 0.99;

    for wndw in interface.window_order.iter() {
        match wndw.0 {
            Window::Inventory => {
                interface.inventory.set_z_order(systems, order, wndw.1)
            }
            Window::Profile => {
                interface.profile.set_z_order(systems, order, wndw.1)
            }
            Window::Setting => {
                interface.setting.set_z_order(systems, order, wndw.1)
            }
            Window::Chatbox => {
                interface.chatbox.set_z_order(systems, order, wndw.1)
            }
            Window::Storage => {
                interface.storage.set_z_order(systems, order, wndw.1)
            }
            Window::Shop => interface.shop.set_z_order(systems, order, wndw.1),
            Window::Trade => {
                interface.trade.set_z_order(systems, order, wndw.1)
            }
        }

        order -= 0.01;
    }
}
