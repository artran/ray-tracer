use std::fmt::Debug;

use crate::color::Color;
use crate::light::PointLight;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::vector4::Vector4;

pub trait Shape {
    fn material(&self) -> &Material;
    fn transformation(&self) -> Matrix<4>;

    fn intersect(&self, ray: &Ray) -> Vec<f32>;
    fn normal_at(&self, world_point: &Vector4) -> Vector4;
    fn lighting(
        &self,
        light: &PointLight,
        point: Vector4,
        eye_vector: Vector4,
        normal_vector: Vector4,
        in_shadow: bool,
    ) -> Color;
}

impl PartialEq for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.material() == other.material() && self.transformation() == other.transformation()
    }
}

impl Debug for dyn Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Shape")
            .field("material", &self.material())
            .field("transform", &self.transformation())
            .finish()
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
    use crate::material::MaterialBuilder;
    use crate::sphere::{Sphere, SphereBuilder};
    use crate::transform::Transform;

    #[fixture]
    fn test_shape() -> Sphere {
        SphereBuilder::new().build()
    }

    #[rstest]
    fn the_default_transformation(test_shape: impl Shape) {
        assert_that!(test_shape.transformation()).is_equal_to(&Matrix::identity());
    }

    #[rstest]
    fn assigning_a_transformation() {
        let test_shape = SphereBuilder::new()
            .with_transform(Matrix::translation(2.0, 3.0, 4.0))
            .build();
        assert_that!(test_shape.transformation()).is_equal_to(&Matrix::translation(2.0, 3.0, 4.0));
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
}
