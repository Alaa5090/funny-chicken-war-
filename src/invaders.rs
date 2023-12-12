use crate::{
    egg::Egg,
    egg_shot::EggShot,
    frame::{Drawable, Frame},
    invader_player::InvaderPlayer,
    player::Player,
    {NUM_COLS, NUM_ROWS},
};
use rusty_time::timer::Timer;
use std::{cmp::max, time::Duration};

pub struct Invader {
    pub x: usize,
    pub y: usize,
    points: u16,
    eggs: Vec<Egg>,
}

impl Invader {
    pub fn new(x: usize, y: usize, points: u16) -> Self {
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
        for egg in self.eggs.iter_mut() {
            if !egg.exploding {
                let is_hitted = player.kill_player_at(egg.x, egg.y);
                if is_hitted {
                    //hit_something += hit_count;
                    egg.explode();
                }
            }
        }
    }
}

pub struct Invaders {
    pub army: Vec<Invader>,
    pub total_count: usize,
    move_timer: Timer,
    direction: i32,
}
impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 10..NUM_COLS + 10 {
            for y in 0..NUM_ROWS {
                if (x > 1)
                    && (x < NUM_COLS - 2)
                    && (y > 0)
                    && (y < 9)
                    && (x % 2 == 0)
                    && (y % 2 == 0)
                {
                    army.push(Invader::new(x, y, 1));
                }
            }
        }
        let total_count = army.len();
        Self {
            army,
            total_count,
            move_timer: Timer::from_millis(2000),
            direction: 1,
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            let mut downwards = false;
            if self.direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }

            if downwards {
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            }
            // } else {
            //     for invader in self.army.iter_mut() {
            //         invader.x = ((invader.x as i32) + self.direction) as usize;

            //     }
            //}
            return true;
        }
        false
    }
    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }
    pub fn reached_bottom(&self) -> bool {
        self.army.iter().any(|invader| invader.y >= NUM_ROWS - 1)
    }

}
