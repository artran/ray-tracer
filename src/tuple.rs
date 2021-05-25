#[derive(Debug, PartialEq)]
pub struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn plus(&self, other: &Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }

    pub fn minus(&self, other: &Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }

    pub fn negate(&self) -> Tuple {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }

    pub fn scale(&self, scale: &f32) -> Tuple {
        Self {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
            w: self.w * scale,
        }
    }

    pub fn magnitude(&self) -> f32 {
        // Note: Not including the 'w' part at the moment as this probably only makes sense for
        //       vectors where w = 0.0
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let scale = self.magnitude();
        Tuple {
            x: self.x / scale,
            y: self.y / scale,
            z: self.z / scale,
            w: self.w / scale,
        }
    }

    pub fn dot(&self, other: &Tuple) -> f32 {
        // a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Tuple) -> Tuple {
        // vector(a.y * b.z - a.z * b.y,
        //        a.z * b.x - a.x * b.z,
        //        a.x * b.y - a.y * b.x)
        Self::vector(self.y * other.z - self.z * other.y,
                     self.z * other.x - self.x * other.z,
                     self.x * other.y - self.y * other.x,
        )
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use spectral::assert_that;
    use spectral::numeric::FloatAssertions;

    use crate::tuple::Tuple;

    #[test]
    fn point_has_three_coordinates() {
        let cv = Tuple::point(4.3, -4.2, 3.1);

        assert_that!(cv.x).is_equal_to(4.3);
        assert_that!(cv.y).is_equal_to(-4.2);
        assert_that!(cv.z).is_equal_to(3.1);
    }

    #[test]
    fn point_has_w_of_1() {
        let cv = Tuple::point(3.0, 2.0, 1.0);

        assert_that!(cv.w).is_equal_to(1.0);
    }

    #[test]
    fn vector_has_three_coordinates() {
        let cv = Tuple::vector(4.3, -4.2, 3.1);

        assert_that!(cv.x).is_equal_to(4.3);
        assert_that!(cv.y).is_equal_to(-4.2);
        assert_that!(cv.z).is_equal_to(3.1);
    }

    #[test]
    fn vector_has_w_of_0() {
        let cv = Tuple::vector(1.0, 2.0, 3.0);

        assert_that!(cv.w).is_equal_to(0.0);
    }

    #[test]
    fn adding_a_vector_to_a_point_returns_a_new_point() {
        let point = Tuple::point(3.0, -2.0, 5.0);
        let vector = Tuple::vector(-2.0, 3.0, 1.0);
        let expected = Tuple::point(1.0, 1.0, 6.0);

        assert_that!(point.plus(&vector)).is_equal_to(expected)
    }

    #[test]
    fn subtracting_two_points_returns_a_vector() {
        let point1 = Tuple::point(3.0, 2.0, 1.0);
        let point2 = Tuple::point(5.0, 6.0, 7.0);
        let expected = Tuple::vector(-2.0, -4.0, -6.0);

        assert_that!(point1.minus(&point2)).is_equal_to(expected)
    }

    #[test]
    fn subtracting_a_vector_from_a_point_returns_a_new_point() {
        let point = Tuple::point(3.0, 2.0, 1.0);
        let vector = Tuple::vector(5.0, 6.0, 7.0);
        let expected = Tuple::point(-2.0, -4.0, -6.0);

        assert_that!(point.minus(&vector)).is_equal_to(expected)
    }

    #[test]
    fn negating_a_vector_returns_the_negative_vector() {
        let vector = Tuple::vector(1.0, -2.0, 3.0);
        let expected = Tuple::vector(-1.0, 2.0, -3.0);

        assert_that!(vector.negate()).is_equal_to(expected);
    }

    #[test]
    fn negating_a_tuple_returns_the_negative_tuple() {
        let tuple = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        let expected = Tuple { x: -1.0, y: 2.0, z: -3.0, w: 4.0 };

        assert_that!(tuple.negate()).is_equal_to(expected);
    }

    #[test]
    fn multiplying_a_tuple_by_scalar_scales_the_tuple() {
        let tuple = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        let expected = Tuple { x: 3.5, y: -7.0, z: 10.5, w: -14.0 };

        assert_that!(tuple.scale(&3.5)).is_equal_to(expected);
    }

    #[test]
    fn multiplying_a_tuple_by_fraction_scales_the_tuple() {
        let tuple = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        let expected = Tuple { x: 0.5, y: -1.0, z: 1.5, w: -2.0 };

        assert_that!(tuple.scale(&0.5)).is_equal_to(expected);
    }

    #[test]
    fn unit_vector_x_has_magnitude_of_1() {
        let x = Tuple::vector(1.0, 0.0, 0.0);

        assert_that!(x.magnitude()).is_equal_to(1.0);
    }

    #[test]
    fn unit_vector_y_has_magnitude_of_1() {
        let y = Tuple::vector(0.0, 1.0, 0.0);

        assert_that!(y.magnitude()).is_equal_to(1.0);
    }

    #[test]
    fn unit_vector_z_has_magnitude_of_1() {
        let z = Tuple::vector(0.0, 0.0, 1.0);

        assert_that!(z.magnitude()).is_equal_to(1.0);
    }

    #[test]
    fn vector_has_magnitude() {
        let z = Tuple::vector(1.0, 2.0, 3.0);

        assert_that!(z.magnitude()).is_equal_to(14.0_f32.sqrt());
    }

    #[test]
    fn negative_vector_has_positive_magnitude() {
        let z = Tuple::vector(-1.0, -2.0, -3.0);

        assert_that!(z.magnitude()).is_equal_to(14.0_f32.sqrt());
    }

    #[test]
    fn normalizing_vector_returns_unit_vector_same_direction() {
        let x = Tuple::vector(4.0, 0.0, 0.0);
        let expected = Tuple::vector(1.0, 0.0, 0.0);

        assert_that!(x.normalize()).is_equal_to(expected);
    }

    #[test]
    fn normalizing_vector_returns_unit_vector_same_direction2() {
        let x = Tuple::vector(1.0, 2.0, 3.0);
        let expected = Tuple::vector(0.26726, 0.53452, 0.80178);

        let normalized = x.normalize();

        assert_that!(normalized.x).is_close_to(expected.x, 0.0001_f32);
        assert_that!(normalized.y).is_close_to(expected.y, 0.0001_f32);
        assert_that!(normalized.z).is_close_to(expected.z, 0.0001_f32);
        assert_that!(normalized.w).is_equal_to(expected.w);
    }

    #[test]
    fn normalized_vector_is_a_unit_vector() {
        let x = Tuple::vector(1.0, 2.0, 3.0);

        assert_that!(x.normalize().magnitude()).is_close_to(1.0, 0.0001_f32);
    }

    #[test]
    fn dot_product_of_two_vectors_is_scalar() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert_that!(a.dot(&b)).is_equal_to(20.0);
    }

    #[test]
    fn cross_product_of_two_vectors_is_vector() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        let expected = Tuple::vector(-1.0, 2.0, -1.0);

        assert_that!(a.cross(&b)).is_equal_to(expected);
    }

    #[test]
    fn cross_product_of_two_vectors_is_not_commutative() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        let expected = Tuple::vector(1.0, -2.0, 1.0);

        assert_that!(b.cross(&a)).is_equal_to(expected);
    }
}
