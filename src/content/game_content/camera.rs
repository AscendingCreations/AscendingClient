use graphics::*;

use crate::{Direction, TILE_SIZE};

#[derive(Clone, Debug)]
pub struct Camera {
    pub pos: Vec2,
    /*target_pos: Vec2,
    pub did_move: bool,
    cur_pos: Vec2,
    // The below value hold the amount of position
    // being added when camera is moving
    move_offset: f32,
    // The below timer handles the interval of how
    // frequent the camera_move is being processed
    timer: f32,
    interval: f32,*/
}

impl Camera {
    pub fn new(tile_pos: Vec2) -> Self {
        Self {
            pos: tile_pos * TILE_SIZE as f32,
            /*target_pos: pos,
            cur_pos: pos,
            did_move: false,
            move_offset: 0.0,
            timer: 0.0,
            interval: 0.0,*/
        }
    }

    /*
    pub fn dir_move(&mut self, dir: &Direction, timer: f32, move_offset: f32, forced: bool) {
        match dir {
            Direction::Up => {self.target_pos += Vec2::new(0.0, 1.0);},
            Direction::Down => {self.target_pos += Vec2::new(0.0, -1.0);},
            Direction::Left => {self.target_pos += Vec2::new(-1.0, 0.0);},
            Direction::Right => {self.target_pos += Vec2::new(1.0, 0.0);},
        }
        self.move_offset = move_offset;
        self.timer = timer;
        if forced {
            self.pos = self.target_pos.clone();
        } else {
            self.did_move = true;
        }
    }

    pub fn camera_move(&mut self, seconds: f32) -> bool {
        if !self.did_move ||
            self.interval + self.timer > seconds ||
            self.pos == self.target_pos {
            return false;
        }
        self.interval = seconds;

        // Update x position
        if self.target_pos.x > self.pos.x {
            self.pos.x += self.move_offset;
            self.pos.x = self.pos.x.min(self.target_pos.x);
        } else {
            self.pos.x -= self.move_offset;
            self.pos.x = self.pos.x.max(self.target_pos.x);
        }

        // Update y position
        if self.target_pos.y > self.pos.y {
            self.pos.y += self.move_offset;
            self.pos.y = self.pos.y.min(self.target_pos.y);
        } else {
            self.pos.y -= self.move_offset;
            self.pos.y = self.pos.y.max(self.target_pos.y);
        }

        self.offset = self.cur_pos - self.pos;

        if self.pos == self.target_pos {
            self.cur_pos = self.pos.clone();
            self.offset = Vec2::new(0.0, 0.0);
            self.did_move = false;
        }
        true
    }
     */
}
