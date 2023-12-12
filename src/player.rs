use crate::{
    egg_shot::EggShot,
    frame::{Drawable, Frame},
    invader_player::InvaderPlayer,
    invaders::Invaders,
    shot::Shot,
    {NUM_COLS, NUM_ROWS},
};
use std::time::Duration;

pub struct Player {
    pub x: usize,
    pub y: usize,

    shots: Vec<Shot>,
    pub killed: bool,
}
impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 3,
            shots: Vec::new(),
            killed: false,
        }
    }

    pub fn move_left(&mut self) {
        if self.x >= 1 {
            self.x -= 1;
        }
    }
    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }
    pub fn kill_player_at(&mut self, x: usize, y: usize) -> bool {
        if self.x == x && self.y == y
        //position(|player| (player.x == x) && (player.y == y))
        {
            self.killed = true;
            self.x = NUM_COLS / 2;
            self.y = NUM_ROWS - 1;
            true
        } else {
            false
        }
    }
}
impl InvaderPlayer for Player {
    fn shoot(&mut self) -> bool {
        if self.shots.len() < 2 {
            self.shots.push(Shot::new(self.x, self.y - 1));
            true
        } else {
            false
        }
    }
    fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }
    fn detect_hits(&mut self, invaders: &mut Invaders) -> u16 {
        let mut hit_something = 0u16;
        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                let hit_count = invaders.kill_invader_at(shot.x, shot.y);
                if hit_count > 0 {
                    hit_something += hit_count;
                    shot.explode();
                }
            }
        }
        hit_something
    }
}
impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        if self.killed{
            frame[self.x][self.y] = '💥';
        }
        else{
            frame[self.x][self.y] = '🚀';
        
            for shot in self.shots.iter() {
                shot.draw(frame);
            }
        }
        
    }
}
