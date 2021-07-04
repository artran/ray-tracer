use crate::intersection::{Intersections, Computations};
use crate::light::PointLight;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::color::Color;
use nalgebra::Vector4;
use crate::tuple::Tuple;

pub struct World {
    objects: Vec<Sphere>,
    light_source: PointLight,
}

impl World {
    pub fn intersect(&self, ray: &Ray) -> Option<Intersections> {
        let mut found: Intersections = Intersections::default();

        for o in self.objects.iter() {
            if let Some(intersections) = o.intersect(ray) {
                for intersection in intersections.into_iter() {
                    found.push(intersection);
                }
            }
        }

        if found.len() == 0 {
            return None;
        }

        Some(found)
    }

    pub fn set_light_source(&mut self, light_source: PointLight) {
        self.light_source = light_source;
    }

    pub fn add_object(&mut self, object: Sphere) {
        self.objects.push(object);
    }

    pub fn shade_hit(&self, comps: Computations) -> Color {
        let ls = &self.light_source;
        comps.object.material.lighting(ls, comps.point, comps.eye_vector, comps.normal_vector, false)
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        if let Some(intersections) = self.intersect(ray) {
            if let Some(hit) = intersections.hit() {
                let comps = hit.prepare_computations(ray);
                return self.shade_hit(comps);
            }
        }

        Color::black()
    }

    fn is_shadowed(&self, point: &Vector4<f32>) -> bool {
        let v = self.light_source.position - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(point.clone(), direction);
        if let Some(intersections) = self.intersect(&r) {
            if let Some(h) = intersections.hit() {
                return h.t < distance;
            }
        }

        false
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            objects: Vec::new(),
            light_source: PointLight::new(Vector4::point(-10.0, 10.0, -10.0), Color::white()),
        }
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use nalgebra::{Matrix4, Vector4};
    use rstest::*;
    use spectral::prelude::*;

    use crate::transform::Transform;
    use crate::tuple::Tuple;

    use super::*;
    use crate::intersection::Intersection;

    #[fixture]
    fn default_world() -> World {
        let mut s1 = Sphere::default();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::default();
        s2.set_transform(Matrix4::scaling(0.5, 0.5, 0.5));

        let mut w = World::default();
        w.objects.push(s1);
        w.objects.push(s2);

        w
    }

    #[rstest]
    fn creating_a_world() {
        let w = World::default();
        let expected_light = PointLight::new(Vector4::point(-10.0, 10.0, -10.0), Color::white());

        assert_that!(w.objects).is_empty();
        assert_that!(w.light_source).is_equal_to(expected_light);
    }

    #[rstest]
    fn intersect_a_world_with_a_ray(default_world: World) {
        let r = Ray::new(Vector4::point(0.0, 0.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));

        let xs = default_world.intersect(&r).unwrap();

        assert_that!(xs.len()).is_equal_to(4);
        assert_that!(xs[0].t).is_equal_to(4.0);
        assert_that!(xs[1].t).is_equal_to(4.5);
        assert_that!(xs[2].t).is_equal_to(5.5);
        assert_that!(xs[3].t).is_equal_to(6.0);
    }

    #[rstest]
    fn shading_an_intersection(default_world: World) {
        let r = Ray::new(Vector4::point(0.0, 0.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));
        let shape = &default_world.objects[0];
        let i = Intersection::new(4.0, shape);
        let comps = i.prepare_computations(&r);
        let expected = Color::new(0.38066, 0.47583, 0.2855);

        let c = default_world.shade_hit(comps);

        assert_that!(c.r).is_close_to(expected.r, 0.0001);
        assert_that!(c.g).is_close_to(expected.g, 0.0001);
        assert_that!(c.b).is_close_to(expected.b, 0.0001);
    }

    #[rstest]
    fn shading_an_intersection_from_the_inside(mut default_world: World) {
        default_world.set_light_source(PointLight::new(Vector4::point(0.0, 0.25, 0.0), Color::white()));
        let r = Ray::new(Vector4::point(0.0, 0.0, 0.0), Vector4::vector(0.0, 0.0, 1.0));
        let shape = &default_world.objects[1];
        let i = Intersection::new(0.5, shape);
        let comps = i.prepare_computations(&r);
        let expected = Color::new(0.90498, 0.90498, 0.90498);

        let c = default_world.shade_hit(comps);

        assert_that!(c.r).is_close_to(expected.r, 0.0001);
        assert_that!(c.g).is_close_to(expected.g, 0.0001);
        assert_that!(c.b).is_close_to(expected.b, 0.0001);
    }

    #[rstest]
    fn the_color_when_a_ray_misses(default_world: World) {
        let r = Ray::new(Vector4::point(0.0, 0.0, -5.0), Vector4::vector(0.0, 1.0, 0.0));

        let c = default_world.color_at(&r);

        assert_that!(c).is_equal_to(Color::black());
    }

    #[rstest]
    fn the_color_when_a_ray_hits(default_world: World) {
        let r = Ray::new(Vector4::point(0.0, 0.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));
        let expected = Color::new(0.38066, 0.47583, 0.2855);

        let c = default_world.color_at(&r);

        assert_that!(c.r).is_close_to(expected.r, 0.0001);
        assert_that!(c.g).is_close_to(expected.g, 0.0001);
        assert_that!(c.b).is_close_to(expected.b, 0.0001);
    }

    #[rstest]
    fn the_color_with_an_intersection_behind_the_ray(mut default_world: World) {
        let mut outer = default_world.objects[0].clone();
        let mut inner = default_world.objects[1].clone();
        let expected_color = inner.material.color.clone();
        outer.material.ambient = 1.0;
        inner.material.ambient = 1.0;
        default_world.objects.clear();
        default_world.objects.push(outer);
        default_world.objects.push(inner);
        let r = Ray::new(Vector4::point(0.0, 0.0, 0.75), Vector4::vector(0.0, 0.0, -1.0));

        let c = default_world.color_at(&r);

        assert_that!(c).is_equal_to(expected_color);
    }

    #[rstest]
    #[case(Vector4::point(0.0, 10.0, 0.0), false)] // there_is_no_shadow_when_nothing_is_co_linear_with_point_and_light
    #[case(Vector4::point(10.0, - 10.0, 10.0), true)] // the_shadow_when_an_object_is_between_the_point_and_the_light
    #[case(Vector4::point(- 20.0, 20.0, - 20.0), false)] // there_is_no_shadow_when_an_object_is_behind_the_light
    #[case(Vector4::point(- 2.0, 2.0, - 2.0), false)] // there_is_no_shadow_when_an_object_is_behind_the_point
    fn test_is_shadowed (default_world: World, #[case] p: Vector4<f32>, #[case] expected: bool) {
        assert_that!(default_world.is_shadowed(&p)).is_equal_to(expected);
    }
}
