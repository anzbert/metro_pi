use crate::{
    def_const::{GRID_HEIGHT, GRID_WIDTH},
    def_plugins::VisPlugin,
    gifs::{self, RgbaImageData, Visualization},
};

// use termion::{clear, color, cursor};
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::io::{stdout, Write};

pub struct VisCons<'a> {
    gif: &'a RgbaImageData,
    last_frame: usize,
}

impl<'a> VisPlugin for VisCons<'a> {
    fn new(visual: Visualization, _brightness: u8) -> VisCons<'a> {
        Self {
            gif: gifs::GIFS.get(&visual).unwrap(),
            last_frame: 0,
        }
    }

    fn update(&mut self, quantum: f64, phase: f64) {
        let number_of_frames_in_animation = self.gif.frames.len();
        let bar_percentage = phase / quantum;
        let current_frame: usize =
            (number_of_frames_in_animation as f64 * bar_percentage).floor() as usize;

        if current_frame != self.last_frame {
            // println!("frame: {}", current_frame);
            let mut stdout = stdout();

            stdout
                .queue(terminal::Clear(terminal::ClearType::All))
                .unwrap()
                .queue(cursor::Hide)
                .unwrap();

            for i in 0..(GRID_HEIGHT * GRID_WIDTH) {
                let pixel_color: &(u8, u8, u8, u8) =
                    self.gif.frames[current_frame].pixels.get(i).unwrap();
                // let termion_color =
                //     termion::color::Rgb(pixel_color.0, pixel_color.1, pixel_color.2);
                // let termion_color = termion::color::AnsiValue::rgb(
                //     pixel_color.0 / 51,
                //     pixel_color.1 / 51,
                //     pixel_color.2 / 51,
                // );

                let x = i % (GRID_WIDTH) + 1;
                let y = i / (GRID_HEIGHT) + 1;

                stdout
                    .queue(style::SetForegroundColor(style::Color::Rgb {
                        r: pixel_color.0,
                        g: pixel_color.1,
                        b: pixel_color.2,
                    }))
                    .unwrap()
                    .queue(cursor::MoveTo(x as u16 * 2, y as u16))
                    .unwrap()
                    .queue(style::PrintStyledContent("⬤".stylize()))
                    .unwrap();
            }
            stdout.flush().unwrap();

            self.last_frame = current_frame;
        }
    }

    fn select(&mut self, visual: Visualization) {
        self.gif = gifs::GIFS.get(&visual).unwrap();
    }

    fn set_brightness(&mut self, _value: u8) {}
}
