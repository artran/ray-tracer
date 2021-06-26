use nalgebra::Vector4;

use crate::color::Color;
use crate::light::PointLight;
use crate::tuple::Tuple;

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Material {
    pub fn lighting(&self, light: PointLight, point: Vector4<f32>, eye_vector: Vector4<f32>, normal_vector: Vector4<f32>) -> Color {
        let effective_color = self.color * light.intensity;

        let ambient = effective_color * self.ambient;
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
            color: Color::new(1.0, 1.0, 1.0),
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
    use spectral::assert_that;
    use spectral::numeric::FloatAssertions;

    use super::*;

    #[test]
    fn the_default_material() {
        let m = Material::default();

        assert_that!(m.color).is_equal_to(Color::new(1.0, 1.0, 1.0));
        assert_that!(m.ambient).is_equal_to(0.1);
        assert_that!(m.diffuse).is_equal_to(0.9);
        assert_that!(m.specular).is_equal_to(0.9);
        assert_that!(m.shininess).is_equal_to(200.0);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::default();
        let position = Vector4::point(0.0, 0.0, 0.0);
        let eye_vector = Vector4::vector(0.0, 0.0, -1.0);
        let normal_vector = Vector4::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Vector4::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(light, position, eye_vector, normal_vector);

        assert_that!(result).is_equal_to(Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45_deg() {
        let m = Material::default();
        let position = Vector4::point(0.0, 0.0, 0.0);
        let eye_vector = Vector4::vector(0.0, 2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0);
        let normal_vector = Vector4::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Vector4::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(light, position, eye_vector, normal_vector);

        assert_that!(result).is_equal_to(Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_deg() {
        let m = Material::default();
        let position = Vector4::point(0.0, 0.0, 0.0);
        let eye_vector = Vector4::vector(0.0, 0.0, -1.0);
        let normal_vector = Vector4::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Vector4::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(light, position, eye_vector, normal_vector);

        let expected = Color::new(0.7364, 0.7364, 0.7364);
        assert_that!(result.r).is_close_to(expected.r, 0.0001);
        assert_that!(result.g).is_close_to(expected.g, 0.0001);
        assert_that!(result.b).is_close_to(expected.b, 0.0001);
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::default();
        let position = Vector4::point(0.0, 0.0, 0.0);
        let eye_vector = Vector4::vector(0.0, -2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0);
        let normal_vector = Vector4::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Vector4::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(light, position, eye_vector, normal_vector);

        let expected = Color::new(1.6364, 1.6364, 1.6364);
        assert_that!(result.r).is_close_to(expected.r, 0.0001);
        assert_that!(result.g).is_close_to(expected.g, 0.0001);
        assert_that!(result.b).is_close_to(expected.b, 0.0001);
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::default();
        let position = Vector4::point(0.0, 0.0, 0.0);
        let eye_vector = Vector4::vector(0.0, 0.0, -1.0);
        let normal_vector = Vector4::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Vector4::point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(light, position, eye_vector, normal_vector);

        assert_that!(result).is_equal_to(Color::new(0.1, 0.1, 0.1));
    }
}
