use crate::def_const::{GRID_LENGTH, GRID_WIDTH, RING};
use crate::utilities::coord_to_index;
use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, Pixel};
use lazy_static::lazy_static;
use std::error::Error;
use std::result::Result;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum VisType {
    Full,
    Partial,
    Static,
}

lazy_static! {
    pub static ref ANIMATIONS: Vec<RgbAnimation> = {
        let mut vec = Vec::new();
        vec.push(
            RgbAnimation::new_from_bytes(include_bytes!("../img/clock.gif"), VisType::Full)
                .unwrap(),
        );
        vec.push(
            RgbAnimation::new_from_bytes(
                include_bytes!("../img/counter_alpha.gif"),
                VisType::Partial,
            )
            .unwrap(),
        );
        vec.push(
            RgbAnimation::new_from_bytes(include_bytes!("../img/rows_alpha.gif"), VisType::Full)
                .unwrap(),
        );
        vec.push(
            RgbAnimation::new_from_bytes(include_bytes!("../img/circular.gif"), VisType::Full)
                .unwrap(),
        );
        vec.push(Ring::construct());

        return vec;
    };
}

pub struct RgbAnimation {
    pub playback: VisType,
    pub frames: Vec<Vec<(u8, u8, u8)>>,
}
impl RgbAnimation {
    pub fn new_from_bytes(bytes: &[u8], playback: VisType) -> Result<RgbAnimation, Box<dyn Error>> {
        let decoder = GifDecoder::new(bytes)?;

        let frames = decoder.into_frames().collect_frames()?;

        let mut output = Self::default();
        output.playback = playback;

        for frame in frames.iter() {
            let image_buffer = frame.buffer();

            let pixels_as_rgba_vec: Vec<(u8, u8, u8)> = image_buffer
                .pixels()
                .map(|p| match p.channels() {
                    [r, g, b, _] => (*r, *g, *b),
                    _ => (0, 0, 0),
                })
                .collect();

            output.frames.push(pixels_as_rgba_vec);
        }
        Ok(output)
    }
    pub fn new(playback: VisType, frames: Vec<Vec<(u8, u8, u8)>>) -> Self {
        Self { playback, frames }
    }
}

impl Default for RgbAnimation {
    fn default() -> Self {
        Self {
            playback: VisType::Full,
            frames: Vec::new(),
        }
    }
}

struct Ring {}

impl Ring {
    fn construct() -> RgbAnimation {
        let mut frames = Vec::new();
        for i in 0..=RING.len() {
            let mut frame: Vec<(u8, u8, u8)> = (0..GRID_LENGTH).map(|_f| (0, 0, 0)).collect();
            for z in 0..i {
                let (x, y) = RING[z];
                let index = coord_to_index((x, y), GRID_WIDTH);
                frame[index] = (100, 100, 100)
            }
            frames.push(frame);
        }
        RgbAnimation::new(VisType::Full, frames)
    }
}
