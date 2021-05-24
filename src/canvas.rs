use std::io::{Write, Error};

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

    pub fn save(&self, file: &mut impl Write) -> Result<(), Error> {
        let _ = file.write(b"P3\n5 3\n255\n").unwrap();
        for row in &self.pixels {
            for (i, pixel) in row.iter().enumerate() {
                if i > 0 {
                    file.write(b" ")?;
                }
                file.write(pixel.to_string().as_bytes())?;
            }
            file.write(b"\n")?;
        }

        Ok(())
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use spectral::assert_that;

    use super::*;
    use std::io::BufRead;

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

        let _ = canvas.save(&mut file);

        let mut readable = &file[..];
        let mut buf = String::new();
        let _ = readable.read_line(&mut buf);
        assert_that!(buf).is_equal_to(String::from("P3\n"));
        buf.clear();
        let _ = readable.read_line(&mut buf);
        assert_that!(buf).is_equal_to(String::from("5 3\n"));
        buf.clear();
        let _ = readable.read_line(&mut buf);
        assert_that!(buf).is_equal_to(String::from("255\n"));
        buf.clear();
    }

    #[test]
    fn saved_canvas_has_correct_image_data() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        canvas.write_pixel(0, 0, &c1);
        canvas.write_pixel(2, 1, &c2);
        canvas.write_pixel(4, 2, &c3);
        let mut file = vec![];

        let _ = canvas.save(&mut file);

        let mut readable = &file[..];
        let mut buf = String::new();
        for _ in 0..3 {
            let _ = readable.read_line(&mut buf);  // Discard header lines
            buf.clear();
        }
        let _ = readable.read_line(&mut buf);
        assert_that!(buf).is_equal_to(String::from("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n"));
        buf.clear();
        let _ = readable.read_line(&mut buf);
        assert_that!(buf).is_equal_to(String::from("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n"));
        buf.clear();
        let _ = readable.read_line(&mut buf);
        assert_that!(buf).is_equal_to(String::from("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n"));
    }
}
