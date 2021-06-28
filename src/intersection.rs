use std::cmp::Ordering::Equal;
use std::ops::Index;

use nalgebra::Vector4;

use crate::ray::Ray;
use crate::sphere::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a Sphere,
}

#[derive(Debug)]
pub struct Intersections<'a> {
    intersections: Vec<Intersection<'a>>
}

struct Computations<'a> {
    t: f32,
    object: &'a Sphere,
    point: Vector4<f32>,
    eye_vector: Vector4<f32>,
    normal_vector: Vector4<f32>,
    inside: bool,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, object: &'a Sphere) -> Self {
        Self {
            t, object
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

        Computations {
            t: self.t,
            object: self.object,
            point,
            eye_vector,
            normal_vector,
            inside,
        }
    }
}

impl<'a> Intersections<'a> {
    fn sort(&mut self) {
        self.intersections.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Equal));
    }

    pub fn len(&self) -> usize {
        self.intersections.len()
    }

    pub fn push(&mut self, intersection: Intersection<'a>) {
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

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}

impl<'a> Default for Intersections<'a> {
    fn default() -> Self {
        Self {
            intersections: Vec::new()
        }
    }
}

impl<'a> IntoIterator for Intersections<'a> {
    type Item = Intersection<'a>;
    type IntoIter = std::vec::IntoIter<Intersection<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.intersections.into_iter()
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use nalgebra::Vector4;
    use spectral::prelude::*;

    use crate::ray::Ray;
    use crate::tuple::Tuple;

    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::default();

        let i = Intersection::new(3.5, &s);

        assert_that!(i.t).is_equal_to(3.5);
        assert_that!(i.object).is_equal_to(&s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);

        let mut xs = Intersections::default();
        xs.push(i1);
        xs.push(i2);

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0].t).is_equal_to(1.0);
        assert_that!(xs[1].t).is_equal_to(2.0);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let mut xs = Intersections::default();
        xs.push(i1.clone());
        xs.push(i2);

        let i = xs.hit();

        assert_that!(i).is_some().is_equal_to(&i1);
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let mut xs = Intersections::default();
        xs.push(i2.clone());
        xs.push(i1);

        let i = xs.hit();

        assert_that!(i).is_some().is_equal_to(&i2);
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let mut xs = Intersections::default();
        xs.push(i2);
        xs.push(i1);

        let i = xs.hit();

        assert_that!(i).is_none();
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::default();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
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
        let shape = Sphere::default();
        let i = Intersection::new(4.0, &shape);

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
        let shape = Sphere::default();
        let i = Intersection::new(4.0, &shape);

        let comps = i.prepare_computations(&r);

        assert_that!(comps.inside).is_false();
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Vector4::point(0.0, 0.0, 0.0), Vector4::vector(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(1.0, &shape);

        let comps = i.prepare_computations(&r);

        assert_that!(comps.point).is_equal_to(Vector4::point(0.0, 0.0, 1.0));
        assert_that!(comps.eye_vector).is_equal_to(Vector4::vector(0.0, 0.0, -1.0));
        assert_that!(comps.inside).is_true();
        // normal would have been (0, 0, 1), but is inverted!
        assert_that!(comps.normal_vector).is_equal_to(Vector4::vector(0.0, 0.0, -1.0));
    }
}
