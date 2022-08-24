use crate::def_plugins::{Input, InputPlugin};
use rppal::gpio::{Gpio, InputPin};

pub struct InputPins {
    pin1: InputPin,
}

impl InputPlugin for InputPins {
    fn new() -> Self {
        let gpio = Gpio::new().unwrap();

        Self {
            pin1: gpio.get(27).unwrap().into_input_pullup(),
        }
    }

    fn poll(&self) -> Option<Input> {
        println!("{} : {}", self.pin1.pin(), self.pin1.read());
        Some(Input::default())
    }
}
