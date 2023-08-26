use std::boxed::Box;

use crate::color::Color;
use crate::light::PointLight;
use crate::pattern::{Pattern, SolidPattern};
use crate::vector4::Vector4;

#[derive(Debug, PartialEq)]
pub struct Material {
    pattern: Box<dyn Pattern>,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
}

pub struct MaterialBuilder {
    pattern: Option<Box<dyn Pattern>>,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
}

impl Material {
    pub(crate) fn lighting(
        &self,
        light: &PointLight,
        point: Vector4,
        eye_vector: Vector4,
        normal_vector: Vector4,
        in_shadow: bool,
    ) -> Color {
        let effective_color = self.pattern.color_at_point(point) * light.intensity;

        let ambient = effective_color * self.ambient;

        // Return early if the point is in shadow
        if in_shadow {
            return ambient;
        }

        let mut diffuse = Color::black();
        let mut specular = Color::black();

        let light_vector = (light.position - point).normalize();
        let light_dot_normal = light_vector.dot(&normal_vector);
        if light_dot_normal >= 0.0 {
            diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflect_vector = (-light_vector).reflect(&normal_vector);
            let reflect_dot_eye = reflect_vector.dot(&eye_vector);
            if reflect_dot_eye > 0.0 {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

impl MaterialBuilder {
    pub fn new() -> Self {
        Self {
            pattern: None,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        let pattern = SolidPattern { color };
        self.pattern = Some(Box::new(pattern));

        self
    }

    pub fn with_pattern(mut self, pattern: Box<impl Pattern>) -> Self {
        self.pattern = Some(pattern);

        self
    }

    pub fn with_ambient(mut self, ambient: f32) -> Self {
        self.ambient = ambient;

        self
    }

    pub fn with_diffuse(mut self, diffuse: f32) -> Self {
        self.diffuse = diffuse;

        self
    }

    pub fn with_specular(mut self, specular: f32) -> Self {
        self.specular = specular;

        self
    }

    pub fn with_shininess(mut self, shininess: f32) -> Self {
        self.shininess = shininess;

        self
    }

    pub fn build(self) -> Material {
        let pattern = match self.pattern {
            Some(p) => p,
            None => Box::new(SolidPattern::default()),
        };
        Material {
            pattern,
            ambient: self.ambient,
            diffuse: self.diffuse,
            specular: self.specular,
            shininess: self.shininess,
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

    use crate::pattern::StripePattern;

    use super::*;

    #[fixture]
    fn default_material() -> Material {
        MaterialBuilder::new().build()
    }

    #[fixture]
    fn default_position() -> Vector4 {
        Vector4::point(0.0, 0.0, 0.0)
    }

    #[rstest]
    fn the_default_material(default_material: Material) {
        // assert_that!(default_material.color).is_equal_to(Color::white());
        assert_that!(default_material.ambient).is_equal_to(0.1);
        assert_that!(default_material.diffuse).is_equal_to(0.9);
        assert_that!(default_material.specular).is_equal_to(0.9);
        assert_that!(default_material.shininess).is_equal_to(200.0);
    }

    #[rstest]
    fn lighting_with_the_eye_between_the_light_and_the_surface(
        default_material: Material,
        default_position: Vector4,
    ) {
        let eye_vector = Vector4::vector(0.0, 0.0, -1.0);
        let normal_vector = Vector4::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Vector4::point(0.0, 0.0, -10.0), Color::white());

        let result =
            default_material.lighting(&light, default_position, eye_vector, normal_vector, false);

        assert_that!(result).is_equal_to(Color::new(1.9, 1.9, 1.9));
    }

    #[rstest]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45_deg(
        default_material: Material,
        default_position: Vector4,
    ) {
        let eye_vector = Vector4::vector(0.0, 2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0);
        let normal_vector = Vector4::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Vector4::point(0.0, 0.0, -10.0), Color::white());

        let result =
            default_material.lighting(&light, default_position, eye_vector, normal_vector, false);

        assert_that!(result).is_equal_to(Color::white());
    }

    #[rstest]
    fn lighting_with_eye_opposite_surface_light_offset_45_deg(
        default_material: Material,
        default_position: Vector4,
    ) {
        let eye_vector = Vector4::vector(0.0, 0.0, -1.0);
        let normal_vector = Vector4::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Vector4::point(0.0, 10.0, -10.0), Color::white());

        let result =
            default_material.lighting(&light, default_position, eye_vector, normal_vector, false);

        let expected = Color::new(0.7364, 0.7364, 0.7364);
        assert_that!(result.r).is_close_to(expected.r, 0.0001);
        assert_that!(result.g).is_close_to(expected.g, 0.0001);
        assert_that!(result.b).is_close_to(expected.b, 0.0001);
    }

    #[rstest]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector(
        default_material: Material,
        default_position: Vector4,
    ) {
        let eye_vector = Vector4::vector(0.0, -2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0);
        let normal_vector = Vector4::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Vector4::point(0.0, 10.0, -10.0), Color::white());

        let result =
            default_material.lighting(&light, default_position, eye_vector, normal_vector, false);

        let expected = Color::new(1.6364, 1.6364, 1.6364);
        assert_that!(result.r).is_close_to(expected.r, 0.0001);
        assert_that!(result.g).is_close_to(expected.g, 0.0001);
        assert_that!(result.b).is_close_to(expected.b, 0.0001);
    }

    #[rstest]
    fn lighting_with_the_light_behind_the_surface(
        default_material: Material,
        default_position: Vector4,
    ) {
        let eye_vector = Vector4::vector(0.0, 0.0, -1.0);
        let normal_vector = Vector4::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Vector4::point(0.0, 0.0, 10.0), Color::white());

        let result =
            default_material.lighting(&light, default_position, eye_vector, normal_vector, false);

        assert_that!(result).is_equal_to(Color::new(0.1, 0.1, 0.1));
    }

    #[rstest]
    fn lighting_with_the_surface_in_shadow(default_material: Material, default_position: Vector4) {
        let eye_vec = Vector4::vector(0.0, 0.0, -1.0);
        let normal_vec = Vector4::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Vector4::point(0.0, 0.0, -10.0), Color::white());
        let in_shadow = true;

        let result =
            default_material.lighting(&light, default_position, eye_vec, normal_vec, in_shadow);

        assert_that!(result).is_equal_to(Color::new(0.1, 0.1, 0.1));
    }

    #[rstest]
    fn lighting_with_a_pattern_appplied() {
        let p = StripePattern {
            color1: Color::white(),
            color2: Color::black(),
        };
        let m = MaterialBuilder::new()
            .with_ambient(1.0)
            .with_diffuse(0.0)
            .with_specular(0.0)
            .with_pattern(Box::new(p))
            .build();
    }
}
