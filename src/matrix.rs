use std::error::Error;

use crate::vector4::Vector4;

const EPSILON: f32 = 0.001;

#[derive(Debug)]
pub struct NonInvertibleError;

impl Error for NonInvertibleError {}

impl std::fmt::Display for NonInvertibleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "matrix is non-invertible, but `inv()` was called")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const L: usize> {
    _inner: [[f32; L]; L],
}

impl std::ops::Mul for Matrix<4> {
    type Output = Matrix<4>;

    /// Performs the `*` operation.
    ///
    /// # Panics
    ///
    /// Panics if either of the matrices are not 4x4.
    fn mul(self, other: Self) -> Self::Output {
        let mut new_inner: Matrix<4> = Matrix::new();
        for row in 0..4 {
            for col in 0..4 {
                new_inner[row][col] = self[[row, 0]] * other[[0, col]]
                    + self[[row, 1]] * other[[1, col]]
                    + self[[row, 2]] * other[[2, col]]
                    + self[[row, 3]] * other[[3, col]];
            }
        }
        Matrix::from(new_inner)
    }
}

impl std::ops::Mul<Vector4> for Matrix<4> {
    type Output = Vector4;

    fn mul(self, other: Vector4) -> Self::Output {
        Vector4::new(
            self[[0, 0]] * other.x
                + self[[0, 1]] * other.y
                + self[[0, 2]] * other.z
                + self[[0, 3]] * other.w,
            self[[1, 0]] * other.x
                + self[[1, 1]] * other.y
                + self[[1, 2]] * other.z
                + self[[1, 3]] * other.w,
            self[[2, 0]] * other.x
                + self[[2, 1]] * other.y
                + self[[2, 2]] * other.z
                + self[[2, 3]] * other.w,
            self[[3, 0]] * other.x
                + self[[3, 1]] * other.y
                + self[[3, 2]] * other.z
                + self[[3, 3]] * other.w,
        )
    }
}

impl std::ops::Mul<Matrix<4>> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: Matrix<4>) -> Self::Output {
        rhs * self
    }
}

impl<const L: usize> PartialEq for Matrix<L> {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..self.len() {
            for col in 0..self.len() {
                if (self[[row, col]] - other[[row, col]]).abs() > EPSILON {
                    return false;
                }
            }
        }
        true
    }
}

impl<const L: usize> std::ops::Index<[usize; 2]> for Matrix<L> {
    type Output = f32;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self._inner[index[0]][index[1]]
    }
}

impl<const L: usize> std::ops::Index<usize> for Matrix<L> {
    type Output = [f32; L];

    fn index(&self, index: usize) -> &Self::Output {
        &self._inner[index]
    }
}

impl<const L: usize> std::ops::IndexMut<[usize; 2]> for Matrix<L> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self._inner[index[0]][index[1]]
    }
}

impl<const L: usize> std::ops::IndexMut<usize> for Matrix<L> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self._inner[index]
    }
}

impl<const L: usize> Matrix<L> {
    /// Create a new Matrix with all blank slots.
    pub fn new() -> Self {
        Self {
            _inner: [[0.0; L]; L],
        }
    }

    /// Get the length of the Matrix (L).
    pub fn len(&self) -> usize {
        L
    }
}

impl<const L: usize> From<[[f32; L]; L]> for Matrix<L> {
    fn from(value: [[f32; L]; L]) -> Self {
        Self { _inner: value }
    }
}

impl Matrix<2> {
    /// Calculate the determinant of the matrix.
    pub fn determinant(&self) -> f32 {
        self[[0, 0]] * self[[1, 1]] - self[[0, 1]] * self[[1, 0]]
    }
}

impl Matrix<3> {
    /// Calculate the determinant of the matrix.
    pub fn determinant(&self) -> f32 {
        let mut det = 0.0;
        for col in 0..self.len() {
            det += self[[0, col]] * self.cofactor(0, col)
        }
        det
    }

    /// Calculate the submatrix of `self`.
    /// This is `self` with row `row` and column `col` removed.
    ///
    /// # Panics
    ///
    /// Panics if the provided indices are invalid.
    pub fn submatrix(&self, cut_row: usize, cut_col: usize) -> Matrix<2> {
        let mut new_matrix: Matrix<2> = Matrix::new();
        new_matrix = submatrix(self, new_matrix, cut_row, cut_col);
        new_matrix
    }

