use nalgebra::Matrix4;

pub trait Transform {
    fn translation(x: f32, y: f32, z: f32) -> Matrix4<f32>;
    fn scaling(x: f32, y: f32, z: f32) -> Matrix4<f32>;
    fn rotation_x(r: f32) -> Matrix4<f32>;
    fn rotation_y(r: f32) -> Matrix4<f32>;
    fn rotation_z(r: f32) -> Matrix4<f32>;
}

impl Transform for Matrix4<f32> {
    fn translation(x: f32, y: f32, z: f32) -> Matrix4<f32> {
        Matrix4::new(
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    fn scaling(x: f32, y: f32, z: f32) -> Matrix4<f32> {
        Matrix4::new(
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    fn rotation_x(r: f32) -> Matrix4<f32> {
        Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, r.cos(), -r.sin(), 0.0,
            0.0, r.sin(), r.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    fn rotation_y(r: f32) -> Matrix4<f32> {
        Matrix4::new(
            r.cos(), 0.0, r.sin(), 0.0,
            0.0, r, 1.0, 0.0,
            -r.sin(), 0.0, r.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    fn rotation_z(r: f32) -> Matrix4<f32> {
        Matrix4::new(
            r.cos(), -r.sin(), 0.0, 0.0,
            r.sin(), r.cos(), 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use nalgebra::{Matrix4, Vector4};
    use spectral::assert_that;

    use crate::tuple::*;

    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = Matrix4::translation(5.0, -3.0, 2.0);
        let p = Vector4::point(-3.0, 4.0, 5.0);

        assert_that!(transform * p).is_equal_to(Vector4::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = Matrix4::translation(5.0, -3.0, 2.0);
        let inv = transform.try_inverse().unwrap();
        let p = Vector4::point(-3.0, 4.0, 5.0);

        assert_that!(inv * p).is_equal_to(Vector4::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Matrix4::translation(5.0, -3.0, 2.0);
        let v = Vector4::vector(-3.0, 4.0, 5.0);

        assert_that!(transform * v).is_equal_to(v);
    }

    #[test]
    fn scaling_matrix_applied_to_a_point() {
        let transform = Matrix4::scaling(2.0, 3.0, 4.0);
        let  p = Vector4::point(-4.0, 6.0, 8.0);

        assert_that!(transform * p).is_equal_to(Vector4::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_matrix_applied_to_a_vector() {
        let transform = Matrix4::scaling(2.0, 3.0, 4.0);
        let v = Vector4::vector(-4.0, 6.0, 8.0);

        assert_that!(transform * v).is_equal_to(Vector4::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = Matrix4::scaling(2.0, 3.0, 4.0);
        let inv = transform.try_inverse().unwrap();
        let v = Vector4::vector(-4.0, 6.0, 8.0);

        assert_that!(inv * v).is_equal_to(Vector4::vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = Matrix4::scaling(-1.0, 1.0, 1.0);
        let p = Vector4::point(2.0, 3.0, 4.0);

        assert_that!(transform * p).is_equal_to(Vector4::point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = Vector4::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix4::rotation_x(PI/4.0);
        let full_quarter = Matrix4::rotation_x(PI/2.0);

        assert_that!(half_quarter * p).is_equal_to(Vector4::point(0.0, 2.0_f32.sqrt()/2.0, 2.0_f32.sqrt()/2.0));
        assert_that!(full_quarter * p).is_equal_to(Vector4::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = Vector4::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix4::rotation_x(PI/4.0);
        let inv = half_quarter.try_inverse().unwrap();

        assert_that!(inv * p).is_equal_to(Vector4::point(0.0, 2.0_f32.sqrt()/2.0, -2.0_f32.sqrt()/2.0));
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Vector4::point(0.0, 0.0, 1.0);
        let half_quarter = Matrix4::rotation_y(PI/4.0);
        let full_quarter = Matrix4::rotation_y(PI/2.0);

        assert_that!(half_quarter * p).is_equal_to(Vector4::point(2.0_f32.sqrt()/2.0, 0.0, 2.0_f32.sqrt()/2.0));
        assert_that!(full_quarter * p).is_equal_to(Vector4::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = Vector4::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix4::rotation_z(PI/4.0);
        let full_quarter = Matrix4::rotation_z(PI/2.0);

        assert_that!(half_quarter * p).is_equal_to(Vector4::point(-2.0_f32.sqrt()/2.0, 2.0_f32.sqrt()/2.0, 0.0));
        assert_that!(full_quarter * p).is_equal_to(Vector4::point(-1.0, 0.0, 0.0));
    }
}
