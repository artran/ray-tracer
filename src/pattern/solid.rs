use std::any::Any;

use crate::Color;
use crate::Vector4;
use crate::pattern::Pattern;

#[derive(Clone, Debug, PartialEq)]
pub struct SolidPattern {
    color: Color,
}

impl Pattern for SolidPattern {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn pattern_eq(&self, other: &dyn Pattern) -> bool {
        other.as_any().downcast_ref::<Self>() == Some(self)
    }

    fn color_at_point(&self, _point: Vector4) -> Color {
        self.color
    }
}

impl SolidPattern {
    #[allow(dead_code)]
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use spectral::assert_that;

    use super::*;

    #[test]
    fn solid_patterns_have_one_colors() {
        let p = SolidPattern::new(Color::white());

        assert_that!(p.color).is_equal_to(Color::white());
    }

    #[test]
    fn solid_patterns_are_constant_in_x() {
        let p = SolidPattern::new(Color::white());

        assert_that!(p.color_at_point(Vector4::point(0.0, 0.0, 0.0))).is_equal_to(Color::white());
        assert_that!(p.color_at_point(Vector4::point(0.9, 0.0, 0.0))).is_equal_to(Color::white());
        assert_that!(p.color_at_point(Vector4::point(1.0, 0.0, 0.0))).is_equal_to(Color::white());
        assert_that!(p.color_at_point(Vector4::point(-0.1, 0.0, 0.0))).is_equal_to(Color::white());
        assert_that!(p.color_at_point(Vector4::point(-1.0, 0.0, 0.0))).is_equal_to(Color::white());
        assert_that!(p.color_at_point(Vector4::point(-1.1, 0.0, 0.0))).is_equal_to(Color::white());
    }

    #[test]
    fn solid_patterns_are_constant_in_y() {
        let p = SolidPattern::new(Color::white());

        assert_that!(p.color_at_point(Vector4::point(0.0, 0.0, 0.0))).is_equal_to(Color::white());
        assert_that!(p.color_at_point(Vector4::point(0.0, 1.0, 0.0))).is_equal_to(Color::white());
        assert_that!(p.color_at_point(Vector4::point(0.0, 2.0, 0.0))).is_equal_to(Color::white());
    }

    #[test]
    fn solid_patterns_are_constant_in_z() {
        let p = SolidPattern::new(Color::white());

        assert_that!(p.color_at_point(Vector4::point(0.0, 0.0, 0.0))).is_equal_to(Color::white());
        assert_that!(p.color_at_point(Vector4::point(0.0, 0.0, 1.0))).is_equal_to(Color::white());
        assert_that!(p.color_at_point(Vector4::point(0.0, 0.0, 2.0))).is_equal_to(Color::white());
    }
}
