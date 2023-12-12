use crate::{
    frame::{Drawable, Frame},
    //invaders::Invaders,
    shot::Shot,
    {NUM_COLS, NUM_ROWS},
};
use std::time::Duration;

pub struct Player {
    pub x: usize,
    pub y: usize,
    
    shots: Vec<Shot>,
    pub killed:bool,
    
}
impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 3,
            shots: Vec::new(),
            killed:false
        }
    }

pub fn move_left(&mut self) {
    if self.x >=1 {
        self.x -= 1;
    }
}





}