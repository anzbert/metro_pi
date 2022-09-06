use crate::{def_plugins::VisPlugin, gifs::RgbAnimation};

pub struct VisNull {}

impl VisPlugin for VisNull {
    fn new(_animation: &RgbAnimation, _brightness: u8) -> Self {
        Self {}
    }

    fn update(&mut self, _quantum: f64, _phase: f64) {}

    fn select(&mut self, _animation: &RgbAnimation) {}

    fn set_brightness(&mut self, _value: u8) {}
}
