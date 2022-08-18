use std::collections::HashMap;

use gif2json::RgbaImageData;
use rgb::RGB8;
use rs_ws281x::*;

use crate::{
    def_const::{GRID_HEIGHT, GRID_WIDTH},
    def_plugins::VisPlugin,
    gifs::{self, Visualization},
};

pub struct VisLed<'a> {
    display_buffer: [RGB8; GRID_HEIGHT * GRID_WIDTH],
    gif: &'a RgbaImageData,
    controller: Controller,
}

impl VisPlugin for VisLed<'_> {
    fn new() -> Self {
        Self {
            display_buffer: [RGB8::new(0, 0, 0); GRID_HEIGHT * GRID_WIDTH],
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
        }
    }

    fn update(&mut self, quantum: f64, phase: f64) {
        // 1) get leds:
        let leds = self.controller.leds_mut(0);

        // 2) update leds:
        // let current_gif_frames = self.

        // for led in leds {
        //     *led = [0, 255, 255, 0];
        // }

        // 3) render:
        self.controller.render().unwrap();
    }

    fn select(&mut self, visual: Visualization) {
        self.gif = gifs::GIFS.get(&visual).unwrap();
    }
}
