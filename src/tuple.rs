use float_cmp::{ApproxEq, F64Margin, approx_eq};
use std::f64;
use std::ops::{Add, Sub, Neg, Mul, Div};

#[derive(Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    w: f64
}

impl Tuple {
    const EPSILON: f64 = 0.00001_f64;

    fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w}
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::tuple(x, y, z, 1.0)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::tuple(x, y, z, 0.0)
    }

    pub fn eq(self, other: Tuple) -> bool {
        approx_eq!(Self, self, other, epsilon=Tuple::EPSILON)
    }

    pub fn magnitude(self) -> f64 {
        (self.x.powf(2.0) +
         self.y.powf(2.0) +
         self.z.powf(2.0) +
         self.w.powf(2.0)).sqrt()
    }

    pub fn normalize(self) -> Tuple {
        Tuple::tuple(self.x / self.magnitude(),
                     self.y / self.magnitude(),
                     self.z / self.magnitude(),
                     self.w / self.magnitude())
    }

    pub fn dot(self, other: Tuple) -> f64 {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z +
        self.w * other.w
    }

    pub fn cross(self, other: Tuple) -> Tuple {
        Tuple::vector(self.y * other.z - self.z * other.y,
                      self.z * other.x - self.x * other.z,
                      self.x * other.y - self.y * other.x)
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple::tuple(self.x + other.x,
                     self.y + other.y,
                     self.z + other.z,
                     self.w + other.w)
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple::tuple(self.x - other.x,
                     self.y - other.y,
                     self.z - other.z,
                     self.w - other.w)
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple::tuple(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self: Tuple, other: f64) -> Tuple {
        Tuple::tuple(self.x * other,
            self.y * other,
            self.z * other,
            self.w * other)
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self: Tuple, other: f64) -> Tuple {
        Tuple::tuple(self.x / other,
                     self.y / other,
                     self.z / other,
                     self.w / other)
    }
}

impl ApproxEq for Tuple {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();
        (self.x == other.x &&
        self.y == other.y &&
        self.z == other.z &&
        self.w == other.w) ||
        (self.x.approx_eq(other.x, margin) &&
        self.y.approx_eq(other.y, margin) &&
        self.z.approx_eq(other.z, margin) &&
        self.w.approx_eq(other.w, margin))
    }
}


