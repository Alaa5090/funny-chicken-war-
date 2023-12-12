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