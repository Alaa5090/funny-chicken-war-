use crate::frame::{Drawable, Frame,EggShot};
use rusty_time::timer::Timer;
use std::time::Duration;

pub struct Egg {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    pub broken:bool,
    timer: Timer,
}