#[allow(dead_code)]
pub const GRID_HEIGHT: usize = 8;
#[allow(dead_code)]
pub const GRID_WIDTH: usize = 8;

pub const GRID_LENGTH: usize = GRID_HEIGHT * GRID_WIDTH;

pub const SOUND_PATHS: [&str; 3] = [
    "snd/met_mech.wav",
    "snd/met_elec.wav",
    "snd/examples_music.wav",
];

pub const FRAME_TIME: usize = 60;

#[allow(unused_allocation)]
pub const RING: [(usize, usize); 28] = [
    (0, 0),
    (1, 0),
    (2, 0),
    (3, 0),
    (4, 0),
    (5, 0),
    (6, 0),
    (7, 0),
    (7, 1),
    (7, 2),
    (7, 3),
    (7, 4),
    (7, 5),
    (7, 6),
    (7, 7),
    (6, 7),
    (5, 7),
    (4, 7),
    (3, 7),
    (2, 7),
    (1, 7),
    (0, 7),
    (0, 6),
    (0, 5),
    (0, 4),
    (0, 3),
    (0, 2),
    (0, 1),
];
