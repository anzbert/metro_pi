use crate::def_plugins::VisPlugin;

pub struct VisNull {}

impl VisPlugin for VisNull {
    fn new(_visual: crate::gifs::Visualization, _brightness: u8) -> Self {
        Self {}
    }

    fn update(&mut self, _quantum: f64, _phase: f64) {}

    fn select(&mut self, _visual: crate::gifs::Visualization) {}

    fn set_brightness(&mut self, _value: u8) {}
}
