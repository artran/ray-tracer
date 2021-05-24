use std::fmt::{Formatter, Display, Error};

#[derive(Clone, Debug, PartialEq)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn plus(&self, other: &Color) -> Color {
        Color::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }

    pub fn minus(&self, other: &Color) -> Color {
        Color::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }

    pub fn scale(&self, factor: &f32) -> Color {
        Color::new(self.r * factor, self.g * factor, self.b * factor)
    }

    pub fn hadamard(&self, other: &Color) -> Color {
        Color::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let red = (self.r.clamp(0.0, 1.0) * 255.0).round() as u8;
        let green = (self.g.clamp(0.0, 1.0) * 255.0).round() as u8;
        let blue = (self.b.clamp(0.0, 1.0) * 255.0).round() as u8;
        write!(f, "{} {} {}", red, green, blue)
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use spectral::assert_that;
    use spectral::numeric::FloatAssertions;
    use crate::color::Color;

    #[test]
    fn adding_colours() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let expected = Color::new(1.6, 0.7, 1.0);

        let result = c1.plus(&c2);

        assert_that!(result.r).is_close_to(expected.r, 0.0001_f32);
        assert_that!(result.g).is_close_to(expected.g, 0.0001_f32);
        assert_that!(result.b).is_close_to(expected.b, 0.0001_f32);
    }

    #[test]
    fn subtracting_colours() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let expected = Color::new(0.2, 0.5, 0.5);

        let result = c1.minus(&c2);

        assert_that!(result.r).is_close_to(expected.r, 0.0001_f32);
        assert_that!(result.g).is_close_to(expected.g, 0.0001_f32);
        assert_that!(result.b).is_close_to(expected.b, 0.0001_f32);
    }

    #[test]
    fn scaling_colours() {
        let c = Color::new(0.2, 0.3, 0.4);
        let expected = Color::new(0.4, 0.6, 0.8);

        assert_that!(c.scale(&2.0)).is_equal_to(expected);
    }

    #[test]
    fn multiplying_colours() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let expected = Color::new(0.9, 0.2, 0.04);

        let result = c1.hadamard(&c2);

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
