use crate::def_plugins::{Input, InputPlugin};

pub struct InputNull {}
impl InputPlugin for InputNull {
    fn new() -> Self {
        InputNull {}
    }

    fn poll(&mut self) -> Option<Input> {
        None
    }
}
