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
}