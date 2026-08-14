use std::rc::Rc;

use crate::shape::{
    Shape,
    plane::{Plane, PlaneBuilder},
    sphere::{Sphere, SphereBuilder},
};

/// Wrap any concrete `impl Shape` into a ref-counted trait object.
///
/// The `Rc<Concrete> → Rc<dyn Shape>` unsized coercion happens at the
/// return type (Lesson 0002: return type as coercion site).
pub fn shape_rc(s: impl Shape + 'static) -> Rc<dyn Shape> {
    Rc::new(s)
}

/// Coerce a dyn Shape to the concrete class and return it.
/// Panics on a type mismatch; this is acceptable because the helper is
/// `#[cfg(test)]`-only, and clippy suppresses `unwrap_used` in test code.
///
/// See lesson 0004: downcasting.
pub fn expect_sphere(shape: &dyn Shape) -> &Sphere {
    shape.as_any().downcast_ref::<Sphere>().unwrap()
}

/// Coerce a dyn Shape to the concrete class and return it.
/// Panics on a type mismatch; this is acceptable because the helper is
/// `#[cfg(test)]`-only, and clippy suppresses `unwrap_used` in test code.
///
/// See lesson 0004: downcasting.
pub fn expect_plane(shape: &dyn Shape) -> &Plane {
    shape.as_any().downcast_ref::<Plane>().unwrap()
}

/// Default sphere wrapped as a ref-counted trait object.
///
/// Convenience over `shape_rc(SphereBuilder::new().build())`;
/// coercion happens in `shape_rc`'s return (Lesson 0002).
pub fn default_sphere_rc() -> Rc<dyn Shape> {
    shape_rc(SphereBuilder::new().build())
}

/// Default plane wrapped as a ref-counted trait object.
///
/// Convenience over `shape_rc(PlaneBuilder::new().build())`;
/// coercion happens in `shape_rc`'s return (Lesson 0002).
pub fn default_plane_rc() -> Rc<dyn Shape> {
    shape_rc(PlaneBuilder::new().build())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use spectral::{assert_that, boolean::BooleanAssertions};

    use super::*;
    use crate::{
        material::Material,
        shape::{plane::PlaneBuilder, sphere::SphereBuilder},
    };

    #[test]
    fn shape_rc_coerces_a_concrete_sphere() {
        let s = shape_rc(SphereBuilder::new().build());
        assert_that!(Rc::strong_count(&s)).is_equal_to(1);
    }

    #[test]
    fn expect_sphere_passes_sphere() {
        let sphere = SphereBuilder::new().build();
        let expected_material = Material::default();

        let downcast_sphere = expect_sphere(&sphere);
        assert_that!(downcast_sphere.material()).is_equal_to(&expected_material);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn expect_sphere_fails_plane() {
        let plane = PlaneBuilder::new().build();

        let _definitely_not_sphere = expect_sphere(&plane);
    }

    #[test]
    fn expect_plane_passes_plane() {
        let plane = PlaneBuilder::new().build();
        let expected_material = Material::default();

        let downcast_plane = expect_plane(&plane);
        assert_that!(downcast_plane.material()).is_equal_to(&expected_material);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn expect_plane_fails_sphere() {
        let plane = SphereBuilder::new().build();

        let _definitely_not_plane = expect_plane(&plane);
    }

    #[test]
    fn default_sphere_rc_has_refcount_one() {
        let sphere_rc = default_sphere_rc();
        assert_that!(Rc::strong_count(&sphere_rc)).is_equal_to(1);
    }

    #[test]
    fn default_sphere_rc_contains_sphere() {
        let sphere_rc = default_sphere_rc();
        assert_that!(sphere_rc.as_any().is::<Sphere>()).is_true();
    }

    #[test]
    fn default_plane_rc_has_refcount_one() {
        let plane_rc = default_plane_rc();
        assert_that!(Rc::strong_count(&plane_rc)).is_equal_to(1);
    }

    #[test]
    fn default_plane_rc_contains_plane() {
        let plane_rc = default_plane_rc();
        assert_that!(plane_rc.as_any().is::<Plane>()).is_true();
    }
}
