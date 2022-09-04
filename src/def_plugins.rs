use crate::{def_input::Input, gifs::Visualization};
pub trait InputPlugin {
    fn new() -> Self;
    fn poll(&mut self) -> Option<Input>;
}
// VIS
pub trait VisPlugin {
    fn new(visual: Visualization, brightness: u8) -> Self;
    fn update(&mut self, quantum: f64, phase: f64);
    fn select(&mut self, visual: Visualization);
    fn set_brightness(&mut self, value: u8);
}
