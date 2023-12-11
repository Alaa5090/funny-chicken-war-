use crate::frame::{Drawable, Frame};
use crate::egg_shot::EggShot;
use rusty_time::timer::Timer;
use std::time::Duration;

pub struct Egg {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    pub broken:bool,
    timer: Timer,
}
impl Egg { 
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            broken:false,
            timer: Timer::from_millis(50),
        }
    } 

pub fn omlet(&mut self) {
    self.broken = true;
    self.timer = Timer::from_millis(250);
 }
}
impl EggShot for Egg{
     fn update(&mut self, delta: Duration) {
        self.timer.update(delta/8);
        if self.timer.ready && !self.exploding {
            if self.y <= 20 {
                self.y += 1;
            }
            if self.y == 18 {
                self.omlet();
            }
            self.timer.reset();
        }
    }

    fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::from_millis(250);
    }
    fn dead(&self) -> bool {
        (self.exploding && self.timer.ready) || (self.y == 19)
    }
}

impl Drawable for Egg{
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding { '🐣' } else { '🥚' };
        if self.broken{
            frame[self.x][self.y] = '🍳';
        }
    }


}
    