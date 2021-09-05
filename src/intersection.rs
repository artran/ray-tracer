use std::cmp::Ordering::Equal;
use std::ops::Index;
use std::rc::Rc;

use nalgebra::Vector4;

use crate::ray::Ray;
use crate::shape::Shape;
use crate::sphere::Sphere;

const EPSILON: f32 = 0.001;

#[derive(Clone, Debug, PartialEq)]
pub struct Intersection {
    pub t: f32,
    pub object: Rc<Sphere>,
}

#[derive(Debug)]
pub struct Intersections {
    intersections: Vec<Intersection>,
}

pub struct Computations {
    pub t: f32,
    pub object: Rc<Sphere>,
    pub point: Vector4<f32>,
    pub over_point: Vector4<f32>,
    pub eye_vector: Vector4<f32>,
    pub normal_vector: Vector4<f32>,
    pub inside: bool,
}

impl Intersection {
    pub fn new(t: f32, object: Rc<Sphere>) -> Self {
        Self {
            t,
            object,
        }
    }

    pub fn prepare_computations(&self, ray: &Ray) -> Computations {
        let point = ray.position(self.t);
        let eye_vector = -ray.direction;

        let mut normal_vector = self.object.normal_at(&point);
        let mut inside = false;
        if normal_vector.dot(&eye_vector) < 0.0 {
            inside = true;
            normal_vector = -normal_vector;
        }

        let over_point = point + normal_vector * EPSILON;

        Computations {
            t: self.t,
            object: Rc::clone(&self.object),
            point,
            over_point,
            eye_vector,
            normal_vector,
            inside,
        }
    }
}

impl<'a> Intersections {
    fn sort(&mut self) {
        self.intersections.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Equal));
    }

    pub fn len(&self) -> usize {
        self.intersections.len()
    }

    pub fn push(&mut self, intersection: Intersection) {
        self.intersections.push(intersection);
        self.sort();
    }

    pub fn hit(&self) -> Option<&Intersection> {
        for i in &self.intersections {
            if i.t >= 0.0 {
                return Some(i);
            }
        }

        None
    }
}

impl<'a> Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}

impl<'a> Default for Intersections {
    fn default() -> Self {
        Self {
            intersections: Vec::new()
        }
    }
}

impl<'a> IntoIterator for Intersections {
    type Item = Intersection;
    type IntoIter = std::vec::IntoIter<Intersection>;

    fn into_iter(self) -> Self::IntoIter {
        self.intersections.into_iter()
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use nalgebra::Matrix4;
    use spectral::prelude::*;

    use crate::ray::Ray;
    use crate::sphere::SphereBuilder;
    use crate::transform::Transform;
    use crate::tuple::Tuple;

    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Rc::new(SphereBuilder::new().build());

        let i = Intersection::new(3.5, Rc::clone(&s));

        assert_that!(i.t).is_equal_to(3.5);
        assert_that!(i.object).is_equal_to(&s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Rc::new(SphereBuilder::new().build());
        let i1 = Intersection::new(1.0, Rc::clone(&s));
        let i2 = Intersection::new(2.0, Rc::clone(&s));

        let mut xs = Intersections::default();
        xs.push(i1);
        xs.push(i2);

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0].t).is_equal_to(1.0);
        assert_that!(xs[1].t).is_equal_to(2.0);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Rc::new(SphereBuilder::new().build());
        let i1 = Intersection::new(1.0, Rc::clone(&s));
        let i2 = Intersection::new(2.0, Rc::clone(&s));
        let mut xs = Intersections::default();
        xs.push(i1.clone());
        xs.push(i2);

        let i = xs.hit();

        assert_that!(i).is_some().is_equal_to(&i1);
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Rc::new(SphereBuilder::new().build());
        let i1 = Intersection::new(-1.0, Rc::clone(&s));
        let i2 = Intersection::new(1.0, Rc::clone(&s));
        let mut xs = Intersections::default();
        xs.push(i2.clone());
        xs.push(i1);

        let i = xs.hit();

        assert_that!(i).is_some().is_equal_to(&i2);
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Rc::new(SphereBuilder::new().build());
        let i1 = Intersection::new(-2.0, Rc::clone(&s));
        let i2 = Intersection::new(-1.0, Rc::clone(&s));
        let mut xs = Intersections::default();
        xs.push(i2);
        xs.push(i1);

        let i = xs.hit();

        assert_that!(i).is_none();
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Rc::new(SphereBuilder::new().build());
        let i1 = Intersection::new(5.0, Rc::clone(&s));
        let i2 = Intersection::new(7.0, Rc::clone(&s));
        let i3 = Intersection::new(-3.0, Rc::clone(&s));
        let i4 = Intersection::new(2.0, Rc::clone(&s));
        let mut xs = Intersections::default();
        xs.push(i1);
        xs.push(i2);
        xs.push(i3);
        xs.push(i4.clone());

        let i = xs.hit();

        assert_that!(i).is_some().is_equal_to(&i4);
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Vector4::point(0.0, 0.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));
        let shape = Rc::new(SphereBuilder::new().build());
        let i = Intersection::new(4.0, Rc::clone(&shape));

        let comps = i.prepare_computations(&r);

        assert_that!(comps.t).is_equal_to(i.t);
        assert_that!(comps.object).is_equal_to(&shape);
        assert_that!(comps.point).is_equal_to(Vector4::point(0.0, 0.0, -1.0));
        assert_that!(comps.eye_vector).is_equal_to(Vector4::vector(0.0, 0.0, -1.0));
        assert_that!(comps.normal_vector).is_equal_to(Vector4::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Vector4::point(0.0, 0.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));
        let shape = Rc::new(SphereBuilder::new().build());
        let i = Intersection::new(4.0, Rc::clone(&shape));

        let comps = i.prepare_computations(&r);

        assert_that!(comps.inside).is_false();
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Vector4::point(0.0, 0.0, 0.0), Vector4::vector(0.0, 0.0, 1.0));
        let shape = Rc::new(SphereBuilder::new().build());
        let i = Intersection::new(1.0, Rc::clone(&shape));

        let comps = i.prepare_computations(&r);

        assert_that!(comps.point).is_equal_to(Vector4::point(0.0, 0.0, 1.0));
        assert_that!(comps.eye_vector).is_equal_to(Vector4::vector(0.0, 0.0, -1.0));
        assert_that!(comps.inside).is_true();
        // normal would have been (0, 0, 1), but is inverted!
        assert_that!(comps.normal_vector).is_equal_to(Vector4::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(Vector4::point(0.0, 0.0, -5.0), Vector4::vector(0.0, 0.0, 1.0));
        let shape = Rc::new(SphereBuilder::new()
            .with_transform(Matrix4::translation(0.0, 0.0, 1.0))
            .build());
        let i = Intersection::new(5.0, shape);

        let comps = i.prepare_computations(&r);

        assert_that!(comps.over_point.z).is_less_than(-EPSILON / 2.0);
        assert_that!(comps.point.z).is_greater_than(comps.over_point.z);
    }
}
