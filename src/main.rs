use std::f32::consts::PI;
use std::fs::File;
use std::rc::Rc;

use crate::camera::CameraBuilder;
use crate::color::Color;
use crate::material::builder::MaterialBuilder;
use crate::matrix::Matrix;
use crate::shape::plane::PlaneBuilder;
use crate::shape::sphere::SphereBuilder;
use crate::transform::Transform;
use crate::vector4::Vector4;
use crate::world::WorldBuilder;

mod camera;
mod canvas;
mod color;
mod consts;
mod intersection;
mod light;
mod material;
mod matrix;
mod pattern;
mod ray;
mod shape;
mod transform;
mod vector4;
mod world;

fn main() -> Result<(), std::io::Error> {
    let floor_material = MaterialBuilder::new()
        .with_color(Color::new(1.0, 0.9, 0.9))
        .with_specular(0.0)
        .build();

    let wall_material = MaterialBuilder::new()
        .with_color(Color::new(1.0, 0.5, 0.5))
        .with_specular(0.0)
        .build();

    let floor = PlaneBuilder::new()
        // .with_transform(Matrix::scaling(10.0, 0.01, 10.0))
        .with_material(floor_material.clone())
        .build();

    let rear_wall = PlaneBuilder::new()
        .with_transform(Matrix::translation(0.0, 0.0, 3.0) * Matrix::rotation_x(PI / 2.0))
        .with_material(wall_material.clone())
        .build();

    let middle_material = MaterialBuilder::new()
        .with_color(Color::new(0.1, 1.0, 0.5))
        .with_diffuse(0.7)
        .with_specular(0.3)
        .build();
    let middle = SphereBuilder::new()
        .with_transform(Matrix::translation(-0.5, 1.0, 0.5))
        .with_material(middle_material)
        .build();

    let right_material = MaterialBuilder::new()
        .with_color(Color::new(0.5, 1.0, 0.1))
        .with_diffuse(0.7)
        .with_specular(0.3)
        .build();
    let right = SphereBuilder::new()
        .with_transform(Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5))
        .with_material(right_material)
        .build();

    let left_material = MaterialBuilder::new()
        .with_color(Color::new(1.0, 0.8, 0.1))
        .with_diffuse(0.7)
        .with_specular(0.3)
        .build();
    let left = SphereBuilder::new()
        .with_transform(Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33))
        .with_material(left_material)
        .build();

    let world = WorldBuilder::new()
        .with_object(Rc::new(floor))
        .with_object(Rc::new(rear_wall))
        .with_object(Rc::new(middle))
        .with_object(Rc::new(right))
        .with_object(Rc::new(left))
        .build();

    let camera = CameraBuilder::new()
        .with_hsize(1000)
        .with_vsize(750)
        .with_field_of_view(PI / 3.0)
        .with_transform(Matrix::view_transform(
            Vector4::point(0.0, 1.5, -5.0),
            Vector4::point(0.0, 1.0, 0.0),
            Vector4::vector(0.0, 1.0, 0.0),
        ))
        .build();

    let canvas = camera.render(&world);

    let mut file = File::create("/tmp/scene.ppm").unwrap();
    canvas.save(&mut file)?;

    Ok(())
}
