use crate::color::Color;
use crate::light::PointLight;
use crate::material::{Material, MaterialBuilder};
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::vector4::Vector4;

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    // Note: we store the inverse of the transform as an optimisation.
    inv_transform: Matrix<4>,
    material: Material,
}

pub struct SphereBuilder {
    transform: Matrix<4>,
    material: Material,
}

impl Shape for Sphere {
    fn material(&self) -> &Material {
        &self.material
    }

    fn transformation(&self) -> Matrix<4> {
        self.inv_transform.try_inverse().unwrap()
    }

    fn inv_transform(&self) -> &Matrix<4> {
        &self.inv_transform
    }

    fn local_intersect(&self, transformed_ray: &Ray) -> Vec<f32> {
        let sphere_to_ray = transformed_ray.origin - Vector4::point(0.0, 0.0, 0.0);
        let a = transformed_ray.direction.dot(&transformed_ray.direction);
        let b = 2.0 * transformed_ray.direction.dot(&sphere_to_ray);
        let c = &sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant: f32 = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Vec::default();
        }

        let two_a = 2.0 * a;
        let root_disc = discriminant.sqrt();
        let t1 = (-b - root_disc) / (two_a);
        let t2 = (-b + root_disc) / (two_a);

        let mut result = Vec::default();
        result.push(t1);
        result.push(t2);
        result
    }

    fn local_normal_at(&self, object_point: Vector4) -> Vector4 {
        object_point - Vector4::point(0.0, 0.0, 0.0)
    }

    fn lighting(
        &self,
        light: &PointLight,
        point: Vector4,
        eye_vector: Vector4,
        normal_vector: Vector4,
        in_shadow: bool,
    ) -> Color {
        self.material
            .lighting(light, point, eye_vector, normal_vector, in_shadow)
    }
}

impl SphereBuilder {
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
        Sphere {
            inv_transform: self.transform.try_inverse().unwrap(),
            material: self.material,
        }
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */
// TODO: Go through tests and move to Shape or delete as needed

#[cfg(test)]
mod tests {
    use std::f32::consts::{FRAC_1_SQRT_2, PI};

    use spectral::prelude::*;

    use crate::consts::EPSILON;
    use crate::transform::Transform;

    use super::*;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(
            Vector4::point(0.0, 0.0, -5.0),
            Vector4::vector(0.0, 0.0, 1.0),
        );
        let s = SphereBuilder::new().build();

