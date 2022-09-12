#[derive(Copy, Clone)]
pub enum Input {
    Left,
    Right,
    Up,
    Down,
    Button,
    Volume(f32),
}
