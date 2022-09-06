use rodio::Decoder;
use rodio::*;
use std::io::Cursor;
use std::thread;

pub struct Sound {
    sound: Cursor<Vec<u8>>,
}
impl Sound {
    pub fn new(path: &str) -> Self {
        Self {
            sound: Cursor::new(std::fs::read(path).unwrap()),
        }
    }
}

pub struct AudioPlayer {
    volume: f32,
}

impl AudioPlayer {
    pub fn new() -> Self {
        Self { volume: 1. }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0., 1.);
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn play(&self, handle: &OutputStreamHandle, sound: &Sound) {
        let sound1 = sound.sound.clone();
        let sink = Sink::try_new(&handle).unwrap();
        sink.set_volume(self.volume);
        let source = Decoder::new(sound1).unwrap();
        sink.append(source);

        thread::spawn(move || {
            sink.sleep_until_end();
        });
    }
}
