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
pub fn move_right(&mut self) {
    if self.x < NUM_COLS - 1 {
        self.x += 1;
    }
}
pub fn kill_player_at(&mut self, x: usize, y: usize) -> bool {
    if  self.x==x && self.y==y//position(|player| (player.x == x) && (player.y == y))
    {
        self.killed=true;
        self.x=NUM_COLS / 2;
        self.y=NUM_ROWS - 1;
        true
    } 
    else {
        false
    }
}




}