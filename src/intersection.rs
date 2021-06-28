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

#[derive(Debug)]
pub struct Intersections<'a> {
    intersections: Vec<Intersection<'a>>
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
    use spectral::prelude::*;

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
}
