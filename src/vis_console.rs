use rgb::RGB8;

use crate::def_const::*;
use crate::def_plugins::{VisPlugin, Visual};

pub struct VisConsole {
    display_buffer: [RGB8; GRID_HEIGHT * GRID_WIDTH],
}

impl VisPlugin for VisConsole {
    fn new() -> Self {
        Self {
            display_buffer: [RGB8::new(0, 0, 0); GRID_HEIGHT * GRID_WIDTH],
        }
    }

    fn update(self, array: &[RGB8; GRID_HEIGHT * GRID_WIDTH]) {
        todo!()
    }

    fn select(&self, visual: Visual) {
        todo!()
    }
}