        let xs = s.intersect(&r);

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0]).is_equal_to(4.0);
        assert_that!(xs[1]).is_equal_to(6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(
            Vector4::point(0.0, 1.0, -5.0),
            Vector4::vector(0.0, 0.0, 1.0),
        );
        let s = SphereBuilder::new().build();

        let xs = s.intersect(&r);

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0]).is_equal_to(5.0);
        assert_that!(xs[1]).is_equal_to(5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(
            Vector4::point(0.0, 2.0, -5.0),
            Vector4::vector(0.0, 0.0, 1.0),
        );
        let s = SphereBuilder::new().build();

        let xs = s.intersect(&r);

        assert_that!(xs.len()).is_equal_to(0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(
            Vector4::point(0.0, 0.0, 0.0),
            Vector4::vector(0.0, 0.0, 1.0),
        );
        let s = SphereBuilder::new().build();

        let xs = s.intersect(&r);

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0]).is_equal_to(-1.0);
        assert_that!(xs[1]).is_equal_to(1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(
            Vector4::point(0.0, 0.0, 5.0),
            Vector4::vector(0.0, 0.0, 1.0),
        );
        let s = SphereBuilder::new().build();

        let xs = s.intersect(&r);

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0]).is_equal_to(-6.0);
        assert_that!(xs[1]).is_equal_to(-4.0);
    }

    #[test]
    fn a_spheres_default_transformation() {
        let s = SphereBuilder::new().build();

        assert_that!(s.transformation()).is_equal_to(Matrix::identity());
    }

    #[test]
    fn changing_a_spheres_transformation() {
        let t = Matrix::translation(2.0, 3.0, 4.0);
        let expected = Matrix::from([
            [1.0, 0.0, 0.0, 2.0],
            [0.0, 1.0, 0.0, 3.0],
            [0.0, 0.0, 1.0, 4.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let s = SphereBuilder::new().with_transform(t.clone()).build();

        assert_that!(s.transformation()).is_equal_to(expected);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(
            Vector4::point(0.0, 0.0, -5.0),
            Vector4::vector(0.0, 0.0, 1.0),
        );
        let s = SphereBuilder::new()
            .with_transform(Matrix::scaling(2.0, 2.0, 2.0))
            .build();

        let xs = s.intersect(&r);

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0]).is_equal_to(3.0);
        assert_that!(xs[1]).is_equal_to(7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(
            Vector4::point(0.0, 0.0, -5.0),
            Vector4::vector(0.0, 0.0, 1.0),
        );
        let s = SphereBuilder::new()
            .with_transform(Matrix::translation(5.0, 0.0, 0.0))
            .build();

        let xs = s.intersect(&r);

        assert_that!(xs.len()).is_equal_to(0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = SphereBuilder::new().build();
        let n = s.normal_at(&Vector4::point(1.0, 0.0, 0.0));
        assert_that!(n).is_equal_to(Vector4::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = SphereBuilder::new().build();
        let n = s.normal_at(&Vector4::point(0.0, 1.0, 0.0));
        assert_that!(n).is_equal_to(Vector4::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = SphereBuilder::new().build();
        let n = s.normal_at(&Vector4::point(0.0, 0.0, 1.0));
        assert_that!(n).is_equal_to(Vector4::vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_non_axial_point() {
        let s = SphereBuilder::new().build();

        let n = s.normal_at(&Vector4::point(
            3.0_f32.sqrt() / 3.0,
            3.0_f32.sqrt() / 3.0,
            3.0_f32.sqrt() / 3.0,
        ));

        let expected = Vector4::vector(
            3.0_f32.sqrt() / 3.0,
            3.0_f32.sqrt() / 3.0,
            3.0_f32.sqrt() / 3.0,
        );
        assert_that!(n.x).is_close_to(expected.x, EPSILON);
        assert_that!(n.y).is_close_to(expected.y, EPSILON);
        assert_that!(n.z).is_close_to(expected.z, EPSILON);
        assert_that!(n.w).is_equal_to(expected.w);
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = SphereBuilder::new().build();

        let n = s.normal_at(&Vector4::point(
            3.0_f32.sqrt() / 3.0,
            3.0_f32.sqrt() / 3.0,
            3.0_f32.sqrt() / 3.0,
        ));

        let normalized: Vector4 = n.normalize();
        assert_that!(n.x).is_close_to(normalized.x, EPSILON);
        assert_that!(n.y).is_close_to(normalized.y, EPSILON);
        assert_that!(n.z).is_close_to(normalized.z, EPSILON);
        assert_that!(n.w).is_equal_to(normalized.w);
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let s = SphereBuilder::new()
            .with_transform(Matrix::translation(0.0, 1.0, 0.0))
            .build();

        let n = s.normal_at(&Vector4::point(0.0, 1.70711, -FRAC_1_SQRT_2));

        let expected = Vector4::vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
        assert_that!(n.x).is_close_to(expected.x, EPSILON);
        assert_that!(n.y).is_close_to(expected.y, EPSILON);
        assert_that!(n.z).is_close_to(expected.z, EPSILON);
        assert_that!(n.w).is_equal_to(expected.w);
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let t = Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(PI / 5.0);
        let s = SphereBuilder::new().with_transform(t).build();

        let n = s.normal_at(&Vector4::point(
            0.0,
            2.0_f32.sqrt() / 2.0,
            -2.0_f32.sqrt() / 2.0,
        ));

        let expected = Vector4::vector(0.0, 0.97014, -0.24254);
        assert_that!(n.x).is_close_to(expected.x, EPSILON);
        assert_that!(n.y).is_close_to(expected.y, EPSILON);
        assert_that!(n.z).is_close_to(expected.z, EPSILON);
        assert_that!(n.w).is_equal_to(expected.w);
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = SphereBuilder::new().build();
        let m = s.material();
        assert_that!(m).is_equal_to(&MaterialBuilder::new().build());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let m = MaterialBuilder::new().with_ambient(1.0).build();
        let s = SphereBuilder::new().with_material(m.clone()).build();

        assert_that!(s.material()).is_equal_to(&m);
    }
}
