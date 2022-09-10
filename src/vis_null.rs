use crate::{animations::RgbAnimation, def_plugins::VisPlugin};

pub struct VisNull {}

impl VisPlugin for VisNull {
    fn new(_animation: &RgbAnimation, _brightness: u8) -> Self {
        Self {}
    }

    fn select(&mut self, _animation: &RgbAnimation) {}

    fn set_brightness(&mut self, _value: u8) {}

    fn update(&mut self, quantum: f64, phase: f64) {}

    // fn show_text(&mut self, _textAnimation: &RgbAnimation) {}
}
