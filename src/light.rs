use nalgebra::Vector4;
use crate::color::Color;

#[derive(Debug, PartialEq)]
pub struct PointLight {
    pub position: Vector4<f32>,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Vector4<f32>, intensity: Color) -> Self {
        Self {
            position,
            intensity,
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
    use crate::tuple::Tuple;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Vector4::point(0.0, 0.0, 0.0);

        let light = PointLight::new(position.clone(), intensity.clone());

        assert_that!(light.position).is_equal_to(position);
        assert_that!(light.intensity).is_equal_to(intensity);
    }
}
