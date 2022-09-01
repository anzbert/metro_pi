use crate::{
    def_const::{GRID_HEIGHT, GRID_WIDTH},
    def_plugins::VisPlugin,
    gifs::{self, RgbaImageData, Visualization},
};

use termion::{clear, color, cursor};

pub struct VisConsole<'a> {
    gif: &'a RgbaImageData,
    last_frame: usize,
}

impl<'a> VisPlugin for VisConsole<'a> {
    fn new(visual: Visualization, _brightness: u8) -> VisConsole<'a> {
        Self {
            gif: gifs::GIFS.get(&visual).unwrap(),
            last_frame: 0,
        }
    }

    fn update(&mut self, quantum: f64, phase: f64) {
        let number_of_frames_in_animation = self.gif.frames.len();
        let bar_percentage = phase / quantum;
        let current_frame = (number_of_frames_in_animation as f64 * bar_percentage) as usize;

        if current_frame != self.last_frame {
            println!("\n{}{}", cursor::Hide, clear::All,);

            for i in 0..(GRID_HEIGHT * GRID_WIDTH) {
                let pixel_color: &(u8, u8, u8, u8) =
                    self.gif.frames[current_frame].pixels.get(i).unwrap();
                // let termion_color =
                //     termion::color::Rgb(pixel_color.0, pixel_color.1, pixel_color.2);
                let termion_color = termion::color::AnsiValue::rgb(
                    pixel_color.0 / 51,
                    pixel_color.1 / 51,
                    pixel_color.2 / 51,
                );

                let x = i % (GRID_WIDTH) + 1;
                let y = i / (GRID_HEIGHT) + 1;

                println!(
                    "{}{}{}",
                    cursor::Goto(x as u16 * 2, y as u16),
                    color::Fg(termion_color),
                    "⬤"
                );
            }
        }
    }

    fn select(&mut self, visual: Visualization) {
        self.gif = gifs::GIFS.get(&visual).unwrap();
    }

    fn set_brightness(&mut self, _value: u8) {}
}
