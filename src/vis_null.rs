use crate::{animations::RgbAnimation, def_plugins::VisPlugin};

pub struct VisNull {}

impl VisPlugin for VisNull {
    fn new(animation: &'static RgbAnimation, brightness: u8) -> Self {
        Self {}
    }

    fn update(&mut self, quantum: f64, phase: f64) {}

    fn select_metro_loop(&mut self, animation: &'static RgbAnimation) {}

    fn select_single_play(&mut self, animation: &'static RgbAnimation) {}

    fn set_brightness(&mut self, value: u8) {}
}
