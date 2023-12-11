use std::time::Duration;



pub trait EggShot{
    fn update(&mut self, delta: Duration);
    fn explode(&mut self);
    fn dead(&self) -> bool;




}