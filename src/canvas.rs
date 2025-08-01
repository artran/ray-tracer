use std::io::{Error, Write};

use crate::color::Color;

const PPM_MAX_LINE_LENGTH: usize = 70;

pub struct Canvas {
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![vec![Color::black(); width]; height],
        }
    }

    pub fn width(&self) -> usize {
        self.pixels[0].len()
    }

    pub fn height(&self) -> usize {
        self.pixels.len()
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y][x]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: &Color) {
        self.pixels[y][x] = *color;
    }

    pub fn save(&self, file: &mut impl Write) -> Result<(), Error> {
        let header = format!(
            "P3\n{width} {height}\n255\n",
            width = self.width(),
            height = self.height()
        );
        let _ = file.write(header.as_bytes()).unwrap();
        for row in &self.pixels {
            let mut current_length = 0;
            for (i, pixel) in row.iter().enumerate() {
                if i > 0 {
                    file.write_all(b" ")?;
                    current_length += 1;
                }
                let pixel_str = pixel.to_string();
                let pixel_bytes = pixel_str.as_bytes();
                if current_length + pixel_bytes.len() > PPM_MAX_LINE_LENGTH {
                    file.write_all(b"\n")?;
                    current_length = 1;
                }
                file.write_all(pixel_bytes)?;
                current_length += pixel_bytes.len();
            }
            file.write_all(b"\n")?;
        }

        Ok(())
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use std::io::BufRead;

    use spectral::assert_that;
    use spectral::prelude::ResultAssertions;

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
    }

    #[test]
    fn saved_canvas_has_correct_width_and_height() {
        let canvas = Canvas::new(10, 15);
        let mut file = vec![];

        let _ = canvas.save(&mut file);

        let mut readable = &file[..];
        let mut buf = String::new();
        let _ = readable.read_line(&mut buf);
        assert_that!(buf).is_equal_to(String::from("P3\n"));
        buf.clear();
        let _ = readable.read_line(&mut buf);
        assert_that!(buf).is_equal_to(String::from("10 15\n"));
        buf.clear();
        let _ = readable.read_line(&mut buf);
        assert_that!(buf).is_equal_to(String::from("255\n"));
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
            let _ = readable.read_line(&mut buf); // Discard header lines
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
        buf.clear();
        let res = readable.read_line(&mut buf);
        assert_that!(res).is_ok().is_equal_to(0);
    }

    #[test]
    fn saved_canvas_has_correct_lines_less_than_70_chars() {
        let mut canvas = Canvas::new(10, 2);
        let c1 = Color::new(1.0, 0.8, 0.6);
        for row in 0..2 {
            for col in 0..10 {
                canvas.write_pixel(col, row, &c1);
            }
        }
        let mut file = vec![];

        let _ = canvas.save(&mut file);

        let mut readable = &file[..];
        let mut buf = String::new();
        for _ in 0..3 {
            let _ = readable.read_line(&mut buf); // Discard header lines
            buf.clear();
        }
        let _ = readable.read_line(&mut buf);
        assert_that!(buf).is_equal_to(String::from(
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 \n",
        ));
        buf.clear();
        let _ = readable.read_line(&mut buf);
        assert_that!(buf).is_equal_to(String::from(
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153\n",
        ));
        buf.clear();
        let _ = readable.read_line(&mut buf);
        assert_that!(buf).is_equal_to(String::from(
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 \n",
        ));
        buf.clear();
        let _ = readable.read_line(&mut buf);
        assert_that!(buf).is_equal_to(String::from(
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153\n",
        ));
        buf.clear();
        let res = readable.read_line(&mut buf);
        assert_that!(res).is_ok().is_equal_to(0);
    }

    #[test]
    fn saved_canvas_ends_with_newline_character() {
        let mut canvas = Canvas::new(10, 2);
        let c1 = Color::new(1.0, 0.8, 0.6);
        for row in 0..2 {
            for col in 0..10 {
                canvas.write_pixel(col, row, &c1);
            }
        }
        let mut file = vec![];

        let _ = canvas.save(&mut file);

        assert_that!(file.last().unwrap()).is_equal_to(&10u8);
    }
}
