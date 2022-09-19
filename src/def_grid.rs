// / Matrix as a 1D Indexed Vector in Horizontal Rows
#[derive(Clone)]
pub struct IndexedMatrix<T: Copy> {
    pub pixels: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Copy> IndexedMatrix<T> {
    pub fn new(pixels: Vec<T>, width: usize, height: usize) -> Self {
        assert_eq!(pixels.len(), width * height);

        Self {
            pixels,
            width,
            height,
        }
    }
    pub fn to_xy_matrix(&self) -> XYMatrix<T> {
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
    pub fn add_to_right(&mut self, input: Self) {
        assert_eq!(
            input.height, self.height,
            "can only add a matrix of same height"
        );

        let mut output: Vec<T> = Vec::with_capacity(self.pixels.len() + input.pixels.len());

        for row in 0..self.height {
            for p in 0..self.width {
                output.push(self.pixels[p + row * self.width]);
            }
            for p in 0..input.width {
                output.push(input.pixels[p + row * input.width]);
            }
        }
        self.pixels = output;
    }
}

/// 2D Matrix in columns with index x. eg: example_matrix[x][y]
pub struct XYMatrix<T: Copy> {
    pub pixels: Vec<Vec<T>>,
}

impl<T: Copy> XYMatrix<T> {
    pub fn new(pixels: Vec<Vec<T>>) -> Self {
        let height = pixels.get(0).unwrap().len();

        for y_column in &pixels {
            if y_column.len() != height {
                panic!();
            }
        }

        Self { pixels }
    }

    pub fn to_indexed_matrix(&self) -> IndexedMatrix<T> {
        let mut indexed_matrix: Vec<T> =
            Vec::with_capacity(self.pixels.len() * self.pixels[0].len());

        let height = self.pixels[0].len();

        for y_position in 0..height {
            for y_column in &self.pixels {
                indexed_matrix.push(y_column[y_position]);
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
    use super::IndexedMatrix;

    const TEST_VEC: [i32; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

    #[test]
    fn matrix_conversion() {
        // assert_ne!(vec![0, 1], vec![1, 0]);
        let input: Vec<i32> = Vec::from(TEST_VEC);
        let indexed1: IndexedMatrix<i32> = IndexedMatrix::new(input.clone(), 4, 2);
        assert_eq!(indexed1.pixels, input);

        let xy = indexed1.to_xy_matrix();
        assert_eq!(xy.pixels, [[0, 4], [1, 5], [2, 6], [3, 7]]);

        let indexed2 = xy.to_indexed_matrix();
        assert_eq!(indexed1.pixels, indexed2.pixels);
    }

    #[test]
    fn test_matrix() {
        let input: Vec<i32> = Vec::from(TEST_VEC);
        let indexed1: IndexedMatrix<i32> = IndexedMatrix::new(input.clone(), 4, 2);
        assert_eq!(indexed1.pixels, input);

        let xy = indexed1.to_xy_matrix();
        assert_eq!(xy.pixels, [[0, 4], [1, 5], [2, 6], [3, 7]]);

        assert_eq!(xy.pixels[0][1], 4);
        assert_eq!(xy.pixels[2][0], 2);
    }
}
