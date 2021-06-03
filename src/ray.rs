use nalgebra::{Matrix4, Vector4};

use crate::tuple::*;

pub struct Ray {
    origin: Vector4<f32>,
    direction: Vector4<f32>,
}

impl Ray {
    pub fn new(origin: Vector4<f32>, direction: Vector4<f32>) -> Self {
        assert!(origin.is_point(), "Origin must be a point");
        assert!(direction.is_vector(), "Direction must be a vector");
        Self { origin, direction }
    }

    pub fn position(&self, t: f32) -> Vector4<f32> {
        self.origin + self.direction * t
    }

    fn transform(&self, transformation: &Matrix4<f32>) -> Self {
        Self {
            origin: transformation * self.origin,
            direction: transformation * self.direction
        }
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use spectral::assert_that;

    use crate::transform::*;
    use super::*;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Vector4::point(1.0, 2.0, 3.0);
        let direction = Vector4::vector(4.0, 5.0, 6.0);

        let r = Ray::new(origin, direction);

        assert_that!(r.origin).is_equal_to(origin);
        assert_that!(r.direction).is_equal_to(direction);
    }

    #[test]
    #[should_panic(expected = "Origin must be a point")]
    fn passing_a_vector_for_origin_causes_panic() {
        let origin = Vector4::vector(1.0, 2.0, 3.0);
        let direction = Vector4::vector(4.0, 5.0, 6.0);

        Ray::new(origin, direction);
    }

    #[test]
    #[should_panic(expected = "Direction must be a vector")]
    fn passing_a_point_for_direction_causes_panic() {
        let origin = Vector4::point(1.0, 2.0, 3.0);
        let direction = Vector4::point(4.0, 5.0, 6.0);

        Ray::new(origin, direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let ray = Ray::new(
            Vector4::point(2.0, 3.0, 4.0),
            Vector4::vector(1.0, 0.0, 0.0),
        );

        assert_that!(ray.position(0.0)).is_equal_to(Vector4::point(2.0, 3.0, 4.0));
        assert_that!(ray.position(1.0)).is_equal_to(Vector4::point(3.0, 3.0, 4.0));
        assert_that!(ray.position(-1.0)).is_equal_to(Vector4::point(1.0, 3.0, 4.0));
        assert_that!(ray.position(2.5)).is_equal_to(Vector4::point(4.5, 3.0, 4.0));
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(Vector4::point(1.0, 2.0, 3.0), Vector4::vector(0.0, 1.0, 0.0));
        let m = Matrix4::translation(3.0, 4.0, 5.0);

        let r2 = r.transform(&m);

        assert_that!(r2.origin).is_equal_to(Vector4::point(4.0, 6.0, 8.0));
        assert_that!(r2.direction).is_equal_to(Vector4::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(Vector4::point(1.0, 2.0, 3.0), Vector4::vector(0.0, 1.0, 0.0));
        let m = Matrix4::scaling(2.0, 3.0, 4.0);

        let r2 = r.transform(&m);

        assert_that!(r2.origin).is_equal_to(Vector4::point(2.0, 6.0, 12.0));
        assert_that!(r2.direction).is_equal_to(Vector4::vector(0.0, 3.0, 0.0));
    }
}
