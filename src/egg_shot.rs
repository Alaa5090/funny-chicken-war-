

pub trait EggShot{
    pub fn update(&mut self, delta: Duration);
    pub fn explode(&mut self);
    pub fn dead(&self) -> bool;




}