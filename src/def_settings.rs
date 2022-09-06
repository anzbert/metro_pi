use crate::animations::RgbAnimation;

pub struct Settings {
    pub visual: &'static RgbAnimation,
    pub tempo: f64,
    pub quantum: f64,
    pub link_enabled: bool,
    pub brightness: u8,
}

// impl Settings {}
