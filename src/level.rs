use crate::frame::{Drawable, Frame};

const MAX_LEVEL: u8 = 3;

pub struct Level {
    level: u8,
}

impl Level {
    pub fn new() -> Self {
        Self { level: 1 }
    }
    pub fn increment_level(&mut self) -> bool {
        if self.level <= MAX_LEVEL {
            self.level += 1;
        }
        self.level == MAX_LEVEL
    }





}