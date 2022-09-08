use crate::{
    animations::{RgbAnimation, VisType},
    def_const::{GRID_HEIGHT, GRID_WIDTH},
    def_plugins::VisPlugin,
};
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, QueueableCommand,
};
use rgb::RGB8;
use std::io::{stdout, Write};

pub struct VisCrossterm {
    animation: &'static RgbAnimation,
    last_frame: usize,
}

impl VisPlugin for VisCrossterm {
    fn new(animation: &'static RgbAnimation, _brightness: u8) -> VisCrossterm {
        let mut stdout = stdout();
        stdout
            .queue(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        stdout.flush().unwrap();

        Self {
            animation,
            last_frame: 0,
        }
    }

    fn update(&mut self, quantum: f64, phase: f64) {
        if self.animation.playback == VisType::BeatIndependent {}

        let number_of_frames_in_animation = self.animation.frames.len();
        let bar_percentage = phase / quantum;
        let current_frame: usize =
            (number_of_frames_in_animation as f64 * bar_percentage).floor() as usize;

        if current_frame != self.last_frame {
            // println!("frame: {}", current_frame);
            let mut stdout = stdout();

            stdout
                .queue(cursor::MoveToRow(GRID_HEIGHT as u16))
                .unwrap()
                .queue(terminal::Clear(terminal::ClearType::FromCursorUp))
                .unwrap()
                .queue(cursor::Hide)
                .unwrap();

            for i in 0..(GRID_HEIGHT * GRID_WIDTH) {
                let pixel_color: &RGB8 =
                    &self.animation.frames[current_frame].pixels.get(i).unwrap();

                let x = (i % (GRID_WIDTH) + 1) * 2;
                let y = i / (GRID_HEIGHT) + 1;

                // Using ansi colors for compatibility with macos terminal, which doesnt support full RGB
                let ansi_color = style::Color::AnsiValue(
                    coolor::Rgb::new(pixel_color.r, pixel_color.g, pixel_color.b)
                        .to_ansi()
                        .code,
                );

                stdout
                    .queue(cursor::MoveTo(x as u16, y as u16))
                    .unwrap()
                    .queue(style::PrintStyledContent("⬤".with(ansi_color)))
                    .unwrap();
            }
            stdout
                .queue(cursor::MoveToNextLine(2))
                .unwrap()
                .queue(style::Print(INFO))
                .unwrap()
                .queue(cursor::MoveToNextLine(2))
                .unwrap()
                .flush()
                .unwrap();

            self.last_frame = current_frame;
        }
    }

    fn select(&mut self, animation: &'static RgbAnimation) {
        self.animation = animation;
    }

    fn set_brightness(&mut self, _value: u8) {}
}

pub const INFO: &str = "L: <- / R: -> / Btn: Space / Vol: 1-9 / Quit: q ";
