use cosmic_text::{Attrs, Metrics};
use graphics::*;

use input::Key;
use winit::keyboard::NamedKey;

use crate::{
    interface::chatbox::*, is_within_area, send_buyitem, send_closeshop,
    send_closestorage, send_closetrade, send_command, send_message,
    send_removetradeitem, send_submittrade, send_unequip,
    send_updatetrademoney, send_useitem, widget::*, Alert, AlertIndex,
    AlertType, GameContent, MouseInputType, Result, Socket, SystemHolder,
    TradeStatus, COLOR_WHITE,
};
use hecs::World;

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
    pub ping_text: usize,
    menu_button: [Button; 3],
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

        let size = (Vec2::new(85.0, 20.0) * systems.scale as f32).floor();
        let addy = if systems.config.show_fps {
            30.0 * systems.scale as f32
        } else {
            5.0 * systems.scale as f32
        }
        .floor();
        let ping_pos = Vec3::new(
            systems.size.width - size.x,
            systems.size.height - size.y - addy,
            0.0,
        );
        let ping = create_label(
            systems,
            ping_pos,
            size,
            Bounds::new(
                ping_pos.x,
                ping_pos.y,
                ping_pos.x + size.x,
                ping_pos.y + size.y,
            ),
            Color::rgba(200, 200, 200, 255),
        );
        let ping_text = systems.gfx.add_text(ping, 5, "Ping".to_string());
        systems.gfx.set_visible(ping_text, systems.config.show_ping);
        systems
            .gfx
            .set_text(&mut systems.renderer, ping_text, "Ping: 0");

        let mut interface = Interface {
            menu_button,
            ping_text,
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
            .set_visible(self.ping_text, systems.config.show_ping);
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
        systems.gfx.set_visible(self.ping_text, false);
    }

    pub fn mouse_input(
        interface: &mut Interface,
        _world: &mut World,
        systems: &mut SystemHolder,
        socket: &mut Socket,
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
                {
                    if let Some(slot) =
                        interface.inventory.find_inv_slot(screen_pos, false)
                    {
                        send_useitem(socket, slot as u16)?;
                    }
                }

                if interface.profile.visible
                    && interface.profile.order_index == 0
                {
                    if let Some(slot) = interface
                        .profile
                        .find_eq_slot(systems, screen_pos, false)
                    {
                        send_unequip(socket, slot as u16)?;
                    }
                }

                if interface.chatbox.order_index == 0 {
                    if let Some(text) = interface.chatbox.get_selected_msg() {
                        set_clipboard_text(text);
                    }
                }
            }
            MouseInputType::MouseLeftDown => {
                result = Interface::click_window_buttons(
                    interface, systems, socket, screen_pos,
                )?;

                let button_index =
                    Interface::click_buttons(interface, systems, screen_pos);
                if let Some(index) = button_index {
                    interface.did_button_click = true;
                    trigger_button(interface, systems, index);
                    result = true;
                }

                if interface.drag_window.is_none() {
                    let window =
                        find_window(systems, interface, screen_pos, None);
                    if let Some(result_window) = window {
                        hold_interface(
                            interface,
                            systems,
                            result_window,
                            screen_pos,
                            true,
                            false,
                        );
                        result = true;
                    }
                }

                if interface.trade.visible
                    && interface.trade.order_index == 0
                    && interface.drag_window.is_none()
                    && interface.trade.trade_status == TradeStatus::None
                {
                    if let Some(slot) =
                        interface.trade.find_mytrade_slot(screen_pos)
                    {
                        if interface.trade.my_items[slot].got_data {
                            if interface.trade.my_items[slot].count_data > 1 {
                                alert.show_alert(
                                    systems,
                                    AlertType::Input,
                                    String::new(),
                                    "Enter the amount to remove".into(),
                                    250,
                                    AlertIndex::RemoveTradeItem(slot as u16),
                                    true,
                                );
                            } else {
                                send_removetradeitem(socket, slot as u16, 1)?;
                            }
                        }
                    }
                }

                if interface.setting.visible && interface.drag_window.is_none()
                {
                    if interface.setting.sfx_scroll.in_scroll(screen_pos) {
                        interface
                            .setting
                            .sfx_scroll
                            .set_hold(systems, true, screen_pos);
                        result = true;
                    }
                    if interface.setting.bgm_scroll.in_scroll(screen_pos) {
                        interface
                            .setting
                            .bgm_scroll
                            .set_hold(systems, true, screen_pos);
                        result = true;
                    }
                    if let Some(index) =
                        interface.setting.click_checkbox(systems, screen_pos)
                    {
                        interface.setting.did_checkbox_click = true;
                        interface.setting.trigger_checkbox(
                            systems,
                            index,
                            interface.ping_text,
                        );
                        result = true;
                    }
                }
                if interface.chatbox.scrollbar.in_scroll(screen_pos) {
                    interface
                        .chatbox
                        .scrollbar
                        .set_hold(systems, true, screen_pos);
                    result = true;
                }
                if interface.shop.visible
                    && interface.drag_window.is_none()
                    && interface.shop.item_scroll.in_scroll(screen_pos)
                {
                    interface
                        .shop
                        .item_scroll
                        .set_hold(systems, true, screen_pos);
                    result = true;
                }

                let chatbox_button_index =
                    interface.chatbox.click_buttons(systems, screen_pos);
                if let Some(index) = chatbox_button_index {
                    interface.chatbox.did_button_click = true;
                    trigger_chatbox_button(interface, systems, socket, index)?;
                    result = true;
                }
                interface.chatbox.select_chat_tab(systems, screen_pos);
                interface.click_textbox(systems, socket, screen_pos)?;
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
        key: &Key,
        pressed: bool,
    ) {
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
        interface: &mut Interface,
        systems: &mut SystemHolder,
        socket: &mut Socket,
        screen_pos: Vec2,
    ) -> Result<bool> {
        if let Some(index) =
            interface.profile.click_buttons(systems, screen_pos)
        {
            if index == 0 {
                close_interface(interface, systems, Window::Profile);
            }
            interface.profile.did_button_click = true;
            return Ok(true);
        }

        if let Some(index) =
            interface.setting.click_buttons(systems, screen_pos)
        {
            if index == 0 {
                close_interface(interface, systems, Window::Setting);
            }
            interface.setting.did_button_click = true;
            return Ok(true);
        }

        if let Some(index) =
            interface.inventory.click_buttons(systems, screen_pos)
        {
            if index == 0 {
                close_interface(interface, systems, Window::Inventory);
            }
            interface.inventory.did_button_click = true;
            return Ok(true);
        }

        if let Some(index) =
            interface.storage.click_buttons(systems, screen_pos)
        {
            if index == 0 {
                close_interface(interface, systems, Window::Storage);
                send_closestorage(socket)?;
            }
            interface.storage.did_button_click = true;
            return Ok(true);
        }

        if let Some(index) = interface.shop.click_buttons(systems, screen_pos) {
            match index {
                0 => {
                    close_interface(interface, systems, Window::Shop);
                    send_closeshop(socket)?;
                } // Close
                1 => {
                    // Scroll Up
                    if interface.shop.item_scroll.max_value == 0 {
                        return Ok(true);
                    }
                    let scrollbar_value =
                        interface.shop.item_scroll.value.saturating_sub(1);
                    interface
                        .shop
                        .item_scroll
                        .set_value(systems, scrollbar_value);
                    interface.shop.set_shop_scroll_value(systems);
                }
                2 => {
                    // Scroll Down
                    if interface.shop.item_scroll.max_value == 0 {
                        return Ok(true);
                    }
                    let scrollbar_value = interface
                        .shop
                        .item_scroll
                        .value
                        .saturating_add(1)
                        .min(interface.shop.item_scroll.max_value);
                    interface
                        .shop
                        .item_scroll
                        .set_value(systems, scrollbar_value);
                    interface.shop.set_shop_scroll_value(systems);
                }
                3..=7 => {
                    let button_index =
                        interface.shop.shop_start_pos + index.saturating_sub(3);
                    send_buyitem(socket, button_index as u16)?;
                }
                _ => {}
            }
            interface.shop.did_button_click = true;
            return Ok(true);
        }

        if let Some(index) = interface.trade.click_buttons(systems, screen_pos)
        {
            match index {
                0 | 2 => {
                    close_interface(interface, systems, Window::Trade);
                    send_closetrade(socket)?;
                }
                1 => {
                    if matches!(
                        interface.trade.trade_status,
                        TradeStatus::None | TradeStatus::Accepted
                    ) {
                        send_submittrade(socket)?;
                    }
                }
                _ => {}
            }
            interface.trade.did_button_click = true;
            return Ok(true);
        }

        Ok(false)
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
        socket: &mut Socket,
        screen_pos: Vec2,
    ) -> Result<()> {
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
    socket: &mut Socket,
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
    if let Some(x_window) = exception {
        if window == x_window {
            return false;
        }
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
    {
        let z_order = interface.profile.z_order;
        if z_order > max_z_order {
            max_z_order = z_order;
            selected_window = Some(Window::Profile);
        }
    }
    if interface.setting.in_window(screen_pos)
        && can_find_window(Window::Setting, exception)
    {
        let z_order = interface.setting.z_order;
        if z_order > max_z_order {
            max_z_order = z_order;
            selected_window = Some(Window::Setting);
        }
    }
    if interface.chatbox.in_window(screen_pos, systems)
        && can_find_window(Window::Chatbox, exception)
    {
        let z_order = interface.chatbox.z_order;
        if z_order > max_z_order {
            max_z_order = z_order;
            selected_window = Some(Window::Chatbox);
        }
    }
    if interface.storage.in_window(screen_pos)
        && can_find_window(Window::Storage, exception)
    {
        let z_order = interface.storage.z_order;
        if z_order > max_z_order {
            max_z_order = z_order;
            selected_window = Some(Window::Storage);
        }
    }
    if interface.shop.in_window(screen_pos)
        && can_find_window(Window::Shop, exception)
    {
        let z_order = interface.shop.z_order;
        if z_order > max_z_order {
            max_z_order = z_order;
            selected_window = Some(Window::Shop);
        }
    }
    if interface.trade.in_window(screen_pos)
        && can_find_window(Window::Trade, exception)
    {
        let z_order = interface.trade.z_order;
        if z_order > max_z_order {
            //max_z_order = z_order;
            selected_window = Some(Window::Trade);
        }
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
            if interface.inventory.can_hold(screen_pos) && !hold_check {
                interface.inventory.hold_window(screen_pos);
            } else if let Some(slot) =
                interface.inventory.find_inv_slot(screen_pos, false)
            {
                if check_content {
                    interface.inventory.hold_inv_slot(slot, screen_pos);
                }

                return;
            } else {
                return;
            }
        }
        Window::Profile => {
            if !interface.profile.can_hold(screen_pos) {
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
            if interface.storage.can_hold(screen_pos) && !hold_check {
                interface.storage.hold_window(screen_pos);
            } else if let Some(slot) =
                interface.storage.find_storage_slot(screen_pos, false)
            {
                if check_content {
                    interface.storage.hold_storage_slot(slot, screen_pos);
                }
                return;
            } else {
                return;
            }
        }
        Window::Shop => {
            if !interface.shop.can_hold(screen_pos) {
                return;
            }
            interface.shop.hold_window(screen_pos);
        }
        Window::Trade => {
            if !interface.trade.can_hold(screen_pos) {
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
