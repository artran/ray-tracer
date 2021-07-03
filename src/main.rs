use std::f32::consts::PI;
use std::fs::File;

use nalgebra::{Matrix4, Vector4};

use crate::camera::Camera;
use crate::color::Color;
use crate::light::PointLight;
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
mod sphere;
mod transform;
mod tuple;
mod world;

fn main() -> Result<(), std::io::Error> {
    let mut floor = Sphere::default();
    floor.set_transform(Matrix4::scaling(10.0, 0.01, 10.0));
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = Sphere::default();
    left_wall.set_transform(Matrix4::translation(0.0, 0.0, 5.0)
        * Matrix4::rotation_y(-PI/4.0)
        * Matrix4::rotation_x(PI/2.0)
        * Matrix4::scaling(10.0, 0.01, 10.0));
    left_wall.material = floor.material.clone();

    let mut right_wall = Sphere::default();
    right_wall.set_transform(Matrix4::translation(0.0, 0.0, 5.0)
        * Matrix4::rotation_y(PI/4.0)
        * Matrix4::rotation_x(PI/2.0)
        * Matrix4::scaling(10.0, 0.01, 10.0));
    right_wall.material = floor.material.clone();

    let mut middle = Sphere::default();
    middle.set_transform(Matrix4::translation(-0.5, 1.0, 0.5));
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::default();
    right.set_transform(Matrix4::translation(1.5, 0.5, -0.5) * Matrix4::scaling(0.5, 0.5, 0.5));
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::default();
    left.set_transform(Matrix4::translation(-1.5, 0.33, -0.75) * Matrix4::scaling(0.33, 0.33, 0.33));
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut world = World::default();
    world.set_light_source(PointLight::new(Vector4::point(-10.0, 10.0, -10.0), Color::white()));
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
