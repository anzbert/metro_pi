struct IndexedMatrix<T: Copy> {
    pixels: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Copy> IndexedMatrix<T> {
    fn new(pixels: Vec<T>, width: usize, height: usize) -> Self {
        assert_eq!(pixels.len(), width * height);

        Self {
            pixels,
            width,
            height,
        }
    }
    fn to_xy_matrix(&self) -> XYMatrix<T> {
        let mut y_columns: Vec<Vec<T>> = Vec::with_capacity(self.width);
        for y_column in 0..self.width {
            let mut column: Vec<T> = Vec::with_capacity(self.height);
            for y_position in 0..self.height {
                column.push(*self.pixels.get(y_position * self.width + y_column).unwrap());
            }
            y_columns.push(column);
        }
        XYMatrix::new(y_columns)
    }
}

struct XYMatrix<T: Copy> {
    pixels: Vec<Vec<T>>,
}

impl<T: Copy> XYMatrix<T> {
    fn new(pixels: Vec<Vec<T>>) -> Self {
        let height = pixels.get(0).unwrap().len();

        for y_column in &pixels {
            if y_column.len() != height {
                panic!();
            }
        }

        Self { pixels }
    }

    fn to_indexed_matrix(&self) -> IndexedMatrix<T> {
        let mut indexed_matrix: Vec<T> =
            Vec::with_capacity(self.pixels.len() * self.pixels[0].len());

        for y_column in &self.pixels {
            for y_position in y_column {
                indexed_matrix.push(*y_position);
            }
        }

        IndexedMatrix::new(
            indexed_matrix,
            self.pixels.len(),
            self.pixels.get(0).unwrap().len(),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::IndexedMatrix;

    #[test]
    fn matrix_conversion() {
        let input: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let indexed1: IndexedMatrix<i32> = IndexedMatrix::new(input.clone(), 4, 2);

        let xy = indexed1.to_xy_matrix();
        let indexed2 = xy.to_indexed_matrix();
        assert_ne!(vec![0, 1], vec![1, 0]);
        assert_eq!(indexed1.pixels, indexed2.pixels);
    }
}
