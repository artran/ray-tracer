use nalgebra::{Matrix4, Vector4};

use crate::canvas::Canvas;
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::world::World;

struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f32,
    transform: Matrix4<f32>,
    pixel_size: f32,
    half_width: f32,
    half_height: f32,
}

impl Camera {
    fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f32 / vsize as f32;
        let half_width: f32;
        let half_height: f32;
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }
        let pixel_size = (half_width * 2.0) / hsize as f32;

        Self {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix4::identity(),
            pixel_size,
            half_width,
            half_height,
        }
    }

    fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        // the offset from the edge of the canvas to the pixel's center
        let xoffset = (px as f32 + 0.5) * self.pixel_size;
        let yoffset = (py as f32 + 0.5) * self.pixel_size;

        // the untransformed coordinates of the pixel in world space.
        // (remember that the camera looks toward -z, so +x is to the *left*.)
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        // using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction vector.
        // (remember that the canvas is at z=-1)
        let pixel = self.transform.try_inverse().unwrap() * Vector4::point(world_x, world_y, -1.0);
        let origin = self.transform.try_inverse().unwrap() * Vector4::point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize - 1 {
            for x in 0..self.hsize - 1 {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                image.write_pixel(x, y, &color)
            }
        }

        return image;
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use nalgebra::Vector4;
    use rstest::*;
    use spectral::prelude::*;

    use crate::color::Color;
    use crate::light::PointLight;
    use crate::sphere::Sphere;
    use crate::transform::Transform;

    use super::*;

    fn vector_values_are_close(actual: Vector4<f32>, expected: Vector4<f32>, tolerance: f32) {
        for row in 0..4 {
            assert_that!(actual[row]).is_close_to(expected[row], tolerance);
        }
    }

    #[test]
    fn constructing_a_camera() {
        let hsize: usize = 160;
        let vsize: usize = 120;
        let field_of_view = PI / 2.0;

        let c = Camera::new(hsize, vsize, field_of_view);

        assert_that!(c.hsize).is_equal_to(160);
        assert_that!(c.vsize).is_equal_to(120);
        assert_that!(c.field_of_view).is_equal_to(PI / 2.0);
        assert_that!(c.transform).is_equal_to(Matrix4::identity());
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);

        assert_that!(c.pixel_size).is_equal_to(0.01);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);

        assert_that!(c.pixel_size).is_equal_to(0.01);
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);

        let r = c.ray_for_pixel(100, 50);

        assert_that!(r.origin).is_equal_to(Vector4::point(0.0, 0.0, 0.0));
        vector_values_are_close(r.direction, Vector4::vector(0.0, 0.0, -1.0), 0.0001);
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);

        let r = c.ray_for_pixel(0, 0);

        assert_that!(r.origin).is_equal_to(Vector4::point(0.0, 0.0, 0.0));
        vector_values_are_close(r.direction, Vector4::vector(0.66519, 0.33259, -0.66851), 0.0001);
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform = Matrix4::rotation_y(PI / 4.0) * Matrix4::translation(0.0, -2.0, 5.0);

        let r = c.ray_for_pixel(100, 50);

        assert_that!(r.origin).is_equal_to(Vector4::point(0.0, 2.0, -5.0));
        vector_values_are_close(r.direction, Vector4::vector(2.0_f32.sqrt() / 2.0, 0.0, -2.0_f32.sqrt() / 2.0), 0.0001);
    }

    #[fixture]
    fn default_world() -> World {
        let light = PointLight::new(Vector4::point(-10.0, 10.0, -10.0), Color::white());

        let mut s1 = Sphere::default();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::default();
        s2.set_transform(Matrix4::scaling(0.5, 0.5, 0.5));

        let mut world = World::default();
        world.objects.push(s1);
        world.objects.push(s2);
        world.light_source = Some(light);

        world
    }

    #[rstest]
    fn rendering_a_world_with_a_camera(default_world: World) {
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = Vector4::point(0.0, 0.0, -5.0);
        let to = Vector4::point(0.0, 0.0, 0.0);
        let up = Vector4::vector(0.0, 1.0, 0.0);
        c.transform = Matrix4::view_transform(from, to, up);

        let image = c.render(&default_world);

        let actual = image.pixel_at(5, 5);
        let expected = Color::new(0.38066, 0.47583, 0.2855);
        assert_that!(actual.r).is_close_to(expected.r, 0.0001);
        assert_that!(actual.g).is_close_to(expected.g, 0.0001);
        assert_that!(actual.b).is_close_to(expected.b, 0.0001);
    }
}
