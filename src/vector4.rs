const EPSILON: f32 = 0.001;

#[derive(Debug, Clone, Copy)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl PartialEq for Vector4 {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
            && (self.w - other.w).abs() < EPSILON
    }
}

impl std::ops::Add for Vector4 {
    type Output = Vector4;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl std::ops::Sub for Vector4 {
    type Output = Vector4;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl std::ops::Neg for Vector4 {
    type Output = Vector4;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl std::ops::Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, scale: f32) -> Self {
        Self {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
            w: self.w * scale,
        }
    }
}

impl std::ops::Div<f32> for Vector4 {
    type Output = Vector4;

    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl std::ops::Index<usize> for Vector4 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Not a valid index"),
        }
    }
}

impl std::ops::IndexMut<usize> for Vector4 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Not a valid index"),
        }
    }
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn cross_product(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: 0.0,
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn is_point(&self) -> bool {
        self.w.round() == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w.round() == 0.0
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - *normal * 2_f32 * self.dot(normal)
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        Self {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
            z: self.z / self.magnitude(),
            w: self.w / self.magnitude(),
        }
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

        let result = point + vector;

        assert_that!(result).is_equal_to(expected);
    }

    #[test]
    fn subtracting_two_points_returns_a_vector() {
        let point1 = Vector4::point(3.0, 2.0, 1.0);
        let point2 = Vector4::point(5.0, 6.0, 7.0);
        let expected = Vector4::vector(-2.0, -4.0, -6.0);

        let result = point1 - point2;

        assert_that!(result).is_equal_to(expected)
    }

    #[test]
    fn subtracting_a_vector_from_a_point_returns_a_new_point() {
        let point = Vector4::point(3.0, 2.0, 1.0);
        let vector = Vector4::vector(5.0, 6.0, 7.0);
        let expected = Vector4::point(-2.0, -4.0, -6.0);

        let result = point - vector;

        assert_that!(result).is_equal_to(expected)
    }

    #[test]
    fn negating_a_vector_returns_the_negative_vector() {
        let vector = Vector4::vector(1.0, -2.0, 3.0);
        let expected = Vector4::vector(-1.0, 2.0, -3.0);

        let result = -vector;

        assert_that!(result).is_equal_to(expected);
    }

    #[test]
    fn negating_a_tuple_returns_the_negative_tuple() {
        let tuple = Vector4::new(1.0, -2.0, 3.0, -4.0);
        let expected = Vector4::new(-1.0, 2.0, -3.0, 4.0);

        let result = -tuple;

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

        assert_that!(normalized.x).is_close_to(expected.x, 0.0001);
        assert_that!(normalized.y).is_close_to(expected.y, 0.0001);
        assert_that!(normalized.z).is_close_to(expected.z, 0.0001);
        assert_that!(normalized.w).is_equal_to(expected.w);
    }

    #[test]
    fn normalized_vector_is_a_unit_vector() {
        let x = Vector4::vector(1.0, 2.0, 3.0);

        assert_that!(x.normalize().magnitude()).is_close_to(1.0, 0.0001);
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

    #[test]
    fn reflecting_a_vector_approaching_at_45_deg() {
        let v = Vector4::vector(1.0, -1.0, 0.0);
        let n = Vector4::vector(0.0, 1.0, 0.0);

        let r = v.reflect(&n);

        assert_that!(r).is_equal_to(Vector4::vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_a_vector_off_a_slanted_surface() {
        let v = Vector4::vector(0.0, -1.0, 0.0);
        let n = Vector4::vector(2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0, 0.0);

        let r = v.reflect(&n);

        let expected = Vector4::vector(1.0, 0.0, 0.0);
        assert_that!(r.x).is_close_to(expected.x, 0.0001);
        assert_that!(r.y).is_close_to(expected.y, 0.0001);
        assert_that!(r.z).is_close_to(expected.z, 0.0001);
        assert_that!(r.w).is_equal_to(expected.w);
    }
}
