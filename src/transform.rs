use nalgebra::Matrix4;

pub trait Transform {
    fn translation(x: f32, y: f32, z: f32) -> Matrix4<f32>;
    fn scaling(x: f32, y: f32, z: f32) -> Matrix4<f32>;
    fn rotation_x(r: f32) -> Matrix4<f32>;
    fn rotation_y(r: f32) -> Matrix4<f32>;
    fn rotation_z(r: f32) -> Matrix4<f32>;
    fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Matrix4<f32>;
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
            0.0, 1.0, 0.0, 0.0,
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

    fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Matrix4<f32> {
        Matrix4::new (
            1.0, xy, xz, 0.0,
            yx, 1.0, yz, 0.0,
            zx, zy, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use nalgebra::{Matrix4, Vector4};
    use spectral::assert_that;
    use spectral::numeric::FloatAssertions;

    use crate::tuple::*;

    use super::*;

    fn vector_values_are_close(actual: Vector4<f32>, expected: Vector4<f32>, tolerance: f32) {
        for row in 0..4 {
            assert_that!(actual[row]).is_close_to(expected[row], tolerance);
        }
    }


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

        vector_values_are_close(half_quarter * p, Vector4::point(0.0, 2.0_f32.sqrt()/2.0, 2.0_f32.sqrt()/2.0), 0.0001);
        vector_values_are_close(full_quarter * p, Vector4::point(0.0, 0.0, 1.0), 0.00001);
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = Vector4::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix4::rotation_x(PI/4.0);
        let inv = half_quarter.try_inverse().unwrap();

        vector_values_are_close(inv * p, Vector4::point(0.0, 2.0_f32.sqrt()/2.0, -2.0_f32.sqrt()/2.0), 0.00001);
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Vector4::point(0.0, 0.0, 1.0);
        let half_quarter = Matrix4::rotation_y(PI/4.0);
        let full_quarter = Matrix4::rotation_y(PI/2.0);

        vector_values_are_close(half_quarter * p, Vector4::point(2.0_f32.sqrt()/2.0, 0.0, 2.0_f32.sqrt()/2.0), 0.00001);
        vector_values_are_close(full_quarter * p, Vector4::point(1.0, 0.0, 0.0), 0.00001);
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = Vector4::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix4::rotation_z(PI/4.0);
        let full_quarter = Matrix4::rotation_z(PI/2.0);

        vector_values_are_close(half_quarter * p, Vector4::point(-2.0_f32.sqrt()/2.0, 2.0_f32.sqrt()/2.0, 0.0), 0.0001);
        vector_values_are_close(full_quarter * p, Vector4::point(-1.0, 0.0, 0.0), 0.0001);
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Matrix4::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Vector4::point(2.0, 3.0, 4.0);
      
        assert_that!(transform * p).is_equal_to(Vector4::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Matrix4::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Vector4::point(2.0, 3.0, 4.0);
      
        assert_that!(transform * p).is_equal_to(Vector4::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Matrix4::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Vector4::point(2.0, 3.0, 4.0);
      
        assert_that!(transform * p).is_equal_to(Vector4::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Matrix4::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Vector4::point(2.0, 3.0, 4.0);
      
        assert_that!(transform * p).is_equal_to(Vector4::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Matrix4::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Vector4::point(2.0, 3.0, 4.0);
      
        assert_that!(transform * p).is_equal_to(Vector4::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Matrix4::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Vector4::point(2.0, 3.0, 4.0);
      
        assert_that!(transform * p).is_equal_to(Vector4::point(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Vector4::point(1.0, 0.0, 1.0);
        let a = Matrix4::rotation_x(PI/2.0);
        let b = Matrix4::scaling(5.0, 5.0, 5.0);
        let c = Matrix4::translation(10.0, 5.0, 7.0);

        // apply rotation first
        let p2 = a * p;
        vector_values_are_close(p2, Vector4::point(1.0, -1.0, 0.0), 0.00001);

        // then apply scaling
        let p3 = b * p2;
        vector_values_are_close(p3, Vector4::point(5.0, -5.0, 0.0), 0.00001);

        // then apply translation
        let p4 = c * p3;
        vector_values_are_close(p4, Vector4::point(15.0, 0.0, 7.0), 0.00001);
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Vector4::point(1.0, 0.0, 1.0);
        let a = Matrix4::rotation_x(PI/2.0);
        let b = Matrix4::scaling(5.0, 5.0, 5.0);
        let c = Matrix4::translation(10.0, 5.0, 7.0);

        let t = c * b * a;

        vector_values_are_close(t * p, Vector4::point(15.0, 0.0, 7.0), 0.00001);
    }

//    #[test]
//    fn the_transformation_matrix_for_the_default_orientation() {
//        let from = Vector4::point(0.0, 0.0, 0.0);
//        let to = Vector4::point(0.0, 0.0, -1.0);
//        let up = Vector4::vector(0.0, 1.0, 0.0);
//
//        let t = view_transform(from, to, up);
//
//        assert_that!(t).is_equal_to(Matrix4::identity());
//    }
//
//    #[test]
//    fn a_view_transformation_matrix_looking_in_positive_z_direction() {
//        let from = Vector4::point(0.0, 0.0, 0.0);
//        let to = Vector4::point(0.0, 0.0, 1.0);
//        let up = Vector4::vector(0.0, 1.0, 0.0);
//
//        let t = view_transform(from, to, up);
//
//        assert_that!(t).is_equal_to(Matrix4::scaling(-1.0, 1.0, -1.0));
//    }
//
//    #[test]
//    fn the_view_transformation_moves_the_world() {
//        let from = Vector4::point(0.0, 0.0, 8.0);
//        let to = Vector4::point(0.0, 0.0, 0.0);
//        let up = Vector4::vector(0.0, 1.0, 0.0);
//
//        let t = view_transform(from, to, up);
//
//        assert_that!(t).is_equal_to(Matrix4::translation(0.0, 0.0, -8.0));
//    }
//
//    #[test]
//    fn an_arbitrary_view_transformation() {
//        let from = Vector4::point(1.0, 3.0, 2.0);
//        let to = Vector4::point(4.0, -2.0, 8.0);
//        let up = Vector4::vector(1.0, 1.0, 0.0);
//
//        let t = view_transform(from, to, up);
//
//        assert_that!(t).is_equal_to(Matrix4::new(
//            -0.50709, 0.50709, 0.67612, -2.36643,
//            0.76772, 0.60609, 0.12122, -2.82843,
//            -0.35857, 0.59761, -0.71714, 0.00000,
//            0.00000, 0.00000, 0.00000, 1.00000,
//       ));
//    }
}
