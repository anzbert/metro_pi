#[derive(Copy, Clone)]
pub enum Input {
    Quit,
    Left,
    Right,
    Up,
    Down,
    Button,
    Volume(f32),
}
