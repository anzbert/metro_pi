use crate::{
    animations::{RgbAnimation, VisType},
    def_grid::{IndexedMatrix, XYMatrix},
};
use lazy_static::lazy_static;
use rgb::RGB8;

lazy_static! {
    static ref LETTERS_5X5: RgbAnimation = {
        RgbAnimation::new_from_gif(include_bytes!("../img/alphabet_5x5.gif"), VisType::Static)
            .unwrap()
    };
}
lazy_static! {
    static ref NUMBERS_5X5: RgbAnimation = {
        RgbAnimation::new_from_gif(include_bytes!("../img/numbers_5x5.gif"), VisType::Static)
            .unwrap()
    };
}

pub fn get_string_sequence(source: String) -> Vec<IndexedMatrix<RGB8>> {
    let mut sequence: Vec<IndexedMatrix<RGB8>> = Vec::new();
    for character in source.trim().chars() {
        if character.is_ascii_alphabetic() {
            sequence.push(
                LETTERS_5X5
                    .frames
                    .get(character.to_ascii_uppercase() as usize - ('A' as usize))
                    .unwrap()
                    .clone(),
            );
        } else if character.is_ascii_digit() {
            sequence.push(
                LETTERS_5X5
                    .frames
                    .get(character as usize - ('0' as usize))
                    .unwrap()
                    .clone(),
            );
        }
        match character {
            ' ' => sequence.push(IndexedMatrix::new(
                Vec::from([RGB8 { r: 0, g: 0, b: 0 }; 25]),
                5,
                5,
            )),
            _ => {}
        }
    }
    sequence
}

pub fn sequence_to_matrix(seq: Vec<IndexedMatrix<RGB8>>) -> IndexedMatrix<RGB8> {
    let mut output: Vec<RGB8> = Vec::new();

    // add 2 empty rows on top:
    for _ in 0..(seq.len() * 2) {
        for _ in 0..(6) {
            output.push(RGB8 { r: 0, g: 0, b: 0 });
        }
    }

    for row in 0..5 {
        for char in seq.iter() {
            // +1 empty for spacing
            output.push(RGB8 { r: 0, g: 0, b: 0 });

            for pixel in (row * 5)..(row * 5 + 5) {
                output.push(char.pixels[pixel]);
            }
        }
    }

    // add 1 empty rows at bottom:
    for _ in 0..seq.len() {
        for _ in 0..(6) {
            output.push(RGB8 { r: 0, g: 0, b: 0 });
        }
    }

    IndexedMatrix::new(output, seq.len() * 6, 8)
}

pub fn animation_from_sequence(sequence: IndexedMatrix<RGB8>) -> RgbAnimation {
    assert_eq!(sequence.pixels.len() % 8, 0);

    let screens = 1 + ((sequence.pixels.len() - 64) / 8);

    let bla = sequence.to_xy_matrix();

    let mut animation = RgbAnimation::default();

    for screen_index in 0..screens {
        let screen = Vec::from(&bla.pixels[screen_index..screen_index + 8]);
        animation
            .frames
            .push(XYMatrix::new(screen).to_indexed_matrix())
    }

    animation
}

#[cfg(test)]
mod tests {
    use super::{get_string_sequence, sequence_to_matrix};

    #[test]
    fn sequence_size() {
        let sequence = get_string_sequence("hello123".to_string());
        assert_eq!(sequence.len(), 8);
        assert_eq!(sequence[0].pixels.len(), 25);
    }

    #[test]
    fn sequence_conversion() {
        let sequence = get_string_sequence("hello123".to_string());
        let final_vec = sequence_to_matrix(sequence);
        assert_eq!(final_vec.pixels.len(), 8 * (8 * 5) + 8 * 8);
    }
}
