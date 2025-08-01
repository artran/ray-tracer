use std::any::Any;
use std::fmt::{Display, Error, Formatter};
use std::ops::{Add, Mul, Sub};

use crate::pattern::Pattern;
use crate::vector4::Vector4;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
}

impl Pattern for Color {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn pattern_eq(&self, other: &dyn Pattern) -> bool {
        other.as_any().downcast_ref::<Self>() == Some(self)
    }

    fn color_at_point(&self, _: Vector4) -> Color {
        *self
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Self) -> Self::Output {
        Color::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Self) -> Self::Output {
        Color::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Self) -> Self::Output {
        Color::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, factor: f32) -> Self::Output {
        Color::new(self.r * factor, self.g * factor, self.b * factor)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let red = (self.r.clamp(0.0, 1.0) * 255.0).round() as u8;
        let green = (self.g.clamp(0.0, 1.0) * 255.0).round() as u8;
        let blue = (self.b.clamp(0.0, 1.0) * 255.0).round() as u8;
        write!(f, "{red} {green} {blue}")
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use spectral::assert_that;
    use spectral::numeric::FloatAssertions;

    #[test]
    fn adding_colours() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let expected = Color::new(1.6, 0.7, 1.0);

        let result = c1 + c2;

        assert_that!(result.r).is_close_to(expected.r, 0.0001_f32);
        assert_that!(result.g).is_close_to(expected.g, 0.0001_f32);
        assert_that!(result.b).is_close_to(expected.b, 0.0001_f32);
    }

    #[test]
    fn subtracting_colours() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let expected = Color::new(0.2, 0.5, 0.5);

        let result = c1 - c2;

        assert_that!(result.r).is_close_to(expected.r, 0.0001_f32);
        assert_that!(result.g).is_close_to(expected.g, 0.0001_f32);
        assert_that!(result.b).is_close_to(expected.b, 0.0001_f32);
    }

    #[test]
    fn scaling_colours() {
        let c = Color::new(0.2, 0.3, 0.4);
        let expected = Color::new(0.4, 0.6, 0.8);

        assert_that!(c * 2.0).is_equal_to(expected);
    }

    #[test]
    fn multiplying_colours() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let expected = Color::new(0.9, 0.2, 0.04);

        let result = c1 * c2;

        assert_that!(result.r).is_close_to(expected.r, 0.0001_f32);
        assert_that!(result.g).is_close_to(expected.g, 0.0001_f32);
        assert_that!(result.b).is_close_to(expected.b, 0.0001_f32);
    }

    #[test]
    fn displaying_colours() {
        let c1 = Color::new(1.0, 0.5, 0.25);

        assert_that(&c1.to_string()).is_equal_to(String::from("255 128 64"));
    }

    #[test]
    fn displaying_colours_clamps_to_zero_or_255() {
        let c1 = Color::new(1.5, -0.5, 0.5);

        assert_that(&c1.to_string()).is_equal_to(String::from("255 0 128"));
    }
}
