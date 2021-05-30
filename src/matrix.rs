
/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use nalgebra::{Matrix2, Matrix3, Matrix4, Vector4};
    use spectral::assert_that;
    use spectral::boolean::BooleanAssertions;
    use spectral::numeric::FloatAssertions;

    use crate::tuple::*;

    #[test]
    fn matrices_constructed_from_rows() {
        let _: Matrix4<f32> = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            5.5, 6.5, 7.5, 8.5,
            9.0, 10.0, 11.0, 12.0,
            13.5, 14.5, 15.5, 16.5,
        );
    }

    #[test]
    fn matrices_indexed_by_row_col() {
        let m: Matrix4<f32> = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            5.5, 6.5, 7.5, 8.5,
            9.0, 10.0, 11.0, 12.0,
            13.5, 14.5, 15.5, 16.5,
        );

        assert_that!(m[(0, 0)]).is_equal_to(1.0);
        assert_that!(m[(0, 3)]).is_equal_to(4.0);
        assert_that!(m[(1, 0)]).is_equal_to(5.5);
        assert_that!(m[(1, 2)]).is_equal_to(7.5);
        assert_that!(m[(2, 2)]).is_equal_to(11.0);
        assert_that!(m[(3, 0)]).is_equal_to(13.5);
        assert_that!(m[(3, 2)]).is_equal_to(15.5);
    }

    #[test]
    fn matrices_with_the_same_values_are_equal() {
        let m1: Matrix4<f32> = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 8.0, 6.0,
            5.0, 4.0, 3.0, 2.0,
        );
        let m2: Matrix4<f32> = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 8.0, 6.0,
            5.0, 4.0, 3.0, 2.0,
        );

        assert_that!(m1).is_equal_to(m2);
    }

    #[test]
    fn matrices_with_the_different_values_are_not_equal() {
        let m1: Matrix4<f32> = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 8.0, 6.0,
            5.0, 4.0, 3.0, 2.0,
        );
        let m2: Matrix4<f32> = Matrix4::new(
            2.0, 3.0, 4.0, 5.0,
            6.0, 7.0, 8.0, 9.0,
            8.0, 7.0, 6.0, 5.0,
            4.0, 3.0, 2.0, 1.0,
        );

        assert_that!(m1).is_not_equal_to(m2);
    }

    #[test]
    fn multiplying_matrices() {
        let m1: Matrix4<f32> = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0,
        );
        let m2: Matrix4<f32> = Matrix4::new(
            -2.0, 1.0, 2.0, 3.0,
            3.0, 2.0, 1.0, -1.0,
            4.0, 3.0, 6.0, 5.0,
            1.0, 2.0, 7.0, 8.0,
        );
        let expected: Matrix4<f32> = Matrix4::new(
            20.0, 22.0, 50.0, 48.0,
            44.0, 54.0, 114.0, 108.0,
            40.0, 58.0, 110.0, 102.0,
            16.0, 26.0, 46.0, 42.0,
        );

        let result: Matrix4<f32> = &m1 * &m2;
        assert_that!(result).is_equal_to(expected);
    }

    #[test]
    fn multiplying_matrices_by_tuples() {
        let a: Matrix4<f32> = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0,
        );
        let b: Vector4<f32> = Vector4::point(1.0, 2.0, 3.0);
        let expected = Vector4::point(18.0, 24.0, 33.0);

        let result = &a * &b;
        assert_that!(result).is_equal_to(expected);
    }

    #[test]
    fn multiplying_matrix_by_the_identity_matrix() {
        let a: Matrix4<f32> = Matrix4::new(
            0.0, 1.0, 2.0, 4.0,
            1.0, 2.0, 4.0, 8.0,
            2.0, 4.0, 8.0, 16.0,
            4.0, 8.0, 16.0, 32.0,
        );
        let ident = Matrix4::identity();

        let result = &a * &ident;
        assert_that!(result).is_equal_to(a);
    }

    #[test]
    fn multiplying_identity_matrix_by_a_tuple() {
        let tuple: Vector4<f32> = Vector4::new(1.0, 2.0, 3.0, 4.0);

        let result = Matrix4::identity() * &tuple;

        assert_that!(result).is_equal_to(tuple);
    }

    #[test]
    fn transposing_a_matrix() {
        let a: Matrix4<f32> = Matrix4::new(
            0.0, 9.0, 3.0, 0.0,
            9.0, 8.0, 0.0, 8.0,
            1.0, 8.0, 5.0, 3.0,
            0.0, 0.0, 5.0, 8.0,
        );
        let expected: Matrix4<f32> = Matrix4::new(
            0.0, 9.0, 1.0, 0.0,
            9.0, 8.0, 8.0, 0.0,
            3.0, 0.0, 5.0, 5.0,
            0.0, 8.0, 3.0, 8.0,
        );

        assert_that(&a.transpose()).is_equal_to(expected);
    }

    #[test]
    fn transposing_the_identity_matrix_return_identity() {
        let a: Matrix4<f32> = Matrix4::identity();

        assert_that(&a.transpose()).is_equal_to(a);
    }

    #[test]
    fn calculating_the_determinant_of_2x2_matrix() {
        let a: Matrix2<f32> = Matrix2::new(
            1.0, 5.0,
            -3.0, 2.0,
        );

        assert_that!(a.determinant()).is_equal_to(17.0);
    }

    #[test]
    fn calculating_the_determinant_of_a_3x3_matrix() {
        let a: Matrix3<f32> = Matrix3::new(
            1.0, 2.0, 6.0,
            -5.0, 8.0, -4.0,
            2.0, 6.0, 4.0,
        );

        assert_that!(a.determinant()).is_equal_to(-196.0);
    }

    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        let a: Matrix4<f32> = Matrix4::new(
            -2.0, -8.0, 3.0, 5.0,
            -3.0, 1.0, 7.0, 3.0,
            1.0, 2.0, -9.0, 6.0,
            -6.0, 7.0, 7.0, -9.0,
        );

        assert_that!(a.determinant()).is_close_to(-4071.0, 0.001);
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let a: Matrix4<f32> = Matrix4::new(
            6.0, 4.0, 4.0, 4.0,
            5.0, 5.0, 7.0, 6.0,
            4.0, -9.0, 3.0, -7.0,
            9.0, 1.0, 7.0, -6.0,
        );

        assert_that!(a.determinant()).is_equal_to(-2120.0);
        assert_that!(a.is_invertible()).is_true();
    }

    #[test]
    fn testing_a_noninvertible_matrix_for_invertibility() {
        let a: Matrix4<f32> = Matrix4::new(
            -4.0, 2.0, -2.0, -3.0,
            9.0, 6.0, 2.0, 6.0,
            0.0, -5.0, 1.0, -5.0,
            0.0, 0.0, 0.0, 0.0,
        );

        assert_that!(a.determinant()).is_equal_to(0.0);
        assert_that!(a.is_invertible()).is_false();
    }

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        let a: Matrix4<f32> = Matrix4::new(
            -5.0, 2.0, 6.0, -8.0,
            1.0, -5.0, 1.0, 8.0,
            7.0, 7.0, -6.0, -7.0,
            1.0, -3.0, 7.0, 4.0,
        );
        let inverse = a.try_inverse().unwrap();
        let expected: Matrix4<f32> = Matrix4::new(
            0.21805, 0.45113, 0.24060, -0.04511,
            -0.80827, -1.45677, -0.44361, 0.52068,
            -0.07895, -0.22368, -0.05263, 0.19737,
            -0.52256, -0.81391, -0.30075, 0.30639,
        );

        assert_that!(a.determinant()).is_close_to(532.0, 0.001);
        for row in 0..4 {
            for col in 0..4 {
                assert_that!(inverse[(row, col)]).is_close_to(expected[(row, col)], 0.00001);
            }
        }
    }

    #[test]
    fn calculating_the_inverse_of_another_matrix() {
        let a: Matrix4<f32> = Matrix4::new(
            8.0, -5.0, 9.0, 2.0,
            7.0, 5.0, 6.0, 1.0,
            -6.0, 0.0, 9.0, 6.0,
            -3.0, 0.0, -9.0, -4.0,
        );
        let inverse = a.try_inverse().unwrap();
        let expected: Matrix4<f32> = Matrix4::new(
            -0.15385, -0.15385, -0.28205, -0.53846,
            -0.07692, 0.12308, 0.02564, 0.03077,
            0.35897, 0.35897, 0.43590, 0.92308,
            -0.69231, -0.69231, -0.76923, -1.92308,
        );

        for row in 0..4 {
            for col in 0..4 {
                assert_that!(inverse[(row, col)]).is_close_to(expected[(row, col)], 0.00001);
            }
        }
    }

    #[test]
    fn calculating_the_inverse_of_a_third_matrix() {
        let a: Matrix4<f32> = Matrix4::new(
            9.0, 3.0, 0.0, 9.0,
            -5.0, -2.0, -6.0, -3.0,
            -4.0, 9.0, 6.0, 4.0,
            -7.0, 6.0, 6.0, 2.0,
        );
        let inverse = a.try_inverse().unwrap();
        let expected: Matrix4<f32> = Matrix4::new(
            -0.04074, -0.07778, 0.14444, -0.22222,
            -0.07778, 0.03333, 0.36667, -0.33333,
            -0.02901, -0.14630, -0.10926, 0.12963,
            0.17778, 0.06667, -0.26667, 0.33333,
        );

        for row in 0..4 {
            for col in 0..4 {
                assert_that!(inverse[(row, col)]).is_close_to(expected[(row, col)], 0.00001);
            }
        }
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let a: Matrix4<f32> = Matrix4::new(
            3.0, -9.0, 7.0, 3.0,
            3.0, -8.0, 2.0, -9.0,
            -4.0, 4.0, 4.0, 1.0,
            -6.0, 5.0, -1.0, 1.0,
        );
        let b: Matrix4<f32> = Matrix4::new(
            8.0, 2.0, 2.0, 2.0,
            3.0, -1.0, 7.0, 0.0,
            7.0, 0.0, 5.0, 4.0,
            6.0, -2.0, 0.0, 5.0,
        );
        let product = &a * &b;

        let prod_x_inv = product * b.try_inverse().unwrap();

        for row in 0..4 {
            for col in 0..4 {
                assert_that!(prod_x_inv[(row, col)]).is_close_to(a[(row, col)], 0.00001);
            }
        }
    }
}
