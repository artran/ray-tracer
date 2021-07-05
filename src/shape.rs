use nalgebra::Matrix4;
use crate::material::Material;

pub trait Shape {
    fn set_transform(&mut self, transform: Matrix4<f32>);
    fn set_material(&mut self, material: Material);
    fn get_transform(&self) -> Matrix4<f32>;
    fn get_material(&self) -> Material;
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    // use rstest::*;
    // use spectral::prelude::*;
    //
    // use super::*;
    // use crate::sphere::{Sphere, SphereBuilder};
    // use crate::transform::Transform;
    // use crate::material::MaterialBuilder;
    //
    // #[fixture]
    // fn test_shape() -> Sphere {
    //     SphereBuilder::new().build()
    // }
    //
    // #[rstest]
    // fn the_default_transformation(test_shape: impl Shape) {
    //     assert_that!(test_shape.get_transform()).is_equal_to(Matrix4::identity());
    // }
    //
    // #[rstest]
    // fn assigning_a_transformation(mut test_shape: impl Shape) {
    //     test_shape.set_transform(Matrix4::translation(2.0, 3.0, 4.0));
    //     assert_that!(test_shape.get_transform()).is_equal_to(Matrix4::translation(2.0, 3.0, 4.0));
    // }
    //
    // #[rstest]
    // fn the_default_material(test_shape: impl Shape) {
    //     assert_that!(test_shape.get_material()).is_equal_to(MaterialBuilder::new().build());
    // }
    //
    // #[rstest]
    // fn assigning_a_material(mut test_shape: impl Shape) {
    //     let m = MaterialBuilder::new()
    //         .with_ambient(1.0)
    //         .build();
    //
    //     test_shape.set_material(m.clone());
    //
    //     assert_that!(test_shape.get_material()).is_equal_to(m);
    // }
}
