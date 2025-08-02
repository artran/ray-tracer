use std::any::Any;

use crate::consts::EPSILON;
use crate::material::{Material, MaterialBuilder};
use crate::matrix::Matrix;
use crate::shape::Shape;
use crate::vector4::Vector4;

#[derive(Clone, Debug, PartialEq)]
pub struct Plane {
    // Note: we store the inverse of the transform as an optimisation.
    inv_transform: Matrix<4>,
    material: Material,
}

pub struct PlaneBuilder {
    transform: Matrix<4>,
    material: Material,
}

impl Shape for Plane {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn shape_eq(&self, other: &dyn Shape) -> bool {
        other.as_any().downcast_ref::<Self>() == Some(self)
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn transformation(&self) -> Matrix<4> {
        self.inv_transform.try_inverse().unwrap()
    }

    fn inv_transform(&self) -> &Matrix<4> {
        &self.inv_transform
    }

    fn local_intersect(&self, ray: &crate::ray::Ray) -> Vec<f32> {
        let mut result = Vec::default();

        if ray.direction.y.abs() < EPSILON {
            return result;
        }

        let t = -ray.origin.y / ray.direction.y;
        result.push(t);
        result
    }

    fn local_normal_at(&self, _world_point: crate::vector4::Vector4) -> crate::vector4::Vector4 {
        Vector4::vector(0.0, 1.0, 0.0)
    }

    fn lighting(
        &self,
        light: &crate::light::PointLight,
        point: crate::vector4::Vector4,
        eye_vector: crate::vector4::Vector4,
        normal_vector: crate::vector4::Vector4,
        in_shadow: bool,
    ) -> crate::color::Color {
        self.material
            .lighting(light, point, eye_vector, normal_vector, in_shadow)
    }
}

impl PlaneBuilder {
    pub fn new() -> Self {
        Self {
            transform: Matrix::identity(),
            material: MaterialBuilder::new().build(),
        }
    }

    pub fn with_transform(mut self, transform: Matrix<4>) -> Self {
        self.transform = transform;

        self
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;

        self
    }

    pub fn build(self) -> impl Shape {
        Plane {
            inv_transform: self.transform.try_inverse().unwrap(),
            material: self.material,
        }
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use rstest::*;
    use spectral::prelude::*;

    use super::*;
    use crate::ray::Ray;
    use crate::vector4::Vector4;

    #[fixture]
    fn test_shape() -> impl Shape {
        PlaneBuilder::new().build()
    }

    #[rstest]
    fn the_normal_of_a_plane_is_constant_everywhere(test_shape: impl Shape) {
        let n1 = test_shape.normal_at(&Vector4::point(0.0, 0.0, 0.0));
        let n2 = test_shape.normal_at(&Vector4::point(10.0, 0.0, -10.0));
        let n3 = test_shape.normal_at(&Vector4::point(-5.0, 0.0, 150.0));

        assert_that!(n1).is_equal_to(Vector4::vector(0.0, 1.0, 0.0));
        assert_that!(n2).is_equal_to(Vector4::vector(0.0, 1.0, 0.0));
        assert_that!(n3).is_equal_to(Vector4::vector(0.0, 1.0, 0.0));
    }

    #[rstest]
    fn rays_parallel_to_the_plane_do_not_intersect_it(test_shape: impl Shape) {
        let r = Ray::new(
            Vector4::point(0.0, 10.0, 0.0),
            Vector4::vector(0.0, 0.0, 1.0),
        );

        let xs = test_shape.intersect(&r);
        assert_that!(xs).is_empty();
    }

    #[rstest]
    fn rays_coplanar_to_the_plane_do_not_intersect_it(test_shape: impl Shape) {
        let r = Ray::new(
            Vector4::point(0.0, 0.0, 0.0),
            Vector4::vector(0.0, 0.0, 1.0),
        );

        let xs = test_shape.intersect(&r);
        assert_that!(xs).is_empty();
    }

    #[rstest]
    fn rays_intersecting_the_plane_from_above_intersect_it(test_shape: impl Shape) {
        let r = Ray::new(
            Vector4::point(0.0, 1.0, 0.0),
            Vector4::vector(0.0, -1.0, 0.0),
        );

        let xs = test_shape.intersect(&r);
        assert_that!(xs.len()).is_equal_to(1);
        assert_that!(xs[0]).is_equal_to(1.0);
    }

    #[rstest]
    fn rays_intersecting_the_plane_from_below_intersect_it(test_shape: impl Shape) {
        let r = Ray::new(
            Vector4::point(0.0, -1.0, 0.0),
            Vector4::vector(0.0, 1.0, 0.0),
        );

        let xs = test_shape.intersect(&r);
        assert_that!(xs.len()).is_equal_to(1);
        assert_that!(xs[0]).is_equal_to(1.0);
    }
}
