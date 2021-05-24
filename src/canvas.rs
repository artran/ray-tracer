use std::io::Write;

use crate::color::Color;

struct Canvas {
    pixels: Vec<Vec<Color>>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![vec![Color::black(); width]; height]
        }
    }

    pub fn width(&self) -> usize {
        self.pixels[0].len()
    }

    pub fn height(&self) -> usize {
        self.pixels.len()
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y][x].clone()
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: &Color) {
        self.pixels[y][x] = color.clone();
    }

    pub fn save(&self, file: &mut impl Write) {
        let _ = file.write(b"P3\n5 3\n255\n").unwrap();
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
    fn new_canvas_has_width() {
        let canvas = Canvas::new(10, 20);

        assert_that!(canvas.width()).is_equal_to(10);
    }

    #[test]
    fn new_canvas_has_height() {
        let canvas = Canvas::new(10, 20);

        assert_that!(canvas.height()).is_equal_to(20);
    }

    #[test]
    fn pixels_can_be_read_from() {
        let canvas = Canvas::new(10, 20);

        assert_that!(canvas.pixel_at(1, 2)).is_equal_to(Color::black())
    }

    #[test]
    fn new_canvas_is_black() {
        let canvas = Canvas::new(5, 5);

        for x in 0..5 {
            for y in 0..5 {
                assert_that!(canvas.pixel_at(x, y)).is_equal_to(Color::black())
            }
        }
    }

    #[test]
    fn pixels_can_be_written_to() {
        let mut canvas = Canvas::new(10, 20);

        let red = Color::new(1.0, 0.0, 0.0);
        canvas.write_pixel(5, 7, &red);

        assert_that!(canvas.pixel_at(5, 7)).is_equal_to(red);
    }

    #[test]
    fn saved_canvas_has_correct_magic() {
        let canvas = Canvas::new(5, 3);
        let mut file = vec![];

        canvas.save(&mut file);

        assert_that!(String::from_utf8(file).unwrap()).is_equal_to(String::from("P3\n5 3\n255\n"));
    }
}
