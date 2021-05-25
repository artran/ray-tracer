pub struct Matrix<const M: usize, const N: usize> {
    contents: [[f32; M]; N],
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    fn new(contents: [[f32; M]; N]) -> Self {
        Self {
            contents,
        }
    }

    pub fn rows(rows: [[f32; M]; N]) -> Self {
        Matrix::new(rows)
    }

    pub fn index(&self, row: usize, col: usize) -> f32 {
        self.contents[row][col]
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use spectral::assert_that;
    use super::*;

    #[test]
    fn matrices_constructed_from_rows() {
        let _: Matrix<4, 4> = Matrix::rows([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);
    }

    #[test]
    fn matrices_indexed_by_row_col() {
        let m: Matrix<4, 4> = Matrix::rows([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_that!(m.index(0, 0)).is_equal_to(1.0);
        assert_that!(m.index(0, 3)).is_equal_to(4.0);
        assert_that!(m.index(1, 0)).is_equal_to(5.5);
        assert_that!(m.index(1, 2)).is_equal_to(7.5);
        assert_that!(m.index(2, 2)).is_equal_to(11.0);
        assert_that!(m.index(3, 0)).is_equal_to(13.5);
        assert_that!(m.index(3, 2)).is_equal_to(15.5);
    }
}
