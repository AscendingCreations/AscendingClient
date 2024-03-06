use serde::{Deserialize, Serialize};
use crate::game_content::entity::*;

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
    PartialEq,
    Eq,
    Default,
    Deserialize,
    Serialize,
)]
pub enum EntityType {
    #[default]
    None,
    Player(Entity),
    Npc(Entity),
}

pub const ORDER_MENU_BG: f32 = 10.9;
pub const ORDER_MENU_WINDOW: f32 = 10.8;
pub const ORDER_MENU_WINDOW_CONTENT: f32 = 10.7;
pub const ORDER_MENU_WINDOW_CONTENT_DETAIL: f32 = 10.6;
pub const ORDER_MENU_WINDOW_CONTENT_DETAIL2: f32 = 10.5;
// Lower Map Order 9.4 - 9.0
pub const ORDER_PLAYER: f32 = 8.0;
pub const ORDER_NPC: f32 = 8.0;
// Upper Map Order 5.1 - 5.0
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
pub const ORDER_FADE: f32 = 0.9;