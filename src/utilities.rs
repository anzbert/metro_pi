#![allow(dead_code)]
pub fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

/// Index to Tuple(x, y)
pub fn index_to_coord(input: usize, grid_size_x: usize, grid_size_y: usize) -> (usize, usize) {
    let x = input % grid_size_x;
    let y = input / grid_size_y;
    (x, y)
}

/// Tuple(x, y) to Index
pub fn coord_to_index(input: (usize, usize), grid_size_x: usize) -> usize {
    let index = input.1 * grid_size_x + input.0;
    index
}
