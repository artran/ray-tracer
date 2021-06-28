use crate::intersection::Intersections;
use crate::light::PointLight;
use crate::ray::Ray;
use crate::sphere::Sphere;

struct World {
    pub objects: Vec<Sphere>,
    pub light_source: Option<PointLight>,
}

impl World {
    fn new() -> Self {
        Self {
            objects: Vec::new(),
            light_source: None
        }
    }

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
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use nalgebra::{Matrix4, Vector4};
    use rstest::*;
    use spectral::prelude::*;

    use crate::color::Color;
    use crate::transform::Transform;
    use crate::tuple::Tuple;

    use super::*;

    #[fixture]
    fn world() -> World {
        let light = PointLight::new(Vector4::point(-10.0, 10.0, -10.0), Color::white());

        let mut s1 = Sphere::default();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::default();
        s2.set_transform(Matrix4::scaling(0.5, 0.5, 0.5));

        let mut w = World::new();
        w.objects.push(s1);
        w.objects.push(s2);
        w.light_source = Some(light);

        w
    }

    #[rstest]
    fn creating_a_world() {
        let w = World::new();
        assert_that!(w.objects).is_empty();
        assert_that!(w.light_source).is_none();
    }

    #[rstest]
    fn intersect_a_world_with_a_ray(world: World) {
        let r = Ray::new(Vector4::point(0.0, 0.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));

        let xs = world.intersect(&r).unwrap();

        assert_that!(xs.len()).is_equal_to(4);
        assert_that!(xs[0].t).is_equal_to(4.0);
        assert_that!(xs[1].t).is_equal_to(4.5);
        assert_that!(xs[2].t).is_equal_to(5.5);
        assert_that!(xs[3].t).is_equal_to(6.0);
    }
}
