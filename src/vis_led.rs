use gif2json::RgbaImageData;
use rgb::RGB8;
use rs_ws281x::*;

use crate::{
    def_const::{GRID_HEIGHT, GRID_WIDTH},
    def_plugins::VisPlugin,
};

pub struct VisLed {
    display_buffer: [RGB8; GRID_HEIGHT * GRID_WIDTH],
    controller: Controller,
}

impl VisPlugin for VisLed {
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
        }
    }

    fn update(&mut self, phase: u32) {
        let leds = self.controller.leds_mut(0);

        for led in leds {
            *led = [0, 255, 255, 0];
        }
        self.controller.render().unwrap();
    }

    fn select(&self, visual: RgbaImageData) {
        todo!()
    }
}
