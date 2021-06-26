use std::fs::File;

use nalgebra::{Matrix4, Vector4};

use crate::canvas::Canvas;
use crate::color::Color;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::transform::Transform;
use crate::tuple::Tuple;

mod canvas;
mod color;
mod intersection;
mod light;
mod matrix;
mod ray;
mod sphere;
mod transform;
mod tuple;

fn main() -> Result<(), std::io::Error> {
    let ray_origin = Vector4::point(0.0, 0.0, -5.0);
    let wall_z: f32 = 10.0;
    let wall_size: f32 = 7.0;
    let canvas_pixels: usize = 100;
    let pixel_size = wall_size / canvas_pixels as f32;
    let half: f32 = wall_size / 2.0;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let red = Color::new(1.0, 0.0, 0.0);
    let mut shape = Sphere::new();
    shape.set_transform(Matrix4::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * Matrix4::scaling(0.5, 1.0, 1.0));

    for y in 0..canvas_pixels - 1 {
        let world_y = half - pixel_size * y as f32;

        for x in 0..canvas_pixels - 1 {
            let world_x = -half + pixel_size * x as f32;

            let position = Vector4::point(world_x, world_y, wall_z);
            let direction = position - ray_origin;

            let r = Ray::new(ray_origin, direction.normalize());
            if let Some(xs) = shape.intersect(&r) {
                if let Some(_) = xs.hit() {
                    canvas.write_pixel(x, y, &red);
                }
            }
        }
    }

    let mut file = File::create("/tmp/sphere.ppm").unwrap();
    canvas.save(&mut file)?;

    Ok(())
}
