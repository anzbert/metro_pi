use crate::gifs::Visualization;

#[derive(PartialEq)]
pub struct Settings {
    pub visual: Visualization,
    pub tempo: f64,
    pub quantum: f64,
    pub link_enabled: bool,
    pub sound_enabled: bool,
    pub volume: u8,
    pub brightness: u8,
}

// impl Settings {}
