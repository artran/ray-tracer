use crate::sphere::*;

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

// pub struct Intersections<'a> {
//     intersections: Vec<Intersection<'a>>
// }
//
// impl<'a> Intersections<'a> {
//     pub fn new(i1: Intersection<'a>, i2: Intersection<'a>) -> Self {
//         let intersections = vec!(i1, i2);
//         Self {
//             intersections
//         }
//     }
//
//     pub fn len(self) -> usize {
//         self.intersections.len()
//     }
// }

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use spectral::assert_that;

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

        let xs = vec!(i1, i2);

        assert_that!(xs.len()).is_equal_to(2);
        assert_that!(xs[0].t).is_equal_to(1.0);
        assert_that!(xs[1].t).is_equal_to(2.0);
    }
}
