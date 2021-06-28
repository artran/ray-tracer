use std::fs::File;

use nalgebra::{Matrix4, Vector4};

use crate::canvas::Canvas;
use crate::color::Color;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::transform::Transform;
use crate::tuple::Tuple;
use crate::light::PointLight;

mod canvas;
mod color;
mod intersection;
mod light;
mod material;
mod matrix;
mod ray;
mod sphere;
mod transform;
mod tuple;
mod world;

fn main() -> Result<(), std::io::Error> {
    let ray_origin = Vector4::point(0.0, 0.0, -5.0);
    let wall_z: f32 = 10.0;
    let wall_size: f32 = 7.0;
    let canvas_pixels: usize = 500;
    let pixel_size = wall_size / canvas_pixels as f32;
    let half: f32 = wall_size / 2.0;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    let mut shape = Sphere::default();
    shape.material.color = Color::new(1.0, 0.2, 1.0);
    // shape.set_transform(Matrix4::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * Matrix4::scaling(0.5, 1.0, 1.0));

    let light = PointLight::new(Vector4::point(-10.0, 10.0, -10.0), Color::white());

    for y in 0..canvas_pixels - 1 {
        let world_y = half - pixel_size * y as f32;

        for x in 0..canvas_pixels - 1 {
            let world_x = -half + pixel_size * x as f32;

            let position = Vector4::point(world_x, world_y, wall_z);
            let direction = position - ray_origin;

            let r = Ray::new(ray_origin, direction.normalize());
            if let Some(xs) = shape.intersect(&r) {
                if let Some(hit) = xs.hit() {
                    let point = r.position(hit.t);
                    let normal_vector = hit.object.normal_at(&point);
                    let eye_vector = -r.direction;

                    let color = hit.object.material.lighting(&light, point, eye_vector, normal_vector);
                    canvas.write_pixel(x, y, &color);
                }
            }
        }
    }

    let mut file = File::create("/tmp/sphere.ppm").unwrap();
    canvas.save(&mut file)?;

    Ok(())
}
