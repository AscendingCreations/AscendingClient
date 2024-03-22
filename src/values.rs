use crate::game_content::entity::*;
use bytey::{ByteBufferRead, ByteBufferWrite};
use serde::{Deserialize, Serialize};
use serde_repr::*;

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
    Copy, Clone, Debug, PartialEq, Eq, Default, Deserialize, Serialize,
)]
pub enum EntityType {
    #[default]
    None,
    Player(Entity),
    Npc(Entity),
    MapItem(Entity),
}

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize_repr,
    Deserialize_repr,
    ByteBufferRead,
    ByteBufferWrite,
)]
#[repr(u8)]
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
    Serialize_repr,
    Deserialize_repr,
    Default,
    ByteBufferRead,
    ByteBufferWrite,
)]
#[repr(u8)]
pub enum VitalTypes {
    Hp,
    Mp,
    Sp,
    #[default]
    Count,
}

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize_repr,
    Deserialize_repr,
    ByteBufferRead,
    ByteBufferWrite,
)]
#[repr(u8)]
pub enum FtlType {
    Message,
    Error,
    Item,
    Quest,
    Level,
    Money,
}

pub const VITALS_MAX: usize = VitalTypes::Count as usize;
pub const MAX_EQPT: usize = 5;

pub const ORDER_MENU_BG: f32 = 10.9;
pub const ORDER_MENU_WINDOW: f32 = 10.8;
pub const ORDER_MENU_WINDOW_CONTENT: f32 = 10.7;
pub const ORDER_MENU_WINDOW_CONTENT_DETAIL: f32 = 10.6;
pub const ORDER_MENU_WINDOW_CONTENT_DETAIL2: f32 = 10.5;
// Lower Map Order 9.3 - 9.0
pub const ORDER_MAP_ITEM: f32 = 8.5;
pub const ORDER_PLAYER: f32 = 8.0;
pub const ORDER_NPC: f32 = 8.0;
pub const ORDER_HPBAR_BG: f32 = 7.9;
pub const ORDER_HPBAR: f32 = 7.8;
pub const ORDER_TARGET: f32 = 7.7;
// Upper Map Order 5.1 - 5.0
pub const ORDER_MAP_FADE: f32 = 4.0;
pub const ORDER_GUI_BUTTON: f32 = 3.9;
pub const ORDER_GUI_BUTTON_DETAIL: f32 = 3.8;
pub const ORDER_GUI_CHATBOX: f32 = 3.9;
pub const ORDER_GUI_CHATBOX_SCROLLBAR: f32 = 3.8;
pub const ORDER_GUI_CHATBOX_BUTTON: f32 = 3.8;
pub const ORDER_GUI_CHATBOX_BUTTON_CONTENT: f32 = 3.7;
pub const ORDER_GUI_CHATBOX_TEXTBOX_BG: f32 = 3.8;
pub const ORDER_GUI_CHATBOX_TEXTBOX: f32 = 3.8;
pub const ORDER_GUI_CHATBOX_TEXTBOX_TEXT: f32 = 3.7;
pub const ORDER_GUI_WINDOW: f32 = 2.999; // The whole 2.9xxx will be used for Game Window
pub const ORDER_ALERT_BG: f32 = 1.9;
pub const ORDER_ALERT: f32 = 1.8;
pub const ORDER_ALERT_HEADER: f32 = 1.7;
pub const ORDER_ALERT_HEADER_TEXT: f32 = 1.6;
pub const ORDER_ALERT_TEXT: f32 = 1.7;
pub const ORDER_ALERT_BUTTON: f32 = 1.6;
pub const ORDER_TOOLTIP: f32 = 1.05;
pub const ORDER_TOOLTIP_TEXT: f32 = 1.0;
pub const ORDER_FADE: f32 = 0.9;
