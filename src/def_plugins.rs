use crate::def_const::{GRID_HEIGHT, GRID_WIDTH};
use rgb::RGB8;

pub struct Plugins<T: InputPlugin, U: VisPlugin> {
    pub input: T,
    pub vis: U,
    // sound: V,
}

// INPUT
pub trait InputPlugin {
    fn new() -> Self;
    fn poll(&self) -> Option<Input>;
}

#[derive(Debug)]
pub struct Input {
    pub left: bool,
    pub right: bool,
    pub button: bool,
}

impl Input {
    pub fn new(left: bool, right: bool, button: bool) -> Self {
        Self {
            left,
            right,
            button,
        }
    }
}

// VIS
pub trait VisPlugin {
    fn new() -> Self;
    fn update(self, array: &[RGB8; GRID_HEIGHT * GRID_WIDTH]);
    fn select(&self, visual: Visual);
}

pub enum Visual {
    Lines,
    Circle,
}
