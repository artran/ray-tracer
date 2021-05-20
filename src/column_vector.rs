#[derive(Debug, PartialEq)]
pub struct ColumnVector {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl ColumnVector {
    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            w: 1.0,
        }
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            w: 0.0,
        }
    }

    pub fn plus(&self, other: &ColumnVector) -> ColumnVector {
        ColumnVector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }

    pub fn minus(&self, other: &ColumnVector) -> ColumnVector {
        ColumnVector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }

    pub fn negate(&self) -> ColumnVector {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }

    pub fn scale(&self, scale: &f32) -> ColumnVector {
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

    pub fn normalize(&self) -> ColumnVector {
        let scale = self.magnitude();
        ColumnVector {
            x: self.x / scale,
            y: self.y / scale,
            z: self.z / scale,
            w: self.w / scale,
        }
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use spectral::assert_that;
    use spectral::numeric::FloatAssertions;

    use crate::column_vector::ColumnVector;

    #[test]
    fn point_has_three_coordinates() {
        let cv = ColumnVector::point(4.3, -4.2, 3.1);

        assert_that!(cv.x).is_equal_to(4.3);
        assert_that!(cv.y).is_equal_to(-4.2);
        assert_that!(cv.z).is_equal_to(3.1);
    }

    #[test]
    fn point_has_w_of_1() {
        let cv = ColumnVector::point(3.0, 2.0, 1.0);

        assert_that!(cv.w).is_equal_to(1.0);
    }

    #[test]
    fn vector_has_three_coordinates() {
        let cv = ColumnVector::vector(4.3, -4.2, 3.1);

        assert_that!(cv.x).is_equal_to(4.3);
        assert_that!(cv.y).is_equal_to(-4.2);
        assert_that!(cv.z).is_equal_to(3.1);
    }

    #[test]
    fn vector_has_w_of_0() {
        let cv = ColumnVector::vector(1.0, 2.0, 3.0);

        assert_that!(cv.w).is_equal_to(0.0);
    }

    #[test]
    fn adding_a_vector_to_a_point_returns_a_new_point() {
        let point = ColumnVector::point(3.0, -2.0, 5.0);
        let vector = ColumnVector::vector(-2.0, 3.0, 1.0);
        let expected = ColumnVector::point(1.0, 1.0, 6.0);

        assert_that!(point.plus(&vector)).is_equal_to(expected)
    }

    #[test]
    fn subtracting_two_points_returns_a_vector() {
        let point1 = ColumnVector::point(3.0, 2.0, 1.0);
        let point2 = ColumnVector::point(5.0, 6.0, 7.0);
        let expected = ColumnVector::vector(-2.0, -4.0, -6.0);

        assert_that!(point1.minus(&point2)).is_equal_to(expected)
    }

    #[test]
    fn subtracting_a_vector_from_a_point_returns_a_new_point() {
        let point = ColumnVector::point(3.0, 2.0, 1.0);
        let vector = ColumnVector::vector(5.0, 6.0, 7.0);
        let expected = ColumnVector::point(-2.0, -4.0, -6.0);

        assert_that!(point.minus(&vector)).is_equal_to(expected)
    }

    #[test]
    fn negating_a_vector_returns_the_negative_vector() {
        let vector = ColumnVector::vector(1.0, -2.0, 3.0);
        let expected = ColumnVector::vector(-1.0, 2.0, -3.0);

        assert_that!(vector.negate()).is_equal_to(expected);
    }

    #[test]
    fn negating_a_tuple_returns_the_negative_tuple() {
        let tuple = ColumnVector { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        let expected = ColumnVector { x: -1.0, y: 2.0, z: -3.0, w: 4.0 };

        assert_that!(tuple.negate()).is_equal_to(expected);
    }

    #[test]
    fn multiplying_a_tuple_by_scalar_scales_the_tuple() {
        let tuple = ColumnVector { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        let expected = ColumnVector { x: 3.5, y: -7.0, z: 10.5, w: -14.0 };

        assert_that!(tuple.scale(&3.5)).is_equal_to(expected);
    }

    #[test]
    fn multiplying_a_tuple_by_fraction_scales_the_tuple() {
        let tuple = ColumnVector { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        let expected = ColumnVector { x: 0.5, y: -1.0, z: 1.5, w: -2.0 };

        assert_that!(tuple.scale(&0.5)).is_equal_to(expected);
    }

    #[test]
    fn unit_vector_x_has_magnitude_of_1() {
        let x = ColumnVector::vector(1.0, 0.0, 0.0);

        assert_that!(x.magnitude()).is_equal_to(1.0);
    }

    #[test]
    fn unit_vector_y_has_magnitude_of_1() {
        let y = ColumnVector::vector(0.0, 1.0, 0.0);

        assert_that!(y.magnitude()).is_equal_to(1.0);
    }

    #[test]
    fn unit_vector_z_has_magnitude_of_1() {
        let z = ColumnVector::vector(0.0, 0.0, 1.0);

        assert_that!(z.magnitude()).is_equal_to(1.0);
    }

    #[test]
    fn vector_has_magnitude() {
        let z = ColumnVector::vector(1.0, 2.0, 3.0);

        assert_that!(z.magnitude()).is_equal_to(14.0_f32.sqrt());
    }

    #[test]
    fn negative_vector_has_positive_magnitude() {
        let z = ColumnVector::vector(-1.0, -2.0, -3.0);

        assert_that!(z.magnitude()).is_equal_to(14.0_f32.sqrt());
    }

    #[test]
    fn normalizing_vector_returns_unit_vector_same_direction() {
        let x = ColumnVector::vector(4.0, 0.0, 0.0);
        let expected = ColumnVector::vector(1.0, 0.0, 0.0);

        assert_that!(x.normalize()).is_equal_to(expected);
    }

    #[test]
    fn normalizing_vector_returns_unit_vector_same_direction2() {
        let x = ColumnVector::vector(1.0, 2.0, 3.0);
        let expected = ColumnVector::vector(0.26726, 0.53452, 0.80178);

        let normalized = x.normalize();

        assert_that!(normalized.x).is_close_to(expected.x, 0.0001_f32);
        assert_that!(normalized.y).is_close_to(expected.y, 0.0001_f32);
        assert_that!(normalized.z).is_close_to(expected.z, 0.0001_f32);
        assert_that!(normalized.w).is_equal_to(expected.w);
    }

    #[test]
    fn normalized_vector_is_a_unit_vector() {
        let x = ColumnVector::vector(1.0, 2.0, 3.0);

        assert_that!(x.normalize().magnitude()).is_close_to(1.0, 0.0001_f32);
    }
}
