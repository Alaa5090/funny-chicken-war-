use crate::{
    frame::{Drawable, Frame},
    {NUM_COLS, NUM_ROWS},
    egg::Egg, player::Player,
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


