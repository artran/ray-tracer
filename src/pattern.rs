use std::any::Any;
use std::fmt::Debug;

use crate::color::Color;
use crate::vector4::Vector4;

pub trait Pattern: Debug {
    fn as_any(&self) -> &dyn Any;
    fn pattern_eq(&self, other: &dyn Pattern) -> bool;
    fn color_at_point(&self, point: Vector4) -> Color;
}

impl PartialEq for dyn Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.pattern_eq(other)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StripePattern {
    color1: Color,
    color2: Color,
}

impl Pattern for StripePattern {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn pattern_eq(&self, other: &dyn Pattern) -> bool {
        other.as_any().downcast_ref::<Self>() == Some(self)
    }

    fn color_at_point(&self, point: Vector4) -> Color {
        if point.x.floor() as isize % 2 == 0 {
            return self.color1;
        }
        self.color2
    }
}

impl StripePattern {
    #[allow(dead_code)]
    pub fn new(color1: Color, color2: Color) -> Self {
        Self { color1, color2 }
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
    fn stripe_patterns_have_two_colors() {
        let p = StripePattern {
            color1: Color::white(),
            color2: Color::black(),
        };

        assert_that!(p.color1).is_equal_to(Color::white());
        assert_that!(p.color2).is_equal_to(Color::black());
    }

    #[test]
    fn stripe_patterns_are_constant_in_y() {
        let p = StripePattern {
            color1: Color::white(),
            color2: Color::black(),
        };

        assert_that!(p.color_at_point(Vector4::point(0.0, 0.0, 0.0))).is_equal_to(Color::white());
        assert_that!(p.color_at_point(Vector4::point(0.0, 1.0, 0.0))).is_equal_to(Color::white());
        assert_that!(p.color_at_point(Vector4::point(0.0, 2.0, 0.0))).is_equal_to(Color::white());
    }

    #[test]
    fn stripe_patterns_are_constant_in_z() {
        let p = StripePattern {
            color1: Color::white(),
            color2: Color::black(),
        };

        assert_that!(p.color_at_point(Vector4::point(0.0, 0.0, 0.0))).is_equal_to(Color::white());
        assert_that!(p.color_at_point(Vector4::point(0.0, 0.0, 1.0))).is_equal_to(Color::white());
        assert_that!(p.color_at_point(Vector4::point(0.0, 0.0, 2.0))).is_equal_to(Color::white());
    }

    #[test]
    fn stripe_patterns_are_alternate_in_x() {
        let p = StripePattern {
            color1: Color::white(),
            color2: Color::black(),
        };

        assert_that!(p.color_at_point(Vector4::point(0.0, 0.0, 0.0))).is_equal_to(Color::white());
        assert_that!(p.color_at_point(Vector4::point(0.9, 0.0, 0.0))).is_equal_to(Color::white());
        assert_that!(p.color_at_point(Vector4::point(1.0, 0.0, 0.0))).is_equal_to(Color::black());
        assert_that!(p.color_at_point(Vector4::point(-0.1, 0.0, 0.0))).is_equal_to(Color::black());
        assert_that!(p.color_at_point(Vector4::point(-1.0, 0.0, 0.0))).is_equal_to(Color::black());
        assert_that!(p.color_at_point(Vector4::point(-1.1, 0.0, 0.0))).is_equal_to(Color::white());
    }
}
