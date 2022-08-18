use gif2json::RgbaImageData;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum Visualization {
    Clock,
    Counter,
    Rows,
    Circular,
}

impl Default for Visualization {
    fn default() -> Self {
        Self::Rows
    }
}

lazy_static! {
    pub static ref GIFS: HashMap<Visualization, RgbaImageData> = {
        let mut map = HashMap::new();

        map.insert(
            Visualization::Clock,
            RgbaImageData::new_from_bytes(include_bytes!("../img/clock.gif")).unwrap(),
        );
        map.insert(
            Visualization::Counter,
            RgbaImageData::new_from_bytes(include_bytes!("../img/counter_alpha.gif")).unwrap(),
        );
        map.insert(
            Visualization::Rows,
            RgbaImageData::new_from_bytes(include_bytes!("../img/rows_alpha.gif")).unwrap(),
        );
        map.insert(
            Visualization::Circular,
            RgbaImageData::new_from_bytes(include_bytes!("../img/circular.gif")).unwrap(),
        );

        return map;
    };
}

// pub fn init_gifs() -> HashMap<Visualization, RgbaImageData> {
//     let mut map = HashMap::new();

//     map.insert(
//         Visualization::Clock,
//         RgbaImageData::new_from_bytes(include_bytes!("../img/clock.gif")).unwrap(),
//     );
//     map.insert(
//         Visualization::Counter,
//         RgbaImageData::new_from_bytes(include_bytes!("../img/counter_alpha.gif")).unwrap(),
//     );
//     map.insert(
//         Visualization::Rows,
//         RgbaImageData::new_from_bytes(include_bytes!("../img/rows_alpha.gif")).unwrap(),
//     );
//     map.insert(
//         Visualization::Circular,
//         RgbaImageData::new_from_bytes(include_bytes!("../img/circular.gif")).unwrap(),
//     );

//     return map;
// }
