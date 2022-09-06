use crate::gifs::RgbAnimation;

pub struct Settings {
    pub visual: &'static RgbAnimation,
    pub tempo: f64,
    pub quantum: f64,
    pub link_enabled: bool,
    pub volume: u8,
    pub brightness: u8,
}

// impl Settings {}
