use crate::{def_input::Input, def_plugins::InputPlugin};
use rppal::gpio::{Gpio, InputPin, Level};

pub struct InputHardware {
    pin1: InputPin,
    pin1_stored_state: Level,
    input_state: Option<Input>,
}

impl InputPlugin for InputHardware {
    fn new() -> Self {
        let gpio = Gpio::new().unwrap();

        Self {
            pin1: gpio.get(27).unwrap().into_input_pullup(),
            pin1_stored_state: Level::High,
            input_state: None,
        }
    }

    fn poll(&mut self) -> Option<Input> {
        // println!("{} : {}", self.pin1.pin(), self.pin1.read());

        let updated_state = self.pin1.read();

        if updated_state == self.pin1_stored_state {
            None
        } else {
            self.pin1_stored_state = updated_state;
            self.input_state = match updated_state {
                Level::Low => Some(Input::Button),
                Level::High => None,
            };
            self.input_state
        }
    }
}
