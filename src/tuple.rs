// FIXME: Import utils ?!?
// The 'Token' struct is used to represent vectors, points, and colors.
// TODO: Document the public API
#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {x, y, z, w: 1f64}
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {x, y, z, w: 0f64}
    }

    pub fn is_point(&self) -> bool {
        super::utils::approx_eq(self.w, 1f64)
    }

    pub fn is_vector(&self) -> bool {
        super::utils::approx_eq(self.w, 0f64)
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let mag = self.magnitude();
        Tuple { x: self.x / mag, y: self.y / mag, z: self.z / mag, w: self.w }
    }

    pub fn dot_product(&a: &Tuple, &b: &Tuple) -> f64 {
        a.x * b.x +
        a.y * b.y +
        a.z * b.z
    }

    pub fn cross_product(&a: &Tuple, &b: &Tuple) -> Tuple {
        Tuple::vector(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x
        )
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        super::utils::approx_eq(self.x, other.x) &&
        super::utils::approx_eq(self.y, other.y) &&
        super::utils::approx_eq(self.z, other.z) &&
        super::utils::approx_eq(self.w, other.w)
    }
}

impl std::ops::Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // Invalid operation (like adding 2 points) are not handled.
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl std::ops::Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        // Invalid operations (like sub a point from a vector) are not handled
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl std::ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        }
    }
}

impl std::ops::Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, factor: f64) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
            w: self.w
        }
    }
}

impl std::ops::Div<f64> for Tuple {
    type Output = Self;

