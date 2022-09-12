use crate::{
    animations::RgbAnimation,
    def_const::{GRID_HEIGHT, GRID_WIDTH},
    def_plugins::VisPlugin,
};
use rs_ws281x::*;

pub struct VisLed<'a> {
    metro_animation: &'a RgbAnimation,
    controller: Controller,
    last_frame: usize,
}

impl<'a> VisPlugin for VisLed<'a> {
    fn new(metro_animation: &'a RgbAnimation, brightness: u8) -> VisLed<'a> {
        let controller = ControllerBuilder::new()
            .freq(800_000)
            .dma(10)
            .channel(
                0, // Channel Index
                ChannelBuilder::new()
                    .pin(10) // GPIO 10 = SPI0 MOSI
                    .count(64) // Number of LEDs
                    .strip_type(StripType::Ws2812)
                    .brightness(brightness) // default: 255
                    .build(),
            )
            .build()
            .unwrap();

        Self {
            controller,
            metro_animation,
            last_frame: 0,
        }
    }

    fn update(&mut self, quantum: f64, phase: f64) {
        // 1) get leds:
        let leds = self.controller.leds_mut(0);

        // 2) update leds:
        let number_of_frames_in_animation = self.metro_animation.frames.len();
        let bar_percentage = phase / quantum;
        let current_frame = (number_of_frames_in_animation as f64 * bar_percentage) as usize;

        if current_frame != self.last_frame {
            for (i, led) in leds.iter_mut().enumerate() {
                // *led = [0, 255, 255, 0]; // <- example
                let pixel_color = self.metro_animation.frames[current_frame]
                    .pixels
                    .get(i)
                    .unwrap();
                *led = [pixel_color.r, pixel_color.g, pixel_color.b, 255];
            }

            // 3) render:
            self.controller.render().unwrap();
            self.last_frame = current_frame;
        }
    }

    fn select(&mut self, animation: &'a RgbAnimation) {
        self.metro_animation = animation;
        let pixels_in_first_gif_frame = self.metro_animation.frames.get(0).unwrap().pixels.len();
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

    fn set_brightness(&mut self, value: u8) {
        self.controller.set_brightness(0, value);
    }
}
