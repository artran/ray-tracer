use nalgebra::Vector4;

pub trait Tuple {
    fn point(x: f32, y: f32, z: f32) -> Vector4<f32>;
    fn vector(x: f32, y: f32, z: f32) -> Vector4<f32>;
    fn cross_product(&self, other: &Vector4<f32>) -> Vector4<f32>;
    fn is_point(&self) -> bool;
    fn is_vector(&self) -> bool;
}

impl Tuple for Vector4<f32> {
    fn point(x: f32, y: f32, z: f32) -> Vector4<f32> {
        Vector4::new(x, y, z, 1.0)
    }

    fn vector(x: f32, y: f32, z: f32) -> Vector4<f32> {
        Vector4::new(x, y, z, 0.0)
    }

    fn cross_product(&self, other: &Vector4<f32>) -> Vector4<f32> {
        let me = self.remove_row(3);
        let rhs = other.remove_row(3);

        let res = me.cross(&rhs);

        res.insert_row(3, 0.0)
    }

    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use spectral::assert_that;
    use spectral::boolean::BooleanAssertions;
    use spectral::numeric::FloatAssertions;

    use super::*;

    #[test]
    fn point_has_three_coordinates() {
        let cv = Vector4::point(4.3, -4.2, 3.1);

        assert_that!(cv.x).is_equal_to(4.3);
        assert_that!(cv.y).is_equal_to(-4.2);
        assert_that!(cv.z).is_equal_to(3.1);
    }

    #[test]
    fn point_has_w_of_1() {
        let cv = Vector4::point(3.0, 2.0, 1.0);

        assert_that!(cv.w).is_equal_to(1.0);
    }

    #[test]
    fn point_returns_true_for_is_point_false_for_is_vector() {
        let cv = Vector4::point(1.0, 2.0, 3.0);

        assert_that!(cv.is_vector()).is_false();
        assert_that!(cv.is_point()).is_true();
    }

    #[test]
    fn vector_has_three_coordinates() {
        let cv = Vector4::vector(4.3, -4.2, 3.1);

        assert_that!(cv.x).is_equal_to(4.3);
        assert_that!(cv.y).is_equal_to(-4.2);
        assert_that!(cv.z).is_equal_to(3.1);
    }

    #[test]
    fn vector_has_w_of_0() {
        let cv = Vector4::vector(1.0, 2.0, 3.0);

        assert_that!(cv.w).is_equal_to(0.0);
    }

    #[test]
    fn vector_returns_true_for_is_vector_false_for_is_point() {
        let cv = Vector4::vector(1.0, 2.0, 3.0);

        assert_that!(cv.is_vector()).is_true();
        assert_that!(cv.is_point()).is_false();
    }

    #[test]
    fn adding_a_vector_to_a_point_returns_a_new_point() {
        let point = Vector4::point(3.0, -2.0, 5.0);
        let vector = Vector4::vector(-2.0, 3.0, 1.0);
        let expected = Vector4::point(1.0, 1.0, 6.0);

        let result = &point + &vector;

        assert_that!(result).is_equal_to(expected);
    }

    #[test]
    fn subtracting_two_points_returns_a_vector() {
        let point1 = Vector4::point(3.0, 2.0, 1.0);
        let point2 = Vector4::point(5.0, 6.0, 7.0);
        let expected = Vector4::vector(-2.0, -4.0, -6.0);

        let result = &point1 - &point2;

        assert_that!(result).is_equal_to(expected)
    }

    #[test]
    fn subtracting_a_vector_from_a_point_returns_a_new_point() {
        let point = Vector4::point(3.0, 2.0, 1.0);
        let vector = Vector4::vector(5.0, 6.0, 7.0);
        let expected = Vector4::point(-2.0, -4.0, -6.0);

        let result = &point - &vector;

        assert_that!(result).is_equal_to(expected)
    }

    #[test]
    fn negating_a_vector_returns_the_negative_vector() {
        let vector = Vector4::vector(1.0, -2.0, 3.0);
        let expected = Vector4::vector(-1.0, 2.0, -3.0);

        let result = -&vector;

        assert_that!(result).is_equal_to(expected);
    }

    #[test]
    fn negating_a_tuple_returns_the_negative_tuple() {
        let tuple = Vector4::new(1.0, -2.0, 3.0, -4.0 );
        let expected = Vector4::new(-1.0, 2.0, -3.0, 4.0);

        let result = -&tuple;

        assert_that!(result).is_equal_to(expected);
    }

    #[test]
    fn multiplying_a_tuple_by_scalar_scales_the_tuple() {
        let tuple = Vector4::new(1.0, -2.0, 3.0, -4.0);
        let expected = Vector4::new(3.5, -7.0, 10.5, -14.0);

        assert_that!(tuple * 3.5).is_equal_to(expected);
    }