    /// Calculate the minor of `self`.
    /// This is the determinant of the submatrix at row `row` and column `col`.
    ///
    /// # Panics
    ///
    /// Panics if the provided indices are invalid.
    pub fn minor(&self, row: usize, col: usize) -> f32 {
        self.submatrix(row, col).determinant()
    }

    /// Calculate the cofactor of `self`.
    /// This is the minor at row `row` and column `col`, but with the sign flipped if `row + col` is odd.
    ///
    /// # Panics
    ///
    /// Panics if the provided indices are invalid.
    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }
}

impl Matrix<4> {
    /// Create the 4x4 identity matrix.
    pub fn identity() -> Self {
        Self::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Transpose the matrix (turn columns into rows and rows into columns).
    pub fn transpose(&self) -> Self {
        Self::from([
            [self[[0, 0]], self[[1, 0]], self[[2, 0]], self[[3, 0]]],
            [self[[0, 1]], self[[1, 1]], self[[2, 1]], self[[3, 1]]],
            [self[[0, 2]], self[[1, 2]], self[[2, 2]], self[[3, 2]]],
            [self[[0, 3]], self[[1, 3]], self[[2, 3]], self[[3, 3]]],
        ])
    }

    /// Calculate the submatrix of `self`.
    /// This is `self` with row `row` and column `col` removed.
    ///
    /// # Panics
    ///
    /// Panics if the provided indices are invalid.
    pub fn submatrix(&self, cut_row: usize, cut_col: usize) -> Matrix<3> {
        let mut new_matrix: Matrix<3> = Matrix::new();
        new_matrix = submatrix(self, new_matrix, cut_row, cut_col);
        new_matrix
    }

    /// Calculate the minor of `self`.
    /// This is the determinant of the submatrix at row `row` and column `col`.
    ///
    /// # Panics
    ///
    /// Panics if the provided indices are invalid.
    pub fn minor(&self, row: usize, col: usize) -> f32 {
        self.submatrix(row, col).determinant()
    }

    /// Calculate the cofactor of `self`.
    /// This is the minor at row `row` and column `col`, but with the sign flipped if `row + col` is odd.
    ///
    /// # Panics
    ///
    /// Panics if the provided indices are invalid.
    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    /// Calculate the determinant of the matrix.
    pub fn determinant(&self) -> f32 {
        let mut det = 0.0;
        for col in 0..self.len() {
            det += self[[0, col]] * self.cofactor(0, col)
        }
        det
    }

    /// `self.det() != 0`
    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    /// Invert the matrix.
    pub fn try_inverse(&self) -> Result<Self, NonInvertibleError> {
        if !self.is_invertible() {
            return Err(NonInvertibleError);
        }
        let det = self.determinant();
        let mut new_matrix: Matrix<4> = Matrix::new();
        for row in 0..4 {
            for col in 0..4 {
                let cofactor = self.cofactor(row, col);
                new_matrix[col][row] = cofactor / det;
            }
        }
        Ok(new_matrix)
    }
}

fn submatrix<const L: usize, const M: usize>(
    old_matrix: &Matrix<L>,
    mut new_matrix: Matrix<M>,
    cut_row: usize,
    cut_col: usize,
) -> Matrix<M> {
    let mut vec_old_inner = old_matrix._inner.to_vec();
    vec_old_inner.remove(cut_row);
    for i in 0..vec_old_inner.len() {
        let mut vec_col = vec_old_inner[i].to_vec();
        vec_col.remove(cut_col);
        for j in 0..vec_col.len() {
            new_matrix[[i, j]] = vec_col[j]
        }
    }
    new_matrix
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use spectral::assert_that;
    use spectral::boolean::BooleanAssertions;
    use spectral::numeric::FloatAssertions;

    use super::*;
    use crate::vector4::Vector4;

    #[test]
    fn matrices_constructed_from_rows() {
        let _ = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);
    }

    #[test]
    fn matrices_indexed_by_row_col() {
        let m = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_that!(m[0][0]).is_equal_to(1.0);
        assert_that!(m[0][3]).is_equal_to(4.0);
        assert_that!(m[1][0]).is_equal_to(5.5);
        assert_that!(m[1][2]).is_equal_to(7.5);
        assert_that!(m[2][2]).is_equal_to(11.0);
        assert_that!(m[3][0]).is_equal_to(13.5);
        assert_that!(m[3][2]).is_equal_to(15.5);
    }

