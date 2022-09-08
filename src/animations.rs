use crate::def_const::{GRID_LENGTH, GRID_WIDTH, RING};
use crate::def_grid::IndexedMatrix;
use crate::font;
use crate::utilities::coord_to_index;
use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, Pixel};
use lazy_static::lazy_static;
use rgb::RGB8;
use std::error::Error;
use std::result::Result;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum VisType {
    Full,
    Partial,
    Static,
    BeatIndependent,
}

lazy_static! {
    pub static ref ANIMATIONS: Vec<RgbAnimation> = {
        let mut vec = Vec::new();
        vec.push(
            RgbAnimation::new_from_gif(include_bytes!("../img/clock.gif"), VisType::Full).unwrap(),
        );
        vec.push(
            RgbAnimation::new_from_gif(
                include_bytes!("../img/counter_alpha.gif"),
                VisType::Partial,
            )
            .unwrap(),
        );
        vec.push(
            RgbAnimation::new_from_gif(include_bytes!("../img/rows_alpha.gif"), VisType::Full)
                .unwrap(),
        );
        vec.push(
            RgbAnimation::new_from_gif(include_bytes!("../img/circular.gif"), VisType::Full)
                .unwrap(),
        );
        vec.push(Ring::construct());
        vec.push(TextTest::construct());

        return vec;
    };
}

pub struct RgbAnimation {
    pub playback: VisType,
    pub frames: Vec<IndexedMatrix<RGB8>>,
}
impl RgbAnimation {
    pub fn dimensions(&self) -> (usize, usize) {
        (
            self.frames.get(0).unwrap().width,
            self.frames.get(0).unwrap().height,
        )
    }
    pub fn new_from_gif(bytes: &[u8], playback: VisType) -> Result<RgbAnimation, Box<dyn Error>> {
        let decoder = GifDecoder::new(bytes)?;

        let frames = decoder.into_frames().collect_frames()?;
        let (width, height) = frames.get(0).unwrap().buffer().dimensions();

        let mut output = Self::default();
        output.playback = playback;

        for frame in frames.iter() {
            let image_buffer = frame.buffer();

            let indexed_matrix = IndexedMatrix::new(
                image_buffer
                    .pixels()
                    .map(|p| match p.channels() {
                        [r, g, b, _] => RGB8 {
                            r: *r,
                            g: *g,
                            b: *b,
                        },
                        _ => RGB8 { r: 0, g: 0, b: 0 },
                    })
                    .collect(),
                width as usize,
                height as usize,
            );

            output.frames.push(indexed_matrix);
        }
        Ok(output)
    }
    pub fn new(playback: VisType, frames: Vec<IndexedMatrix<RGB8>>) -> Self {
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
            let mut frame: Vec<RGB8> = (0..GRID_LENGTH)
                .map(|_f| RGB8 { r: 0, g: 0, b: 0 })
                .collect();
            for z in 0..i {
                let (x, y) = RING[z];
                let index = coord_to_index((x, y), GRID_WIDTH);
                frame[index] = RGB8 { r: 200, g: 0, b: 0 }
            }
            frames.push(IndexedMatrix::new(frame, 8, 8));
        }
        RgbAnimation::new(VisType::BeatIndependent, frames)
    }
}

struct TextTest {}
impl TextTest {
    fn construct() -> RgbAnimation {
        let test_text = "     hello world ";
        let text1 = font::get_string_sequence(test_text.to_string());
        let sequence = font::sequence_to_matrix(text1);
        font::animation_from_sequence(sequence)
    }
}
