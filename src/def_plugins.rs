use std::time::Duration;

use crate::{animations::RgbAnimation, def_input::Input};
pub trait InputPlugin {
    fn new() -> Self;
    fn poll(&mut self) -> Option<Input>;
}
// VIS
pub trait VisPlugin {
    fn new(animation: &'static RgbAnimation, brightness: u8) -> Self;
    fn update(&mut self, quantum: f64, phase: f64);
    fn select_metro_loop(&mut self, animation: &'static RgbAnimation);
    fn select_single_play(&mut self, animation: &'static RgbAnimation);
    fn set_brightness(&mut self, value: u8);
}
