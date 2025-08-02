pub mod builder;

use std::rc::Rc;

use crate::color::Color;
use crate::light::PointLight;
use crate::pattern::Pattern;
use crate::pattern::solid::SolidPattern;
use crate::vector4::Vector4;

#[derive(Clone, Debug)]
pub struct Material {
    pattern: Rc<dyn Pattern>,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.ambient == other.ambient
            && self.diffuse == other.diffuse
            && self.specular == other.specular
            && self.shininess == other.shininess
            && *self.pattern == *other.pattern
    }
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

impl Default for Material {
    fn default() -> Self {
        Self {
            pattern: Rc::new(SolidPattern::new(Color::white())),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
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

    use crate::material::builder::MaterialBuilder;
    use crate::pattern::stripes::StripePattern;

    use super::*;

    #[fixture]
    fn default_material() -> Material {
        Material::default()
    }

    #[fixture]
    fn striped_material() -> Material {
        let stripes = StripePattern::new(Color::white(), Color::black());
        MaterialBuilder::new()
            .with_pattern(Rc::new(stripes))
            .with_ambient(1.0)
            .with_diffuse(0.0)
            .with_specular(0.0)
            .build()
    }

    #[fixture]
    fn default_position() -> Vector4 {
        Vector4::point(0.0, 0.0, 0.0)
    }

    #[rstest]
    fn the_default_material(default_material: Material) {
        // assert_that!(default_material.pattern).is_equal_to(Color::white()); // FIXME:
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
        let eye_vector = Vector4::vector(0.0, 2.0_f32.sqrt() / 2.0, -(2.0_f32.sqrt()) / 2.0);
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
        let eye_vector = Vector4::vector(0.0, -(2.0_f32.sqrt()) / 2.0, -(2.0_f32.sqrt()) / 2.0);
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
    fn lighting_with_a_pattern_applied(striped_material: Material) {
        let eye_vector = Vector4::vector(0.0, 0.0, -1.0);
        let normal_vector = Vector4::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Vector4::point(0.0, 0.0, -10.0), Color::white());

        let point1 = Vector4::point(0.9, 0.0, 0.0);
        let point2 = Vector4::point(1.1, 0.0, 0.0);
        let c1 = striped_material.lighting(&light, point1, eye_vector, normal_vector, false);
        let c2 = striped_material.lighting(&light, point2, eye_vector, normal_vector, false);
        assert_that!(c1).is_equal_to(Color::white());
        assert_that!(c2).is_equal_to(Color::black());
    }
}
