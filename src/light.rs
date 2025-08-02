use crate::color::Color;
use crate::vector4::Vector4;

#[derive(Debug, PartialEq)]
pub struct PointLight {
    pub position: Vector4,
    pub intensity: Color,
}

impl PointLight {
    #[allow(dead_code)]
    pub fn new(position: Vector4, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

impl Default for PointLight {
    fn default() -> Self {
        Self {
            position: Vector4::point(-10.0, 10.0, -10.0),
            intensity: Color::white(),
        }
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use spectral::assert_that;

    use super::*;
    use crate::vector4::Vector4;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Vector4::point(0.0, 0.0, 0.0);

        let light = PointLight::new(position, intensity);

        assert_that!(light.position).is_equal_to(position);
        assert_that!(light.intensity).is_equal_to(intensity);
    }
}
