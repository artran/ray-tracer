pub mod plane;
pub mod sphere;

use std::fmt::Debug;

use ray_math::Matrix;
use ray_math::Vector4;
use ray_traits::AsAny;

use crate::color::Color;
use crate::light::PointLight;
use crate::material::Material;
use crate::ray::Ray;

pub trait Shape: AsAny + Debug {
    fn shape_eq(&self, other: &dyn Shape) -> bool;

    fn material(&self) -> &Material;

    #[allow(dead_code, reason = "Used in tests")]
    fn transformation(&self) -> Option<Matrix<4>>;
    fn inv_transform(&self) -> &Matrix<4>;

    fn intersect(&self, ray: &Ray) -> Vec<f32> {
        let transformed_ray = ray.transform(self.inv_transform());
        self.local_intersect(&transformed_ray)
    }
    fn local_intersect(&self, ray: &Ray) -> Vec<f32>;

    fn normal_at(&self, world_point: &Vector4) -> Vector4 {
        let object_point = *self.inv_transform() * *world_point;
        let local_normal = self.local_normal_at(object_point);
        let mut world_normal = self.inv_transform().transpose() * local_normal;
        world_normal.w = 0.0;

        (world_normal).normalize()
    }
    fn local_normal_at(&self, world_point: Vector4) -> Vector4;

    fn lighting(
        &self,
        light: &PointLight,
        point: Vector4,
        eye_vector: Vector4,
        normal_vector: Vector4,
        in_shadow: bool,
    ) -> Color {
        self.material()
            .lighting(light, point, eye_vector, normal_vector, in_shadow)
    }
}

impl PartialEq for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.shape_eq(other)
    }
}

/// Default implementation for `Shape::shape_eq`.
/// Reduces boilerplate in concrete shape implementations.
pub fn default_shape_eq<T: PartialEq + 'static>(this: &T, other: &dyn Shape) -> bool {
    other.as_any().downcast_ref::<T>() == Some(this)
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use rstest::*;
    use spectral::prelude::*;

    use super::*;
    use crate::material::builder::MaterialBuilder;
    use crate::shape::sphere::SphereBuilder;
    use crate::transform::Transform;

    #[fixture]
    fn test_shape() -> impl Shape {
        SphereBuilder::new().build()
    }

    #[rstest]
    fn the_default_transformation(test_shape: impl Shape) {
        assert_that!(test_shape.transformation()).is_equal_to(Some(Matrix::identity()));
    }

    #[rstest]
    fn assigning_a_transformation() {
        let test_shape = SphereBuilder::new()
            .with_transform(Matrix::translation(2.0, 3.0, 4.0))
            .unwrap()
            .build();
        assert_that!(test_shape.transformation())
            .is_equal_to(Some(Matrix::translation(2.0, 3.0, 4.0)));
    }

    #[rstest]
    fn the_default_material(test_shape: impl Shape) {
        assert_that!(test_shape.material()).is_equal_to(&MaterialBuilder::new().build());
    }

    #[rstest]
    fn assigning_a_material() {
        let m = MaterialBuilder::new().with_ambient(1.0).build();
        let test_shape = SphereBuilder::new().with_material(m.clone()).build();

        assert_that!(test_shape.material()).is_equal_to(&m);
    }

    #[rstest]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let r = Ray::new(
            Vector4::point(0.0, 0.0, -5.0),
            Vector4::vector(0.0, 0.0, 1.0),
        );
        let s = SphereBuilder::new()
            .with_transform(Matrix::scaling(2.0, 2.0, 2.0))
            .unwrap()
            .build();

        let xs = s.intersect(&r);

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0]).is_equal_to(3.0);
        assert_that!(xs[1]).is_equal_to(7.0);
    }
}
