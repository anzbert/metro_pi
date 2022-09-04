use crate::{def_input::Input, def_plugins::InputPlugin};

pub struct InputNull {}
impl InputPlugin for InputNull {
    fn new() -> Self {
        InputNull {}
    }

    fn poll(&mut self) -> Option<Input> {
        None
    }
}
