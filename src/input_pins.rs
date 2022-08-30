use crate::def_plugins::{Input, InputPlugin};
use rppal::gpio::{Gpio, InputPin, Level};

pub struct InputPins {
    pin1: InputPin,
    pin1_stored_state: Level,
    input_state: Input,
}

impl InputPlugin for InputPins {
    fn new() -> Self {
        let gpio = Gpio::new().unwrap();

        Self {
            pin1: gpio.get(27).unwrap().into_input_pullup(),
            pin1_stored_state: Level::High,
            input_state: Input::default(),
        }
    }

    fn poll(&mut self) -> Option<Input> {
        // println!("{} : {}", self.pin1.pin(), self.pin1.read());

        let updated_state = self.pin1.read();

        if updated_state == self.pin1_stored_state {
            None
        } else {
            self.pin1_stored_state = updated_state;
            self.input_state.button = match updated_state {
                Level::Low => true,
                Level::High => false,
            };
            Some(self.input_state)
        }
    }
}
