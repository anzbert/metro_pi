use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, Pixel};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::error::Error;
use std::result::Result;

struct AllVis {}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum Visualization {
    Rows(VisType),
    Clock(VisType),
    Counter(VisType),
    Circular(VisType),
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum VisType {
    Gif,
    // Algorithm,
}

impl Default for Visualization {
    fn default() -> Self {
        Self::Circular(VisType::Gif)
    }
}

lazy_static! {
    pub static ref GIFS: HashMap<Visualization, RgbaImageData> = {
        let mut map = HashMap::new();

        map.insert(
            Visualization::Clock(VisType::Gif),
            RgbaImageData::new_from_bytes(include_bytes!("../img/clock.gif")).unwrap(),
        );
        map.insert(
            Visualization::Counter(VisType::Gif),
            RgbaImageData::new_from_bytes(include_bytes!("../img/counter_alpha.gif")).unwrap(),
        );
        map.insert(
            Visualization::Rows(VisType::Gif),
            RgbaImageData::new_from_bytes(include_bytes!("../img/rows_alpha.gif")).unwrap(),
        );
        map.insert(
            Visualization::Circular(VisType::Gif),
            RgbaImageData::new_from_bytes(include_bytes!("../img/circular.gif")).unwrap(),
        );

        return map;
    };
}

pub struct RgbaImageData {
    pub dimensions: (u32, u32),
    pub frames: Vec<DecodedFrame>,
}
impl RgbaImageData {
    pub fn new_from_bytes(bytes: &[u8]) -> Result<RgbaImageData, Box<dyn Error>> {
        let decoder = GifDecoder::new(bytes)?;

        let frames = decoder.into_frames().collect_frames()?;

        let dimensions = frames.get(0).unwrap().buffer().dimensions();
        let mut output = Self::new(dimensions);

        for frame in frames.iter() {
            let image_buffer = frame.buffer();

            let pixels_as_rgba_vec: Vec<(u8, u8, u8, u8)> = image_buffer
                .pixels()
                .map(|p| match p.channels() {
                    [r, g, b, a] => (*r, *g, *b, *a),
                    _ => (0, 0, 0, 0),
                })
                .collect();

            let decoded_frame =
                DecodedFrame::new(frame.delay().numer_denom_ms(), pixels_as_rgba_vec);

            output.add(decoded_frame);
        }
        Ok(output)
    }

    fn new(dimensions: (u32, u32)) -> Self {
        Self {
            dimensions,
            frames: Vec::new(),
        }
    }

    pub fn _get_frame_vec_ref(&self, frame: usize) -> Option<&Vec<(u8, u8, u8, u8)>> {
        match self.frames.get(frame) {
            Some(f) => Some(&f.pixels),
            None => None,
        }
    }
    pub fn _get_frame_delay(&self, frame: usize) -> Option<(u32, u32)> {
        match self.frames.get(frame) {
            Some(f) => Some(f.delay_ratio),
            None => None,
        }
    }
    pub fn _get_dimensions(&self) -> (u32, u32) {
        self.dimensions
    }
    fn add(&mut self, next_frame: DecodedFrame) {
        self.frames.push(next_frame);
    }
}

pub struct DecodedFrame {
    pub delay_ratio: (u32, u32),
    pub pixels: Vec<(u8, u8, u8, u8)>,
}

impl DecodedFrame {
    fn new(delay_ratio: (u32, u32), pixels: Vec<(u8, u8, u8, u8)>) -> Self {
        Self {
            delay_ratio,
            pixels,
        }
    }
}
