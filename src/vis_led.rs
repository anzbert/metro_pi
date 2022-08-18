use crate::{
    def_const::{GRID_HEIGHT, GRID_WIDTH},
    def_plugins::VisPlugin,
    gifs::{self, RgbaImageData, Visualization},
};
use rgb::RGB8;
use rs_ws281x::*;

pub struct VisLed<'a> {
    gif: &'a RgbaImageData,
    controller: Controller,
    last_frame: usize,
}

impl VisPlugin for VisLed<'_> {
    fn new() -> Self {
        Self {
            controller: ControllerBuilder::new()
                .freq(800_000)
                .dma(10)
                .channel(
                    0, // Channel Index
                    ChannelBuilder::new()
                        .pin(10) // GPIO 10 = SPI0 MOSI
                        .count(64) // Number of LEDs
                        .strip_type(StripType::Ws2812)
                        .brightness(8) // default: 255
                        .build(),
                )
                .build()
                .unwrap(),
            gif: gifs::GIFS.get(&Visualization::default()).unwrap(),
            last_frame: 0,
        }
    }

    fn update(&mut self, quantum: f64, phase: f64) {
        // 1) get leds:
        let leds = self.controller.leds_mut(0);

        // 2) update leds:
        let number_of_frames_in_animation = self.gif.frames.len();
        let bar_percentage = phase / quantum;
        let current_frame = (number_of_frames_in_animation as f64 * bar_percentage) as usize;

        if current_frame != self.last_frame {
            for (i, led) in leds.iter_mut().enumerate() {
                // *led = [0, 255, 255, 0]; // <- example
                let pixel_color = self.gif.frames[current_frame].pixels.get(i).unwrap();
                *led = [pixel_color.0, pixel_color.1, pixel_color.2, pixel_color.3];
            }

            // 3) render:
            self.controller.render().unwrap();
            self.last_frame = current_frame;
        }
    }

    fn select(&mut self, visual: Visualization) {
        self.gif = gifs::GIFS.get(&visual).unwrap();
        let pixels_in_first_gif_frame = self.gif.frames.get(0).unwrap().pixels.len();
        if GRID_HEIGHT * GRID_WIDTH != pixels_in_first_gif_frame {
            panic!(
                "led matrix ({} x {} = {}) does not match gif pixel size ({})",
                GRID_HEIGHT,
                GRID_WIDTH,
                GRID_HEIGHT * GRID_WIDTH,
                pixels_in_first_gif_frame
            );
        }
    }
}