#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;
    use std::f64;


    #[test]
    fn test_approx_eq() {
        let t1 = Tuple::tuple(4.3, -4.2, 3.1, 1.0);
        let t2 = Tuple::tuple(4.3, -4.2, 3.1, 1.0);

        assert!(t1.eq(t2));
    }
    #[test]
    fn test_tuple_as_point() {
        let t = Tuple::tuple(4.3, -4.2, 3.1, 1.0);
        assert!(t.x.eq(&4.3));
        assert!(t.y.eq(&-4.2));
        assert!(t.z.eq(&3.1));
        assert!(t.w.eq(&1.0));
    }

    #[test]
    fn test_tuple_as_vector() {
        let t = Tuple::tuple(4.3, -4.2, 3.1, 0.0);
        assert!(t.x.eq(&4.3));
        assert!(t.y.eq(&-4.2));
        assert!(t.z.eq(&3.1));
        assert!(t.w.eq(&0.0));
    }

    #[test]
    fn test_point_fn_creates_a_point() {
        let t = Tuple::point(4.3, -4.2, 3.1);
        assert!(t.x.eq(&4.3));
        assert!(t.y.eq(&-4.2));
        assert!(t.z.eq(&3.1));
        assert!(t.w.eq(&1.0));
    }

    #[test]
    fn test_vector_fn_creates_a_vector() {
        let t = Tuple::vector(4.3, -4.2, 3.1);
        assert!(t.x.eq(&4.3));
        assert!(t.y.eq(&-4.2));
        assert!(t.z.eq(&3.1));
        assert!(t.w.eq(&0.0));
    }

    #[test]
    fn test_tuple_addition() {
        let t1 = Tuple::tuple(3.0, -2.0, 5.0, 1.0);
        let t2 = Tuple::tuple(-2.0, 3.0, 1.0, 0.0);
        let result = t1 + t2;
        let expected = Tuple::tuple(1.0, 1.0, 6.0, 1.0);
        assert!(result.eq(expected));
    }

    #[test]
    fn test_sub_two_points() {
        let t1 = Tuple::point(3.0, 2.0, 1.0);
        let t2 = Tuple::point(5.0, 6.0, 7.0);
        let result = t1 - t2;
        let expected = Tuple::vector(-2.0, -4.0, -6.0);

        assert!(result.eq(expected));
    }

    #[test]
    fn test_sub_vector_from_point() {
        let t1 = Tuple::point(3.0, 2.0, 1.0);
        let t2 = Tuple::vector(5.0, 6.0, 7.0);
        let result = t1 - t2;
        let expected = Tuple::point(-2.0, -4.0, -6.0);

        assert!(result.eq(expected));
    }

    #[test]
    fn test_sub_two_vectors() {
        let t1 = Tuple::vector(3.0, 2.0, 1.0);
        let t2 = Tuple::vector(5.0, 6.0, 7.0);
        let result = t1 - t2;
        let expected = Tuple::vector(-2.0, -4.0, -6.0);

        assert!(result.eq(expected));
    }

    #[test]
    fn test_neg_of_tuple() {
        let t = Tuple::tuple(1.0, -2.0, 3.0, -4.0);
        let result = -t;
        let expected = Tuple::tuple(-1.0, 2.0, -3.0, 4.0);

        assert!(result.eq(expected));
    }

    #[test]
    fn test_multiplication_by_scalar() {
        let t = Tuple::tuple(1.0, -2.0, 3.0, -4.0);
        let result = t * 3.5;
        let expected = Tuple::tuple(3.5, -7.0, 10.5, -14.0);

        assert!(result.eq(expected));
    }

    #[test]
    fn test_multiplication_by_fraction() {
        let t = Tuple::tuple(1.0, -2.0, 3.0, -4.0);
        let result = t * 0.5;
        let expected = Tuple::tuple(0.5, -1.0, 1.5, -2.0);

        assert!(result.eq(expected));
    }

    #[test]
    fn test_division_by_scalar() {
        let t = Tuple::tuple(1.0, -2.0, 3.0, -4.0);
        let result = t / 2.0;
        let expected = Tuple::tuple(0.5, -1.0, 1.5, -2.0);

        assert!(result.eq(expected));
    }

    #[test]
    fn test_magnitude_of_tuple() {
        let t1 = Tuple::vector(1.0, 0.0, 0.0);
        assert!(t1.magnitude().eq(&1.0));

        let t2 = Tuple::vector(0.0, 1.0, 0.0);
        assert!(t2.magnitude().eq(&1.0));

        let t3 = Tuple::vector(0.0, 0.0, 1.0);
        assert!(t3.magnitude().eq(&1.0));

        let t4 = Tuple::vector(1.0, 2.0, 3.0);
        let r4 = (14.0 as f64).sqrt();
        assert!(t4.magnitude().eq(&r4));

        let t5 = Tuple::vector(-1.0, -2.0, -3.0);
        let r5 = (14.0 as f64).sqrt();
        assert!(t5.magnitude().eq(&r5));
    }

    #[test]
    fn test_normalize_tuple() {
        let t1 = Tuple::vector(4.0, 0.0, 0.0);
        let result = t1.normalize();
        let expected = Tuple::vector(1.0, 0.0, 0.0);

        assert!(result.eq(expected));

        let t2 = Tuple::vector(1.0, 2.0, 3.0);
        let result2 = t2.normalize();
        let expected2 = Tuple::vector(0.26726, 0.53452, 0.80178);

        assert!(result2.eq(expected2))
    }

    #[test]
    fn test_dot_product() {
        let t1 = Tuple::vector(1.0, 2.0, 3.0);
        let t2 = Tuple::vector(2.0, 3.0, 4.0);
        let result = t1.dot(t2);
        let expected = 20.0;

        assert!(result.eq(&expected));
    }

    #[test]
    fn test_cross_product() {
        let t1 = Tuple::vector(1.0, 2.0, 3.0);
        let t2 = Tuple::vector(2.0, 3.0, 4.0);

        let result1 = t1.cross(t2);
        let expected1 = Tuple::vector(-1.0, 2.0, -1.0);

        assert!(result1.eq(expected1));

        let result2 = t2.cross(t1);
        let expected2 = Tuple::vector(1.0, -2.0, 1.0);

        assert!(result2.eq(expected2));
    }
}
