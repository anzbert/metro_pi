use crate::{animations::RgbAnimation, def_plugins::VisPlugin};

pub struct VisNull {}

impl VisPlugin for VisNull {
    fn new(_animation: &'static RgbAnimation, _brightness: u8) -> Self {
        Self {}
    }

    fn update(&mut self, _quantum: f64, _phase: f64) {}

    fn select_metro_loop(&mut self, _animation: &'static RgbAnimation) {}

    fn select_single_play(&mut self, _animation: &'static RgbAnimation) {}

    fn set_brightness(&mut self, _value: u8) {}
}
