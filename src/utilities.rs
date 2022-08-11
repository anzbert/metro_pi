#![allow(dead_code)]
pub fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

/// Grid by index to Tuple(x,y)
fn array_to_coord(input: usize, grid_size_x: usize, grid_size_y: usize) -> (usize, usize) {
    let x = input % grid_size_x;
    let y = input / grid_size_y;
    (x, y)
}
