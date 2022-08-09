pub trait InputPlugin {
    fn new() -> Self;
    fn poll(&self) -> Option<Input>;
}

#[derive(Debug)]
pub struct Input {
    pub left: bool,
    pub right: bool,
    pub button: bool,
}

impl Input {
    pub fn new(left: bool, right: bool, button: bool) -> Self {
        Self {
            left,
            right,
            button,
        }
    }
}
