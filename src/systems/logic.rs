use graphics::*;
use winit::dpi::PhysicalSize;

use crate::Direction;

pub trait FloatFix {
    fn add_f32(&self, b: f32, dec: i32) -> Self;
    fn sub_f32(&self, b: f32, dec: i32) -> Self;
}

impl FloatFix for f32 {
    fn add_f32(&self, b: f32, dec: i32) -> Self {
        let a_convert_to_int = (self * 10_f32.powi(dec)) as i32;
        let b_convert_to_int = (b * 10_f32.powi(dec)) as i32;
        let total = a_convert_to_int + b_convert_to_int;
        total as f32 / 10_f32.powi(dec)
    }

    fn sub_f32(&self, b: f32, dec: i32) -> Self {
        let a_convert_to_int = (self * 10_f32.powi(dec)) as i32;
        let b_convert_to_int = (b * 10_f32.powi(dec)) as i32;
        let total = a_convert_to_int - b_convert_to_int;
        total as f32 / 10_f32.powi(dec)
    }
}

#[derive(Debug)]
pub enum InsertTypes {
    Int(i64),
    UInt(u64),
    Str(String),
    Bool(bool),
}

impl InsertTypes {
    pub fn get_int(&self) -> i64 {
        match self {
            InsertTypes::Int(data) => *data,
            _ => 0,
        }
    }

    pub fn get_uint(&self) -> u64 {
        match self {
            InsertTypes::UInt(data) => *data,
            _ => 0,
        }
    }

    pub fn get_string(&self) -> String {
        match self {
            InsertTypes::Str(data) => data.clone(),
            _ => String::new(),
        }
    }
}

pub fn get_screen_center(size: &PhysicalSize<f32>) -> Vec2 {
    Vec2::new((size.width * 0.5).floor(), (size.height * 0.5).floor())
}

pub fn is_within_area(area: Vec2, target_pos: Vec2, target_size: Vec2) -> bool {
    area.x >= target_pos.x
        && area.x <= target_pos.x + target_size.x
        && area.y >= target_pos.y
        && area.y <= target_pos.y + target_size.y
}

pub const fn is_name_acceptable(n: char) -> bool {
    matches!(n, '!' | '$' | '&' | '_' | '~' | '0'..='9' | 'A'..='Z' | 'a'..='z')
}

pub const fn is_password_acceptable(n: char) -> bool {
    matches!(n, '!' | '$' | '&' | '_' | '%' | '@' | '?' | '~' | '0'..='9' | 'A'..='Z' | 'a'..='z')
}

pub const fn dir_to_enum(dir: u8) -> Direction {
    match dir {
        1 => Direction::Right,
        2 => Direction::Up,
        3 => Direction::Left,
        _ => Direction::Down,
    }
}

pub const fn enum_to_dir(dir: Direction) -> u8 {
    match dir {
        Direction::Up => 2,
        Direction::Down => 0,
        Direction::Left => 3,
        Direction::Right => 1,
    }
}

pub fn get_percent(value: i32, max_value: i32, size: i32) -> i32 {
    ((value as f32 / max_value as f32) * size as f32).floor() as i32
}
