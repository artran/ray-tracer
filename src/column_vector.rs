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
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use crate::column_vector::ColumnVector;
    use spectral::assert_that;

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
}
