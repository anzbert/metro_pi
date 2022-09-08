use crate::{
    animations::{RgbAnimation, VisType},
    def_const::GRID_HEIGHT,
};
use lazy_static::lazy_static;

lazy_static! {
    static ref LETTERS_5X5: RgbAnimation = {
        RgbAnimation::new_from_bytes(include_bytes!("../img/alphabet_5x5.gif"), VisType::Static)
            .unwrap()
    };
}
lazy_static! {
    static ref NUMBERS_5X5: RgbAnimation = {
        RgbAnimation::new_from_bytes(include_bytes!("../img/numbers_5x5.gif"), VisType::Static)
            .unwrap()
    };
}
// lazy_static! {
//     static ref LETTERS_3X5: RgbAnimation = {
//         RgbAnimation::new_from_bytes(include_bytes!("../img/numbers_3x5.gif"), VisType::Static)
//             .unwrap()
//     };
// }

pub fn get_string_sequence(source: String) -> Vec<&'static Vec<(u8, u8, u8)>> {
    let mut sequence: Vec<&Vec<(u8, u8, u8)>> = Vec::new();
    for character in source.trim().chars() {
        if character.is_ascii_alphabetic() {
            sequence.push(
                LETTERS_5X5
                    .frames
                    .get(character.to_ascii_uppercase() as usize - ('A' as usize))
                    .unwrap(),
            );
        } else if character.is_ascii_digit() {
            sequence.push(
                LETTERS_5X5
                    .frames
                    .get(character as usize - ('0' as usize))
                    .unwrap(),
            );
        }
    }
    sequence
}

fn sequence_to_matrix(seq: Vec<&'static Vec<(u8, u8, u8)>>) -> Vec<(u8, u8, u8)> {
    let mut output = Vec::new();

    // add 2 empty rows on top:
    for _ in 0..(seq.len() * 2) {
        for _ in 0..(5 + 1) {
            output.push((0, 0, 0));
        }
    }

    for row in 0..5 {
        for char in seq.iter() {
            for pixel in (row * 5)..(row * 5 + 5) {
                output.push(char[pixel]);
            }
            // +1
            output.push((0, 0, 0));
        }
    }

    // add 1 empty rows at bottom:
    for _ in 0..seq.len() {
        for _ in 0..(5 + 1) {
            output.push((0, 0, 0));
        }
    }

    output
}

pub fn animation_from_sequence(sequence: Vec<(u8, u8, u8)>) {
    assert_eq!(sequence.len() % 8, 0);
    let screens = (sequence.len() - 64) / 8;
    for screen_index in 0..screens {
        for row in 0..8 {
            todo!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{get_string_sequence, sequence_to_matrix};

    #[test]
    fn sequence_size() {
        let sequence = get_string_sequence("hello123".to_string());
        assert_eq!(sequence.len(), 8);
        assert_eq!(sequence[0].len(), 25);
    }

    #[test]
    fn sequence_conversion() {
        let sequence = get_string_sequence("hello123".to_string());
        let final_vec = sequence_to_matrix(sequence);
        assert_eq!(final_vec.len(), 8 * (8 * 5) + 8 * 8);
    }
}
