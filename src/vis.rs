use crate::constants::*;
use rand::{thread_rng, Rng};

/// Grid by index to Tuple(x,y)
fn array_to_coord(input: usize, grid_size_x: usize, grid_size_y: usize) -> (usize, usize) {
    let x = input % grid_size_x;
    let y = input / grid_size_y;
    (x, y)
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct RGB8 {
    r: u8,
    g: u8,
    b: u8,
}

#[allow(dead_code)]
impl RGB8 {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub fn default() -> Self {
        Self {
            r: 50,
            g: 50,
            b: 50,
        }
    }
    pub fn new_rnd() -> Self {
        let mut rng = thread_rng();
        Self {
            r: rng.gen_range(0..=255),
            g: rng.gen_range(0..=255),
            b: rng.gen_range(0..=255),
        }
    }
}

pub struct Leds {
    grid: [[RGB8; GRID_HEIGHT]; GRID_WIDTH],
}

#[allow(dead_code)]
impl Leds {
    pub fn new() -> Self {
        Self {
            grid: [[RGB8::default(); 8]; 8],
        }
    }

    pub fn get_mut_ref_rgb8(&mut self, x: usize, y: usize) -> &mut RGB8 {
        &mut self.grid[x][y]
    }

    pub fn update_off(&mut self) {
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                self.update_pixel(x, y, RGB8::new(0, 0, 0))
            }
        }
    }

    pub fn update_clockwise(&mut self, percentage: f32, color: RGB8) {
        let perc = if percentage > 1.0 {
            percentage - 1.0
        } else {
            percentage
        };

        let phase = (CLOCK.len() as f32 * perc).ceil() as usize;
        for (x, y) in &CLOCK[0..phase] {
            *self.get_mut_ref_rgb8(*x, *y) = color;
        }
    }

    pub fn update_with_image(&mut self, rgb_vector: Vec<(u8, u8, u8, u8)>) {
        for (index, pixel) in rgb_vector.iter().enumerate() {
            if pixel.3 == 0 {
                continue;
            }
            let rgb = RGB8::new(pixel.0, pixel.1, pixel.2);
            let (x, y) = array_to_coord(index, GRID_WIDTH, GRID_HEIGHT);
            self.update_pixel(x, y, rgb);
        }
    }

    pub fn update_pixel(&mut self, x: usize, y: usize, new_color: RGB8) {
        self.grid[x][y] = new_color;
    }
}
