use crate::frame::{Drawable, Frame};

pub struct Menu {
    pub options: Vec<String>,
    pub selection: usize,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            options: vec![String::from("New game"), String::from("Exit")],
            selection: 0,
        }
    }
    pub fn change_option(&mut self, upwards: bool) {
        if upwards && self.selection > 0 {
            self.selection -= 1;
        } else if !upwards && self.selection < self.options.len() - 1 {
            self.selection += 1;
        }
    }
}