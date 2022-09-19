use crate::{
    animations::RgbAnimation,
    def_const::{FRAME_TIME, GRID_HEIGHT, GRID_WIDTH},
    def_plugins::VisPlugin,
};

use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, QueueableCommand,
};
use rgb::RGB8;
use std::{
    io::{stdout, Write},
    time::Instant,
};

pub struct VisCrossterm<'a> {
    metro_animation: &'a RgbAnimation,
    play_once_animation: Option<&'a RgbAnimation>,
    play_once_select_time: Instant,
    last_frame: usize,
}

impl<'a> VisPlugin for VisCrossterm<'a> {
    fn new(metro_animation: &'a RgbAnimation, _brightness: u8) -> VisCrossterm {
        let mut stdout = stdout();
        stdout
            .queue(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        stdout.flush().unwrap();

        Self {
            play_once_select_time: Instant::now(),
            metro_animation,
            last_frame: 0,
            play_once_animation: None,
        }
    }

    fn update(&mut self, quantum: f64, phase: f64) {
        let animation;
        let current_frame;
        if let Some(single_play_animation) = self.play_once_animation {
            animation = single_play_animation;

            // let number_of_frames_in_animation = animation.frames.len();
            let elapsed_time = self.play_once_select_time.elapsed().as_millis() as usize;

            if let Some(_) = animation.frames.get(elapsed_time / FRAME_TIME) {
                current_frame = elapsed_time / FRAME_TIME;
            } else {
                self.play_once_animation = None;
                current_frame = animation.frames.len() - 1;
            }
        } else {
            animation = self.metro_animation;
            let number_of_frames_in_animation = animation.frames.len();
            let bar_percentage = phase / quantum;
            current_frame =
                (number_of_frames_in_animation as f64 * bar_percentage).floor() as usize;
        }

        if current_frame != self.last_frame {
            let mut stdout = stdout();

            stdout
                .queue(cursor::MoveToRow(GRID_HEIGHT as u16))
                .unwrap()
                .queue(terminal::Clear(terminal::ClearType::FromCursorUp))
                .unwrap()
                .queue(cursor::Hide)
                .unwrap();

            for i in 0..(GRID_HEIGHT * GRID_WIDTH) {
                let pixel_color: &RGB8 = animation.frames[current_frame].pixels.get(i).unwrap();

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
                .queue(style::Print(INFO_TEXT))
                .unwrap()
                .queue(cursor::MoveToNextLine(2))
                .unwrap()
                .flush()
                .unwrap();

            self.last_frame = current_frame;
        }
    }

    fn select_metro_loop(&mut self, animation: &'a RgbAnimation) {
        self.metro_animation = animation;
    }

    fn select_single_play(&mut self, animation: &'static RgbAnimation) {
        self.play_once_animation = Some(animation);
        self.play_once_select_time = Instant::now();
    }

    fn set_brightness(&mut self, _value: u8) {}
}

pub const INFO_TEXT: &str = "L: <- / R: -> / Btn: Space / Vol: 1-9 / Quit: q ";
