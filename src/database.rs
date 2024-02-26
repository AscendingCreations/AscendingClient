pub mod map;

pub use map::*;

pub struct Database {
    pub map: Vec<MapData>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            map: Vec::with_capacity(9),
        }
    }

    pub fn load_map(&mut self, x: i32, y: i32, group: u64) {
        for i in 0..9 {
            let (mx, my) = match i {
                1 => (x - 1, y - 1), // Top Left
                2 => (x, y - 1), // Top
                3 => (x + 1, y - 1), // Top Right
                4 => (x - 1, y), // Left
                5 => (x + 1, y), // Right
                6 => (x - 1, y + 1), // Bottom Left
                7 => (x, y + 1), // Bottom
                8 => (x + 1, y + 1), // Bottom Right
                _ => (x, y), // Center
            };
            self.map.push(load_file(mx, my, group));
        }
    }
}