use crate::{GlobalKey, data_types::*, socket::*};
use graphics::*;
use serde::{Deserialize, Serialize};

pub const TILE_SIZE: usize = 20;
pub const SCREEN_WIDTH: usize = 800;
pub const SCREEN_HEIGHT: usize = 600;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Deserialize,
    Serialize,
    ByteBufferRead,
    ByteBufferWrite,
    MByteBufferRead,
    MByteBufferWrite,
)]
pub enum MessageChannel {
    #[default]
    Map,
    Global,
    Trade,
    Party,
    Private,
    Guild,
    Help,
    Quest,
    Npc,
}

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Deserialize,
    Serialize,
    Default,
    ByteBufferRead,
    ByteBufferWrite,
    MByteBufferRead,
    MByteBufferWrite,
)]
pub enum VitalTypes {
    Hp,
    Mp,
    Sp,
    #[default]
    Count,
}

#[derive(
    PartialEq,
    Eq,
    Copy,
    Clone,
    Debug,
    Default,
    Deserialize,
    Serialize,
    ByteBufferRead,
    ByteBufferWrite,
    MByteBufferRead,
    MByteBufferWrite,
)]
pub enum TradeStatus {
    #[default]
    None,
    Accepted,
    Submitted,
}

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    ByteBufferRead,
    ByteBufferWrite,
    MByteBufferRead,
    MByteBufferWrite,
)]
pub enum EquipmentType {
    Weapon,
    Helmet,
    Chest,
    Pants,
    Accessory,
    Count,
} //5

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Deserialize,
    Serialize,
    ByteBufferRead,
    ByteBufferWrite,
    MByteBufferRead,
    MByteBufferWrite,
)]
pub enum FtlType {
    Message,
    Error,
    Item,
    Quest,
    Level,
    Money,
}

pub const COLOR_WHITE: Color = Color::rgba(255, 255, 255, 255);
pub const COLOR_RED: Color = Color::rgba(230, 30, 30, 255);
pub const COLOR_BLUE: Color = Color::rgba(30, 30, 230, 255);
pub const COLOR_GREEN: Color = Color::rgba(40, 255, 40, 255);

pub const VITALS_MAX: usize = VitalTypes::Count as usize;
pub const MAX_INV: usize = 30;
pub const MAX_TRADE_SLOT: usize = 30;
pub const MAX_STORAGE: usize = 70;
pub const MAX_EQPT: usize = 5;
pub const MAX_SHOP_ITEM: usize = 20;

pub const MAX_NPCS: usize = 1000;
pub const MAX_ITEMS: usize = 2000;
pub const MAX_SHOPS: usize = 100;

pub const ORDER_MENU_BG: f32 = 10.9;
pub const ORDER_MENU_WINDOW: f32 = 10.8;
pub const ORDER_MENU_WINDOW_CONTENT: f32 = 10.7;
pub const ORDER_MENU_WINDOW_CONTENT_DETAIL: f32 = 10.6;
pub const ORDER_MENU_WINDOW_CONTENT_DETAIL2: f32 = 10.5;
pub const ORDER_SERVER_STATUS: f32 = 10.8;
// Lower Map Order 9.3 - 9.0
pub const ORDER_MAP_ITEM: f32 = 8.5;
pub const ORDER_PLAYER: f32 = 8.0;
pub const ORDER_NPC: f32 = 8.0;
pub const ORDER_HPBAR_BG: f32 = 7.9;
pub const ORDER_HPBAR: f32 = 7.8;
pub const ORDER_TARGET: f32 = 7.7;
// Upper Map Order 5.1 - 5.0
pub const ORDER_LIGHT: f32 = 4.9;
pub const ORDER_ENTITY_NAME: f32 = 4.8;
pub const ORDER_FLOAT_TEXT_BG: f32 = 4.7;
pub const ORDER_FLOAT_TEXT: f32 = 4.6;
pub const ORDER_MAP_FADE: f32 = 4.0;
pub const ORDER_VITAL_BG: f32 = 3.9;
pub const ORDER_VITAL_HPBG: f32 = 3.8;
pub const ORDER_VITAL_HP: f32 = 3.7;
pub const ORDER_GUI_BUTTON: f32 = 3.9;
pub const ORDER_GUI_BUTTON_DETAIL: f32 = 3.8;
pub const ORDER_GUI_WINDOW: f32 = 2.999; // The whole 2.9xxx will be used for Game Window
pub const ORDER_HOLD_ITEM: f32 = 1.99;
pub const ORDER_ITEM_DESC: f32 = 1.989;
pub const ORDER_ITEM_DESC_TEXT: f32 = 1.988;
pub const ORDER_ALERT_BG: f32 = 1.59;
pub const ORDER_ALERT: f32 = 1.58;
pub const ORDER_ALERT_HEADER: f32 = 1.57;
pub const ORDER_ALERT_HEADER_TEXT: f32 = 1.56;
pub const ORDER_ALERT_TEXT: f32 = 1.57;
pub const ORDER_ALERT_TEXTBOX_BG: f32 = 1.57;
pub const ORDER_ALERT_TEXTBOX: f32 = 1.56;
pub const ORDER_ALERT_BUTTON: f32 = 1.57;
pub const ORDER_TOOLTIP: f32 = 1.05;
pub const ORDER_TOOLTIP_TEXT: f32 = 1.0;
pub const ORDER_FADE: f32 = 0.9;
