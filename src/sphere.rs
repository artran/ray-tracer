use nalgebra::{Matrix4, Vector4};

use crate::intersection::{Intersection, Intersections};
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::tuple::Tuple;

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    // Note: we store the inverse of the transform as an optimisation.
    inv_transform: Matrix4<f32>,

    material: Material,
}

impl Sphere {
    pub fn intersect(&self, ray: &Ray) -> Option<Intersections> {
        let transformed_ray = ray.transform(&self.inv_transform);

        let sphere_to_ray = transformed_ray.origin - Vector4::point(0.0, 0.0, 0.0);
        let a = transformed_ray.direction.dot(&transformed_ray.direction);
        let b = 2.0 * transformed_ray.direction.dot(&sphere_to_ray);
        let c = &sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let two_a = 2.0 * a;
        let root_disc = discriminant.sqrt();
        let t1 = (-b - root_disc) / (two_a);
        let t2 = (-b + root_disc) / (two_a);

        let mut result = Intersections::default();
        result.push(Intersection::new(t1, self));
        result.push(Intersection::new(t2, self));
        Some(result)
    }

    pub fn normal_at(&self, world_point: &Vector4<f32>) -> Vector4<f32> {
        let object_point = self.inv_transform * world_point;
        let object_normal = object_point - Vector4::point(0.0, 0.0, 0.0);
        let mut world_normal = self.inv_transform.transpose() * object_normal;
        world_normal.w = 0.0;

        (world_normal).normalize()
    }
}

impl Shape for Sphere {
    fn set_transform(&mut self, transform: Matrix4<f32>) {
        // As an optimisation we invert the transform before storing it.
        self.inv_transform = transform.try_inverse().unwrap();
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn get_transform(&self) -> Matrix4<f32> {
        self.inv_transform.try_inverse().unwrap()
    }

    fn get_material(&self) -> Material {
        self.material.clone()
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            // The inverse of the identity is the identity
            inv_transform: Matrix4::identity(),
            material: Material::default()
        }
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use std::f32::consts::{FRAC_1_SQRT_2, PI};

    use nalgebra::Matrix4;
    use spectral::assert_that;
    use spectral::numeric::FloatAssertions;
    use spectral::option::OptionAssertions;

    use crate::transform::Transform;

    use super::*;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Vector4::point(0.0, 0.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(&r).unwrap();

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0].t).is_equal_to(4.0);
        assert_that!(xs[1].t).is_equal_to(6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Vector4::point(0.0, 1.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(&r).unwrap();

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0].t).is_equal_to(5.0);
        assert_that!(xs[1].t).is_equal_to(5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Vector4::point(0.0, 2.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(&r);

        assert_that!(xs).is_none();
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Vector4::point(0.0, 0.0, 0.0), Vector4::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(&r).unwrap();

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0].t).is_equal_to(-1.0);
        assert_that!(xs[1].t).is_equal_to(1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Vector4::point(0.0, 0.0, 5.0), Vector4::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(&r).unwrap();

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0].t).is_equal_to(-6.0);
        assert_that!(xs[1].t).is_equal_to(-4.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Vector4::point(0.0, 0.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(&r).unwrap();

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0].object).is_equal_to(&s);
        assert_that!(xs[1].object).is_equal_to(&s);
    }

    #[test]
    fn a_spheres_default_transformation() {
        let s = Sphere::default();

        assert_that!(s.inv_transform).is_equal_to(Matrix4::identity());
    }

    #[test]
    fn changing_a_spheres_transformation() {
        let mut s = Sphere::default();
        let t = Matrix4::translation(2.0, 3.0, 4.0);

        s.inv_transform = t;

        assert_that!(s.inv_transform).is_equal_to(t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Vector4::point(0.0, 0.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::default();
        s.set_transform(Matrix4::scaling(2.0, 2.0, 2.0));

        let xs = s.intersect(&r).unwrap();

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0].t).is_equal_to(3.0);
        assert_that!(xs[1].t).is_equal_to(7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Vector4::point(0.0, 0.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::default();
        s.set_transform(Matrix4::translation(5.0, 0.0, 0.0));

        let xs = s.intersect(&r);

        assert_that!(xs).is_none();
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::default();
        let n = s.normal_at(&Vector4::point(1.0, 0.0, 0.0));
        assert_that!(n).is_equal_to(Vector4::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::default();
        let n = s.normal_at(&Vector4::point(0.0, 1.0, 0.0));
        assert_that!(n).is_equal_to(Vector4::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::default();
        let n = s.normal_at(&Vector4::point(0.0, 0.0, 1.0));
        assert_that!(n).is_equal_to(Vector4::vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_non_axial_point() {
        let s = Sphere::default();

        let n = s.normal_at(&Vector4::point(3.0_f32.sqrt()/3.0, 3.0_f32.sqrt()/3.0, 3.0_f32.sqrt()/3.0));

        let expected = Vector4::vector(3.0_f32.sqrt() / 3.0, 3.0_f32.sqrt() / 3.0, 3.0_f32.sqrt() / 3.0);
        assert_that!(n.x).is_close_to(expected.x, 0.0001);
        assert_that!(n.y).is_close_to(expected.y, 0.0001);
        assert_that!(n.z).is_close_to(expected.z, 0.0001);
        assert_that!(n.w).is_equal_to(expected.w);
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere::default();

        let n = s.normal_at(&Vector4::point(3.0_f32.sqrt()/3.0, 3.0_f32.sqrt()/3.0, 3.0_f32.sqrt()/3.0));

        let normalized: Vector4<f32> = n.normalize();
        assert_that!(n.x).is_close_to(normalized.x, 0.0001);
        assert_that!(n.y).is_close_to(normalized.y, 0.0001);
        assert_that!(n.z).is_close_to(normalized.z, 0.0001);
        assert_that!(n.w).is_equal_to(normalized.w);
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = Sphere::default();
        s.set_transform(Matrix4::translation(0.0, 1.0, 0.0));

        let n = s.normal_at(&Vector4::point(0.0, 1.70711, -FRAC_1_SQRT_2));

        let expected = Vector4::vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
        assert_that!(n.x).is_close_to(expected.x, 0.0001);
        assert_that!(n.y).is_close_to(expected.y, 0.0001);
        assert_that!(n.z).is_close_to(expected.z, 0.0001);
        assert_that!(n.w).is_equal_to(expected.w);
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = Sphere::default();
        let m = Matrix4::scaling(1.0, 0.5, 1.0) * Matrix4::rotation_z(PI/5.0);
        s.set_transform(m);

        let n = s.normal_at(&Vector4::point(0.0, 2.0_f32.sqrt()/2.0, -2.0_f32.sqrt()/2.0));

        let expected = Vector4::vector(0.0, 0.97014, -0.24254);
        assert_that!(n.x).is_close_to(expected.x, 0.0001);
        assert_that!(n.y).is_close_to(expected.y, 0.0001);
        assert_that!(n.z).is_close_to(expected.z, 0.0001);
        assert_that!(n.w).is_equal_to(expected.w);
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = Sphere::default();
        let m = s.material;
        assert_that!(m).is_equal_to(Material::default());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut s = Sphere::default();
        let mut m = Material::default();
        m.ambient = 1.0;

        s.material = m.clone();

        assert_that!(s.material).is_equal_to(m);
    }
}