    #[test]
    fn tuple_returns_false_for_is_vector_false_for_is_point() {
        let cv = Vector4::new(1.0, 2.0, 3.0, 4.0);

        assert_that!(cv.is_vector()).is_false();
        assert_that!(cv.is_point()).is_false();
    }

    #[test]
    fn multiplying_a_tuple_by_fraction_scales_the_tuple() {
        let tuple = Vector4::new(1.0, -2.0, 3.0, -4.0);
        let expected = Vector4::new(0.5, -1.0, 1.5, -2.0);

        assert_that!(tuple * 0.5).is_equal_to(expected);
    }

    #[test]
    fn unit_vector_x_has_magnitude_of_1() {
        let x = Vector4::vector(1.0, 0.0, 0.0);

        assert_that!(x.magnitude()).is_equal_to(1.0);
    }

    #[test]
    fn unit_vector_y_has_magnitude_of_1() {
        let y = Vector4::vector(0.0, 1.0, 0.0);

        assert_that!(y.magnitude()).is_equal_to(1.0);
    }

    #[test]
    fn unit_vector_z_has_magnitude_of_1() {
        let z = Vector4::vector(0.0, 0.0, 1.0);

        assert_that!(z.magnitude()).is_equal_to(1.0);
    }

    #[test]
    fn vector_has_magnitude() {
        let z = Vector4::vector(1.0, 2.0, 3.0);

        assert_that!(z.magnitude()).is_equal_to(14.0_f32.sqrt());
    }

    #[test]
    fn negative_vector_has_positive_magnitude() {
        let z = Vector4::vector(-1.0, -2.0, -3.0);

        assert_that!(z.magnitude()).is_equal_to(14.0_f32.sqrt());
    }

    #[test]
    fn normalizing_vector_returns_unit_vector_same_direction() {
        let x = Vector4::vector(4.0, 0.0, 0.0);
        let expected = Vector4::vector(1.0, 0.0, 0.0);

        assert_that!(x.normalize()).is_equal_to(expected);
    }

    #[test]
    fn normalizing_vector_returns_unit_vector_same_direction2() {
        let x = Vector4::vector(1.0, 2.0, 3.0);
        let expected = Vector4::vector(0.26726, 0.53452, 0.80178);

        let normalized = x.normalize();

        assert_that!(normalized.x).is_close_to(expected.x, 0.0001_f32);
        assert_that!(normalized.y).is_close_to(expected.y, 0.0001_f32);
        assert_that!(normalized.z).is_close_to(expected.z, 0.0001_f32);
        assert_that!(normalized.w).is_equal_to(expected.w);
    }

    #[test]
    fn normalized_vector_is_a_unit_vector() {
        let x = Vector4::vector(1.0, 2.0, 3.0);

        assert_that!(x.normalize().magnitude()).is_close_to(1.0, 0.0001_f32);
    }

    #[test]
    fn dot_product_of_two_vectors_is_scalar() {
        let a = Vector4::vector(1.0, 2.0, 3.0);
        let b = Vector4::vector(2.0, 3.0, 4.0);

        assert_that!(a.dot(&b)).is_equal_to(20.0);
    }

    #[test]
    fn cross_product_of_two_vectors_is_vector() {
        let a = Vector4::vector(1.0, 2.0, 3.0);
        let b = Vector4::vector(2.0, 3.0, 4.0);
        let expected = Vector4::vector(-1.0, 2.0, -1.0);

        assert_that!(a.cross_product(&b)).is_equal_to(expected);
    }

    #[test]
    fn cross_product_of_two_vectors_is_not_commutative() {
        let a = Vector4::vector(1.0, 2.0, 3.0);
        let b = Vector4::vector(2.0, 3.0, 4.0);
        let expected = Vector4::vector(1.0, -2.0, 1.0);

        assert_that!(b.cross_product(&a)).is_equal_to(expected);
    }

    #[test]
    fn tuple_can_be_indexed() {
        let a = Vector4::point(2.0, 3.0, 4.0);

        assert_that!(a[0]).is_equal_to(2.0);
        assert_that!(a[1]).is_equal_to(3.0);
        assert_that!(a[2]).is_equal_to(4.0);
        assert_that!(a[3]).is_equal_to(1.0);
    }

    #[test]
    fn tuple_can_be_indexed_mutably() {
        let mut a = Vector4::point(2.0, 3.0, 4.0);

        a[0] = 9.0;

        assert_that!(a[0]).is_equal_to(9.0);
        assert_that!(a[1]).is_equal_to(3.0);
        assert_that!(a[2]).is_equal_to(4.0);
        assert_that!(a[3]).is_equal_to(1.0);
    }
}
