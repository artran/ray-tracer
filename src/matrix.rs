use crate::tuple::Tuple;
use std::ops::Mul;

#[derive(Debug, PartialEq)]
pub struct Matrix<const M: usize> {
    // Square Matrices only
    contents: [[f32; M]; M],
}

impl<const M: usize> Matrix<M> {
    pub fn identity() -> Self {
        let mut rows = [[0.0_f32; M]; M];

        for idx in 0..M {
            rows[idx][idx] = 1.0_f32;
        }

        Matrix::rows(rows)
    }

    pub fn rows(rows: [[f32; M]; M]) -> Self {
        Matrix {
            contents: rows
        }
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

        Matrix::rows(rows)
    }
}

impl Matrix<2> {
    pub fn determinant(&self) -> f32 {
        self.index(0, 0) * self.index(1, 1) - self.index(0, 1) * self.index(1, 0)
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

        Matrix::rows(rows)
    }

    pub fn minor(&self, row: usize, col: usize) -> f32 {
        let s = self.submatrix(row, col);
        s.determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        if (row + col) % 2 == 0 {
            self.minor(row, col)
        } else {
            - self.minor(row, col)
        }
    }

    pub fn determinant(&self) -> f32 {
        let mut result = 0.0;
        for idx in 0..3 {
            result += self.index(0, idx) * self.cofactor(0, idx);
        }
        result
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

        Matrix::rows(rows)
    }

    pub fn minor(&self, row: usize, col: usize) -> f32 {
        let s = self.submatrix(row, col);
        s.determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        if (row + col) % 2 == 0 {
            self.minor(row, col)
        } else {
            - self.minor(row, col)
        }
    }

    pub fn determinant(&self) -> f32 {
        let mut result = 0.0;
        for idx in 0..4 {
            result += self.index(0, idx) * self.cofactor(0, idx);
        }
        result
    }

    pub fn invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Matrix<4> {
        let mut rows = [[0.0_f32; 4]; 4];
        let determinant = self.determinant();
        if determinant == 0.0 {
            Matrix::rows(rows)
        } else {
            for row in 0..4 {
                for col in 0..4 {
                    let c = self.cofactor(row, col);
                    rows[col][row] = c / determinant;
                }
            }

            Matrix::rows(rows)
        }
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
    use super::*;
    use spectral::assert_that;
    use spectral::boolean::BooleanAssertions;
    use spectral::numeric::FloatAssertions;

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
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
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
            [-3.0, 2.0]
        ]);

        assert_that!(a.determinant()).is_equal_to(17.0);
    }

    #[test]
    fn submatrix_of_3x3_matrix_is_2x2_matrix() {
        let a = Matrix::rows([
            [1.0, 5.0, 0.0],
            [-3.0, 2.0, 7.0],
            [0.0, 6.0, -3.0]
        ]);
        let expected = Matrix::rows([
            [-3.0, 2.0],
            [0.0, 6.0]
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
            [-7.0, -1.0, 1.0]
        ]);

        assert_that(&a.submatrix(2, 1)).is_equal_to(expected);
    }

    #[test]
    fn calculating_a_minor_of_a_3x3_matrix() {
        let a = Matrix::rows([
            [3.0, 5.0, 0.0],
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0]
        ]);

        let b = a.submatrix(1, 0);

        assert_that!(b.determinant()).is_equal_to(25.0);
        assert_that!(a.minor(1, 0)).is_equal_to(25.0);
    }

    #[test]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        let a = Matrix::rows([
            [3.0, 5.0, 0.0],
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0]
        ]);

        assert_that!(a.minor(0, 0)).is_equal_to(-12.0);
        assert_that!(a.cofactor(0, 0)).is_equal_to(-12.0);
        assert_that!(a.minor(1, 0)).is_equal_to(25.0);
        assert_that!(a.cofactor(1, 0)).is_equal_to(-25.0);
    }

    #[test]
    fn calculating_the_determinant_of_a_3x3_matrix() {
        let a = Matrix::rows([
            [1.0, 2.0, 6.0],
            [-5.0, 8.0, -4.0],
            [2.0, 6.0, 4.0],
        ]);

        assert_that!(a.cofactor(0, 0)).is_equal_to(56.0);
        assert_that!(a.cofactor(0, 1)).is_equal_to(12.0);
        assert_that!(a.cofactor(0, 2)).is_equal_to(-46.0);
        assert_that!(a.determinant()).is_equal_to(-196.0);
    }

    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        let a = Matrix::rows([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_that!(a.cofactor(0, 0)).is_equal_to(690.0);
        assert_that!(a.cofactor(0, 1)).is_equal_to(447.0);
        assert_that!(a.cofactor(0, 2)).is_equal_to(210.0);
        assert_that!(a.cofactor(0, 3)).is_equal_to(51.0);
        assert_that!(a.determinant()).is_equal_to(-4071.0);
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let a = Matrix::rows([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);

        assert_that!(a.determinant()).is_equal_to(-2120.0);
        assert_that!(a.invertible()).is_true();
    }

    #[test]
    fn testing_a_noninvertible_matrix_for_invertibility() {
        let a = Matrix::rows([
            [-4.0 , 2.0, -2.0, -3.0],
            [9.0 , 6.0, 2.0, 6.0],
            [0.0 , -5.0, 1.0, -5.0],
            [0.0 , 0.0, 0.0, 0.0],
        ]);

        assert_that!(a.determinant()).is_equal_to(0.0);
        assert_that!(a.invertible()).is_false();
    }

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        let a = Matrix::rows([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        let inverse = a.inverse();
        let expected = Matrix::rows([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);

        assert_that!(a.determinant()).is_equal_to(532.0);
        assert_that!(a.cofactor(2, 3)).is_equal_to(-160.0);
        assert_that!(a.cofactor(3, 2)).is_equal_to(105.0);
        for row in 0..4 {
            for col in 0..4 {
                assert_that!(inverse.index(row, col)).is_close_to(expected.index(row, col), 0.00001);
            }
        }
    }

    #[test]
    fn calculating_the_inverse_of_another_matrix() {
        let a = Matrix::rows([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        let inverse = a.inverse();
        let expected = Matrix::rows([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);

        for row in 0..4 {
            for col in 0..4 {
                assert_that!(inverse.index(row, col)).is_close_to(expected.index(row, col), 0.00001);
            }
        }
    }

    #[test]
    fn calculating_the_inverse_of_a_third_matrix() {
        let a = Matrix::rows([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        let inverse = a.inverse();
        let expected = Matrix::rows([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);

        for row in 0..4 {
            for col in 0..4 {
                assert_that!(inverse.index(row, col)).is_close_to(expected.index(row, col), 0.00001);
            }
        }
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let a = Matrix::rows([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let b = Matrix::rows([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
      ]);

      let c = &a * &b;

      // assert_that!((c as Matrix<4> * b.inverse())).is_equal_to(a);
    }
}
