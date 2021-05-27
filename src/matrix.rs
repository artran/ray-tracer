use std::ops::Mul;
use crate::tuple::Tuple;

#[derive(Debug, PartialEq)]
pub struct Matrix<const M: usize> {
    // Square Matrices only
    contents: [[f32; M]; M],
}

impl<const M: usize> Matrix<M> {
    fn new(contents: [[f32; M]; M]) -> Self {
        Self {
            contents,
        }
    }

    pub fn identity() -> Self {
        let mut rows = [[0.0_f32; M]; M];

        for idx in 0..M {
            rows[idx][idx] = 1.0_f32;
        }

        Matrix::new(rows)
    }

    pub fn rows(rows: [[f32; M]; M]) -> Self {
        Matrix::new(rows)
    }

    pub fn index(&self, row: usize, col: usize) -> f32 {
        self.contents[row][col]
    }

    pub fn transposed(&self) -> Self {
        let mut rows = [[0.0_f32; M]; M];

        for row in 0..M {
            for col in 0..M {
                rows[row][col] = self.index(col, row);
            }
        }

        Matrix::new(rows)
    }
}

impl Matrix<2> {
    pub fn determinant(&self) -> f32 {
        self.index(0, 0) * self.index(1, 1)
            - self.index(0, 1) * self.index(1, 0)
    }
}

impl Matrix<3> {
    // fixme: code duplication
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix<2> {
        let mut rows = [[0.0_f32; 2]; 2];

        let mut result_row = 0;
        let mut result_col = 0;
        for r in 0..3 {
            if r == row {
                continue;
            }
            for c in 0..3 {
                if c == col {
                    continue;
                }
                rows[result_row][result_col] = self.index(r, c);
                result_col += 1;
            }
            result_col = 0;
            result_row += 1;
        }

        Matrix::new(rows)
    }
}

impl Matrix<4> {
    // fixme: code duplication
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix<3> {
        let mut rows = [[0.0_f32; 3]; 3];

        let mut result_row = 0;
        let mut result_col = 0;
        for r in 0..4 {
            if r == row {
                continue;
            }
            for c in 0..4 {
                if c == col {
                    continue;
                }
                rows[result_row][result_col] = self.index(r, c);
                result_col += 1;
            }
            result_col = 0;
            result_row += 1;
        }

        Matrix::new(rows)
    }
}

impl<const M: usize> Mul<&Matrix<M>> for &Matrix<M> {
    type Output = Matrix<M>;

    fn mul(self, rhs: &Matrix<M>) -> Matrix<M> {
        let mut result: Matrix<M> = Matrix::rows([[0.0_f32; M]; M]);
        for row in 0..M {
            for col in 0..M {
                let mut sum = 0.0;
                for idx in 0..M {
                    sum += self.index(row, idx) * rhs.index(idx, col);
                }
                result.contents[row][col] = sum;
            }
        }
        result
    }
}

impl Mul<&Tuple> for &Matrix<4> {
    type Output = Tuple;

    fn mul(self, rhs: &Tuple) -> Tuple {
        let mut result = Tuple::point(0.0, 0.0, 0.0);
        for row in 0..4 {
            let mut sum = 0.0;
            for idx in 0..4 {
                sum += self.index(row, idx) * rhs[idx];
            }
            result[row] = sum;
        }
        result
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
        let _: Matrix<4> = Matrix::rows([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);
    }

    #[test]
    fn matrices_indexed_by_row_col() {
        let m: Matrix<4> = Matrix::rows([
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

    #[test]
    fn matrices_with_the_same_values_are_equal() {
        let m1: Matrix<4> = Matrix::rows([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 8.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2: Matrix<4> = Matrix::rows([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 8.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        assert_that!(m1).is_equal_to(m2);
    }

    #[test]
    fn matrices_with_the_different_values_are_not_equal() {
        let m1: Matrix<4> = Matrix::rows([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 8.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2: Matrix<4> = Matrix::rows([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);

        assert_that!(m1).is_not_equal_to(m2);
    }

    #[test]
    fn multiplying_matrices() {
        let m1: Matrix<4> = Matrix::rows([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2: Matrix<4> = Matrix::rows([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let expected: Matrix<4> = Matrix::rows([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        let result: Matrix<4> = &m1 * &m2;
        assert_that!(result).is_equal_to(expected);
    }

    #[test]
    fn multiplying_matrices_by_tuples() {
        let a: Matrix<4> = Matrix::rows([
            [1.0, 2.0, 3.0, 4.0, ],
            [2.0, 4.0, 4.0, 2.0, ],
            [8.0, 6.0, 4.0, 1.0, ],
            [0.0, 0.0, 0.0, 1.0, ],
        ]);
        let b = Tuple::point(1.0, 2.0, 3.0);
        let expected = Tuple::point(18.0, 24.0, 33.0);

        let result = &a * &b;
        assert_that!(result).is_equal_to(expected);
    }

    #[test]
    fn multiplying_matrix_by_the_identity_matrix() {
        let a = Matrix::rows([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);
        let ident = Matrix::identity();

        let result = &a * &ident;
        assert_that!(result).is_equal_to(a);
    }

    #[test]
    fn multiplying_identity_matrix_by_a_tuple() {
        let tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);

        let result = &Matrix::identity() * &tuple;

        assert_that!(result).is_equal_to(tuple);
    }

    #[test]
    fn transposing_a_matrix() {
        let a = Matrix::rows([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        let transposed = Matrix::rows([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);

        assert_that(&a.transposed()).is_equal_to(transposed);
    }

    #[test]
    fn transposing_the_identity_matrix_return_identity() {
        let a: Matrix<4> = Matrix::identity();

        assert_that(&a.transposed()).is_equal_to(a);
    }

    #[test]
    fn calculating_the_determinant_of_2x2_matrix() {
        let a = Matrix::rows([
            [1.0, 5.0],
            [-3.0, 2.0],
        ]);

        assert_that!(a.determinant()).is_equal_to(17.0);
    }

    #[test]
    fn submatrix_of_3x3_matrix_is_2x2_matrix() {
        let a = Matrix::rows([
            [1.0, 5.0, 0.0],
            [-3.0, 2.0, 7.0],
            [0.0, 6.0, -3.0],
        ]);
        let expected = Matrix::rows([
            [-3.0, 2.0],
            [0.0, 6.0],
        ]);

        assert_that(&a.submatrix(0, 2)).is_equal_to(expected);
    }

    #[test]
    fn submatrix_of_4x4_matrix_is_3x3_matrix() {
        let a = Matrix::rows([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        let expected = Matrix::rows([
            [-6.0, 1.0, 6.0],
            [-8.0, 8.0, 6.0],
            [-7.0, -1.0, 1.0],
        ]);

        assert_that(&a.submatrix(2, 1)).is_equal_to(expected);
    }
}
