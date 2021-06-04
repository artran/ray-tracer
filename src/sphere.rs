pub struct Sphere {

}

impl Sphere {
    fn new() -> Self {
        Self{}
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use nalgebra::Vector4;
    use spectral::assert_that;

    use crate::ray::Ray;
    use crate::tuple::Tuple;

    use super::*;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Vector4::point(0.0, 0.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_that!(xs.count).is_equal_to(2);
        assert_that!(xs[0]).is_equal_to(4.0);
        assert_that!(xs[1]).is_equal_to(6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Vector4::point(0.0, 1.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_that!(xs.count).is_equal_to(2);
        assert_that!(xs[0]).is_equal_to(5.0);
        assert_that!(xs[1]).is_equal_to(5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Vector4::point(0.0, 2.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_that!(xs.count).is_equal_to(0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Vector4::point(0.0, 0.0, 0.0), Vector4::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_that!(xs.count).is_equal_to(2);
        assert_that!(xs[0]).is_equal_to(-1.0);
        assert_that!(xs[1]).is_equal_to(1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Vector4::point(0.0, 0.0, 5.0), Vector4::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_that!(xs.count).is_equal_to(2);
        assert_that!(xs[0]).is_equal_to(-6.0);
        assert_that!(xs[1]).is_equal_to(-4.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Vector4::point(0.0, 0.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_that!(xs.count).is_equal_to(2);
        assert_that!(xs[0].object).is_equal_to(s);
        assert_that!(xs[1].object).is_equal_to(s);
    }
}
