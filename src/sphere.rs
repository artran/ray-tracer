use nalgebra::{Vector4, Matrix4};

use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::tuple::Tuple;

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    // Note: we store the inverse of the transform as an optimisation.
    inv_transform: Matrix4<f32>
}

impl Sphere {
    pub fn new() -> Self {
        Self{
            // The inverse of the identity is the identity
            inv_transform: Matrix4::identity()
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
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

        Some(vec!(Intersection::new(t1, self), Intersection::new(t2, self)))
    }

    pub fn set_transform(&mut self, transform: Matrix4<f32>) {
        // As an optimisation we invert the transform before storing it.
        self.inv_transform = transform.try_inverse().unwrap();
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use spectral::assert_that;
    use spectral::option::OptionAssertions;

    use super::*;
    use crate::transform::Transform;
    use nalgebra::Matrix4;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Vector4::point(0.0, 0.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r).unwrap();

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0].t).is_equal_to(4.0);
        assert_that!(xs[1].t).is_equal_to(6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Vector4::point(0.0, 1.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r).unwrap();

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0].t).is_equal_to(5.0);
        assert_that!(xs[1].t).is_equal_to(5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Vector4::point(0.0, 2.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_that!(xs).is_none();
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Vector4::point(0.0, 0.0, 0.0), Vector4::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r).unwrap();

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0].t).is_equal_to(-1.0);
        assert_that!(xs[1].t).is_equal_to(1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Vector4::point(0.0, 0.0, 5.0), Vector4::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r).unwrap();

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0].t).is_equal_to(-6.0);
        assert_that!(xs[1].t).is_equal_to(-4.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Vector4::point(0.0, 0.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r).unwrap();

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0].object).is_equal_to(&s);
        assert_that!(xs[1].object).is_equal_to(&s);
    }

    #[test]
    fn a_spheres_default_transformation() {
        let s = Sphere::new();

        assert_that!(s.inv_transform).is_equal_to(Matrix4::identity());
    }

    #[test]
    fn changing_a_spheres_transformation() {
        let mut s = Sphere::new();
        let t = Matrix4::translation(2.0, 3.0, 4.0);

        s.inv_transform = t;

        assert_that!(s.inv_transform).is_equal_to(t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Vector4::point(0.0, 0.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(Matrix4::scaling(2.0, 2.0, 2.0));

        let xs = s.intersect(&r).unwrap();

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0].t).is_equal_to(3.0);
        assert_that!(xs[1].t).is_equal_to(7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Vector4::point(0.0, 0.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(Matrix4::translation(5.0, 0.0, 0.0));

        let xs = s.intersect(&r);

        assert_that!(xs).is_none();
    }
}
