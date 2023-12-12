use std::time::Duration;
use crate::player::Player;
pub trait InvaderPlayer {
     fn shoot(&mut self) -> bool;
     fn update(&mut self, delta: Duration);
     fn detect_hits(&mut self, player: &mut Player);


}