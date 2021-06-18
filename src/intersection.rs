use crate::sphere::*;
use std::ops::Index;
use std::cmp::Ordering::Equal;

#[derive(Clone, Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, object: &'a Sphere) -> Self {
        Self {
            t, object
        }
    }
}

pub struct Intersections<'a> {
    intersections: Vec<&'a Intersection<'a>>
}

impl<'a> Intersections<'a> {
    pub fn new(i1: &'a Intersection, i2: &'a Intersection) -> Self {
        let intersections = vec!(i1, i2);
        let mut result = Self {
            intersections
        };

        result.sort();

        result
    }

    fn sort(&mut self) {
        self.intersections.sort_unstable_by(|&a, &b| a.t.partial_cmp(&b.t).unwrap_or(Equal));
    }

    pub fn len(&self) -> usize {
        self.intersections.len()
    }

    pub fn push(&mut self, intersection: &'a Intersection) {
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

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use spectral::assert_that;
    use spectral::option::OptionAssertions;

    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::new();

        let i = Intersection::new(3.5, &s);

        assert_that!(i.t).is_equal_to(3.5);
        assert_that!(i.object).is_equal_to(&s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);

        let xs = Intersections::new(&i1, &i2);

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0].t).is_equal_to(1.0);
        assert_that!(xs[1].t).is_equal_to(2.0);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::new(&i1, &i2);

        let i = xs.hit();

        assert_that!(i).is_some().is_equal_to(&i1);
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = Intersections::new(&i2, &i1);

        let i = xs.hit();

        assert_that!(i).is_some().is_equal_to(&i2);
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = Intersections::new(&i2, &i1);

        let i = xs.hit();

        assert_that!(i).is_none();
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let mut xs = Intersections::new(&i1, &i2);
        xs.push(&i3);
        xs.push(&i4);

        let i = xs.hit();

        assert_that!(i).is_some().is_equal_to(&i4);
    }
}
