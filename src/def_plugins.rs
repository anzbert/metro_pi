use crate::gifs::Visualization;

// INPUT
pub trait InputPlugin {
    fn new() -> Self;
    fn poll(&mut self) -> Option<Input>;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

impl Default for Input {
    fn default() -> Self {
        Self {
            left: false,
            right: false,
            button: false,
        }
    }
}

// VIS
pub trait VisPlugin {
    fn new(visual: Visualization, brightness: u8) -> Self;
    fn update(&mut self, quantum: f64, phase: f64);
    fn select(&mut self, visual: Visualization);
    fn set_brightness(&mut self, value: u8);
}
