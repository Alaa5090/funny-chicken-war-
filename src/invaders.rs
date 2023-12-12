use crate::{
    frame::{Drawable, Frame},
    {NUM_COLS, NUM_ROWS},
    egg::Egg, player::Player, invader_player::InvaderPlayer,egg_shot::EggShot,
};
use rusty_time::timer::Timer;
use std::{cmp::max, time::Duration};

pub struct Invader {
    pub x: usize,
    pub y: usize,
    points: u16,
    eggs: Vec<Egg>,
    
}

impl  Invader{
    pub fn new(x:usize,y:usize,points:u16) -> Self {
        Self {
            x,  
            y,
            points,
            eggs: Vec::new(),
        }
    } 
} 
impl InvaderPlayer for Invader {
    fn shoot(&mut self) -> bool {
        if self.eggs.len() < 2 {
            self.eggs.push(Egg::new(self.x, self.y - 1));
            true
        } else {
            false
        }
    }
    fn update(&mut self, delta: Duration) {
        for egg in self.eggs.iter_mut() {
            egg.update(delta);
        }
        self.eggs.retain(|egg| !egg.dead());
    }

    fn detect_hits(&mut self, player: &mut Player) {
        todo!()
    }
     
}  