    #[test]
    fn matrices_with_the_same_values_are_equal() {
        let m1 = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 8.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 8.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        assert_that!(m1).is_equal_to(m2);
    }

    #[test]
    fn matrices_with_the_different_values_are_not_equal() {
        let m1: Matrix<4> = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 8.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2: Matrix<4> = Matrix::from([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);

        assert_that!(m1).is_not_equal_to(m2);
    }

    #[test]
    fn multiplying_matrices() {
        let m1: Matrix<4> = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2: Matrix<4> = Matrix::from([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let expected: Matrix<4> = Matrix::from([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        let result: Matrix<4> = m1 * m2;
        assert_that!(result).is_equal_to(expected);
    }

    #[test]
    fn multiplying_matrices_by_tuples() {
        let a: Matrix<4> = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let b: Vector4 = Vector4::point(1.0, 2.0, 3.0);
        let expected = Vector4::point(18.0, 24.0, 33.0);

        let result = a * b;
        assert_that!(result).is_equal_to(expected);
    }

    #[test]
    fn multiplying_matrix_by_the_identity_matrix() {
        let a: Matrix<4> = Matrix::from([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);
        let ident = Matrix::identity();

        let result = a * ident;
        assert_that!(result).is_equal_to(a);
    }

    #[test]
    fn multiplying_identity_matrix_by_a_tuple() {
        let tuple: Vector4 = Vector4::new(1.0, 2.0, 3.0, 4.0);

        let result = Matrix::identity() * tuple;

        assert_that!(result).is_equal_to(tuple);
    }

    #[test]
    fn transposing_a_matrix() {
        let a: Matrix<4> = Matrix::from([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        let expected: Matrix<4> = Matrix::from([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);

        assert_that(&a.transpose()).is_equal_to(expected);
    }

    #[test]
    fn transposing_the_identity_matrix_return_identity() {
        let a: Matrix<4> = Matrix::identity();

        assert_that(&a.transpose()).is_equal_to(a);
    }

    #[test]
    fn calculating_the_determinant_of_2x2_matrix() {
        let a: Matrix<2> = Matrix::from([[1.0, 5.0], [-3.0, 2.0]]);

        assert_that!(a.determinant()).is_equal_to(17.0);
    }

    #[test]
    fn calculating_the_determinant_of_a_3x3_matrix() {
        let a: Matrix<3> = Matrix::from([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

        assert_that!(a.determinant()).is_equal_to(-196.0);
    }

    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        let a: Matrix<4> = Matrix::from([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_that!(a.determinant()).is_close_to(-4071.0, 0.001);
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let a: Matrix<4> = Matrix::from([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);

        assert_that!(a.determinant()).is_equal_to(-2120.0);
        assert_that!(a.is_invertible()).is_true();
    }

    #[test]
    fn testing_a_noninvertible_matrix_for_invertibility() {
        let a: Matrix<4> = Matrix::from([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_that!(a.determinant()).is_equal_to(0.0);
        assert_that!(a.is_invertible()).is_false();
    }

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        let a: Matrix<4> = Matrix::from([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        let inverse = a.try_inverse().unwrap();
        let expected: Matrix<4> = Matrix::from([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);

        assert_that!(a.determinant()).is_close_to(532.0, 0.001);
        assert_that!(inverse).is_equal_to(expected);
    }

    #[test]
    fn calculating_the_inverse_of_another_matrix() {
        let a: Matrix<4> = Matrix::from([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        let inverse = a.try_inverse().unwrap();
        let expected: Matrix<4> = Matrix::from([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);

        assert_that!(inverse).is_equal_to(expected);
    }

    #[test]
    fn calculating_the_inverse_of_a_third_matrix() {
        let a: Matrix<4> = Matrix::from([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        let inverse = a.try_inverse().unwrap();
        let expected: Matrix<4> = Matrix::from([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);

        assert_that!(inverse).is_equal_to(expected);
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let a: Matrix<4> = Matrix::from([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let b: Matrix<4> = Matrix::from([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);
        let product = a * b;

        let prod_x_inv = product * b.try_inverse().unwrap();

        assert_that!(prod_x_inv).is_equal_to(a);
    }
}
