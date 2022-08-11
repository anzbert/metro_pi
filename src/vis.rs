use crate::def_settings::Visualization;
use gif2json::RgbaImageData;
use std::collections::HashMap;

pub fn init_gifs() -> HashMap<Visualization, RgbaImageData> {
    let mut map = HashMap::new();

    map.insert(
        Visualization::Clock,
        RgbaImageData::new_from_bytes(include_bytes!("../img/clock.gif")).unwrap(),
    );
    map.insert(
        Visualization::Counter,
        RgbaImageData::new_from_bytes(include_bytes!("../img/counter_alpha.gif")).unwrap(),
    );
    map.insert(
        Visualization::Rows,
        RgbaImageData::new_from_bytes(include_bytes!("../img/rows_alpha.gif")).unwrap(),
    );
    map.insert(
        Visualization::Circular,
        RgbaImageData::new_from_bytes(include_bytes!("../img/circular.gif")).unwrap(),
    );

    map
}

// pub struct Leds {
//     grid: [[RGB8; GRID_HEIGHT]; GRID_WIDTH],
// }

// #[allow(dead_code)]
// impl Leds {
//     pub fn new() -> Self {
//         Self {
//             grid: [[RGB8::default(); 8]; 8],
//         }
//     }

//     pub fn get_mut_ref_rgb8(&mut self, x: usize, y: usize) -> &mut RGB8 {
//         &mut self.grid[x][y]
//     }

//     pub fn update_off(&mut self) {
//         for x in 0..GRID_WIDTH {
//             for y in 0..GRID_HEIGHT {
//                 self.update_pixel(x, y, RGB8::new(0, 0, 0))
//             }
//         }
//     }

//     pub fn update_clockwise(&mut self, percentage: f32, color: RGB8) {
//         let perc = if percentage > 1.0 {
//             percentage - 1.0
//         } else {
//             percentage
//         };

//         let phase = (CLOCK.len() as f32 * perc).ceil() as usize;
//         for (x, y) in &CLOCK[0..phase] {
//             *self.get_mut_ref_rgb8(*x, *y) = color;
//         }
//     }

//     pub fn update_with_image(&mut self, rgb_vector: Vec<(u8, u8, u8, u8)>) {
//         for (index, pixel) in rgb_vector.iter().enumerate() {
//             if pixel.3 == 0 {
//                 continue;
//             }
//             let rgb = RGB8::new(pixel.0, pixel.1, pixel.2);
//             let (x, y) = array_to_coord(index, GRID_WIDTH, GRID_HEIGHT);
//             self.update_pixel(x, y, rgb);
//         }
//     }

//     pub fn update_pixel(&mut self, x: usize, y: usize, new_color: RGB8) {
//         self.grid[x][y] = new_color;
//     }
// }
