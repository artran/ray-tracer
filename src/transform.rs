use nalgebra::Matrix4;

pub trait Transform {
    fn translation(x: f32, y: f32, z: f32) -> Matrix4<f32>;
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
}
