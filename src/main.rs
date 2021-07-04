use std::f32::consts::PI;
use std::fs::File;

use nalgebra::{Matrix4, Vector4};

use crate::camera::Camera;
use crate::color::Color;
use crate::material::Material;
use crate::shape::Shape;
use crate::sphere::Sphere;
use crate::transform::Transform;
use crate::tuple::Tuple;
use crate::world::World;

mod camera;
mod canvas;
mod color;
mod intersection;
mod light;
mod material;
mod matrix;
mod ray;
mod shape;
mod sphere;
mod transform;
mod tuple;
mod world;

fn main() -> Result<(), std::io::Error> {
    let mut wall_material = Material::default();
    wall_material.color = Color::new(1.0, 0.9, 0.9);
    wall_material.specular = 0.0;

    let mut floor = Sphere::default();
    floor.set_transform(Matrix4::scaling(10.0, 0.01, 10.0));
    floor.set_material(wall_material.clone());

    let mut left_wall = Sphere::default();
    left_wall.set_transform(Matrix4::translation(0.0, 0.0, 5.0)
        * Matrix4::rotation_y(-PI/4.0)
        * Matrix4::rotation_x(PI/2.0)
        * Matrix4::scaling(10.0, 0.01, 10.0));
    left_wall.set_material(wall_material.clone());

    let mut right_wall = Sphere::default();
    right_wall.set_transform(Matrix4::translation(0.0, 0.0, 5.0)
        * Matrix4::rotation_y(PI/4.0)
        * Matrix4::rotation_x(PI/2.0)
        * Matrix4::scaling(10.0, 0.01, 10.0));
    right_wall.set_material(wall_material.clone());

    let mut middle = Sphere::default();
    middle.set_transform(Matrix4::translation(-0.5, 1.0, 0.5));
    let mut middle_material = Material::default();
    middle_material.color = Color::new(0.1, 1.0, 0.5);
    middle_material.diffuse = 0.7;
    middle_material.specular = 0.3;
    middle.set_material(middle_material);

    let mut right = Sphere::default();
    right.set_transform(Matrix4::translation(1.5, 0.5, -0.5) * Matrix4::scaling(0.5, 0.5, 0.5));
    let mut right_material = Material::default();
    right_material.color = Color::new(0.5, 1.0, 0.1);
    right_material.diffuse = 0.7;
    right_material.specular = 0.3;
    right.set_material(right_material);

    let mut left = Sphere::default();
    left.set_transform(Matrix4::translation(-1.5, 0.33, -0.75) * Matrix4::scaling(0.33, 0.33, 0.33));
    let mut left_material = Material::default();
    left_material.color = Color::new(1.0, 0.8, 0.1);
    left_material.diffuse = 0.7;
    left_material.specular = 0.3;
    left.set_material(left_material);

    let mut world = World::default();
    world.add_object(floor);
    world.add_object(left_wall);
    world.add_object(right_wall);
    world.add_object(middle);
    world.add_object(right);
    world.add_object(left);

    let mut camera = Camera::new(1000, 750, PI/3.0);
    camera.transform = Matrix4::view_transform(Vector4::point(0.0, 1.5, -5.0),
                                      Vector4::point(0.0, 1.0, 0.0),
                                      Vector4::vector(0.0, 1.0, 0.0));

    let canvas = camera.render(&world);

    let mut file = File::create("/tmp/scene.ppm").unwrap();
    canvas.save(&mut file)?;

    Ok(())
}