    fn div(self, factor: f64) -> Self {
        Self {
            x: self.x / factor,
            y: self.y / factor,
            z: self.z / factor,
            w: self.w
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_creates_a_point() {
        let p = Tuple::point(4.3, -4.2, 3.1);

        assert!(p.is_point());
        assert!(!p.is_vector());

        assert_eq!(p.x, 4.3);
        assert_eq!(p.y, -4.2);
        assert_eq!(p.z, 3.1);
    }

    #[test]
    fn vector_creates_a_vector() {
        let t = Tuple::vector(4f64, -4f64, 3f64);

        assert!(t.is_vector());
        assert!(!t.is_point());

        assert_eq!(t.x, 4.0);
        assert_eq!(t.y, -4.0);
        assert_eq!(t.z, 3.0);
    }

    #[test]
    fn tuple_is_equal() {
        let t1 = Tuple::vector(4f64, -4f64, 3f64);
        let t2 = Tuple::vector(4f64, -4f64, 3f64);

        assert_eq!(t1, t2);
    }

    #[test]
    fn tuple_is_different() {
        let t1 = Tuple::vector(4f64, -4f64, 3f64);
        let t2 = Tuple::vector(4.01, -4f64, 3f64);
        let t3 = Tuple::vector(4f64, -4.01, 3f64);
        let t4 = Tuple::vector(4f64, -4f64, 2.999);
        let p1 = Tuple::point(4f64, -4f64, 3f64);

        assert_ne!(t1, t2);
        assert_ne!(t1, t3);
        assert_ne!(t1, t4);
        assert_ne!(t1, p1);
    }

    #[test]
    fn adding_a_vector_to_a_point() {
        // Returns a new point
        let p = Tuple::point(3f64, -2f64, 5f64);
        let v = Tuple::vector(-2f64, 3f64, 1f64);

        assert_eq!(p + v, Tuple::point(1f64, 1f64, 6f64));
    }

    #[test]
    fn adding_a_vector_to_a_vector() {
        // Returns a new vector
        let v1 = Tuple::vector(3f64, -2f64, 5f64);
        let v2 = Tuple::vector(-2f64, 3f64, 1f64);

        assert_eq!(v1 + v2, Tuple::vector(1f64, 1f64, 6f64));
    }

    #[test]
    fn subtracting_two_points() {
        // Result is a vector (delta between 2 points)
        let p1 = Tuple::point(3f64, 2f64, 1f64);
        let p2 = Tuple::point(5f64, 6f64, 7f64);

        assert_eq!(p1 - p2, Tuple::vector(-2f64, -4f64, -6f64));
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        // Returns a new point
        let p = Tuple::point(3f64, 2f64, 1f64);
        let v = Tuple::vector(5f64, 6f64, 7f64);

        assert_eq!(p - v, Tuple::point(-2f64, -4f64, -6f64));
    }

    #[test]
    fn subtracting_a_vector_from_a_vector() {
        // Returns a vector representing the change of direction between the two
        let v1 = Tuple::vector(3f64, 2f64, 1f64);
        let v2 = Tuple::vector(5f64, 6f64, 7f64);

        assert_eq!(v1 - v2, Tuple::vector(-2f64, -4f64, -6f64));
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        // Note: This is the same as negating a vector
        let zero = Tuple::vector(0f64, 0f64, 0f64);
        let v = Tuple::vector(1f64, -2f64, 3f64);

        assert_eq!(zero - v, Tuple::vector(-1f64, 2f64, -3f64));
    }

    #[test]
    fn negating_a_vector() {
        let v = Tuple::vector(1f64, -2f64, 3f64);

        assert_eq!(-v, Tuple::vector(-1f64, 2f64, -3f64));
    }

    #[test]
    fn multiplying_a_vector_by_a_scalar() {
        let v = Tuple::vector(1.0, -2.0, 3.0);
        assert_eq!(v * 3.5, Tuple::vector(3.5, -7.0, 10.5));
    }

    #[test]
    fn multiplying_a_vector_by_a_fraction() {
        let v = Tuple::vector(1.0, -2.0, 3.0);
        assert_eq!(v * 0.5, Tuple::vector(0.5, -1.0, 1.5));
    }

    #[test]
    fn dividing_a_vector_by_a_scalar() {
        let v = Tuple::vector(1.0, -2.0, 3.0);
        assert_eq!(v / 2.0, Tuple::vector(0.5, -1.0, 1.5));
    }

    #[test]
    fn computing_magnitude_of_vector_1_0_0() {
        let v = Tuple::vector(1f64, 0f64, 0f64);

        assert_eq!(1f64, v.magnitude());
    }

    #[test]
    fn computing_magnitude_of_vector_0_1_0() {
        let v = Tuple::vector(0f64, 1f64, 0f64);

        assert_eq!(1f64, v.magnitude());
    }

    #[test]
    fn computing_magnitude_of_vector_0_0_1() {
        let v = Tuple::vector(0f64, 0f64, 1f64);

        assert_eq!(1f64, v.magnitude());
    }

    #[test]
    fn computing_magnitude_of_vector_1_2_3() {
        let v = Tuple::vector(1f64, 2f64, 3f64);

        assert_eq!(14f64.sqrt(), v.magnitude());
    }

    #[test]
    fn computing_magnitude_of_negative_vector_1_2_3() {
        let v = Tuple::vector(-1f64, -2f64, -3f64);

        assert_eq!(14f64.sqrt(), v.magnitude());
    }

    #[test]
    fn normalize_vector_4_0_0() {
        let v = Tuple::vector(4.0, 0.0, 0.0);

        assert_eq!(v.normalize(), Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalize_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        let divider = 14f64.sqrt();
        assert_eq!(v.normalize(), Tuple::vector(1.0 / divider, 2.0 / divider, 3.0 / divider));
    }

    #[test]
    fn magnitude_of_a_normalized_vector() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        assert_eq!(1.0, v.normalize().magnitude());
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(20.0, Tuple::dot_product(&v1, &v2));
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(Tuple::vector(-1.0, 2.0, -1.0), Tuple::cross_product(&v1, &v2));
        assert_eq!(Tuple::vector(1.0, -2.0, 1.0), Tuple::cross_product(&v2, &v1));
    }
}
