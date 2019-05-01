use float_cmp::ApproxEq;
use std::f64;

pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64
}

impl Tuple {
    fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        return Tuple { x, y, z, w};
    }

    fn point(x: f64, y: f64, z: f64) -> Tuple {
        return Tuple::tuple(x, y, z, 1.0);
    }

    fn vector(x: f64, y: f64, z: f64) -> Tuple {
        return Tuple::tuple(x, y, z, 0.0);
    }

    fn eq(&self, other: &Tuple) -> bool {
        self.approx_eq(other, f64::EPSILON, 0)
    }
}

impl ApproxEq for Tuple {
    type Flt = f64;

    fn approx_eq(&self, other: &Tuple, epsilon: f64, ulps: i64) -> bool {
        (self.x == other.x &&
        self.y == other.y &&
        self.z == other.z &&
        self.w == other.w) ||
        (self.x.approx_eq(&other.x, epsilon, ulps) &&
        self.y.approx_eq(&other.y, epsilon, ulps) &&
        self.z.approx_eq(&other.z, epsilon, ulps) &&
        self.w.approx_eq(&other.w, epsilon, ulps))
    }
}


#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;
    use float_cmp::ApproxEq;
    use std::f64;


    #[test]
    fn test_approx_eq() {
        let t1 = Tuple::tuple(4.3, -4.2, 3.1, 1.0);
        let t2 = Tuple::tuple(4.3, -4.2, 3.1, 1.0);

        assert!(t1.eq(&t2));
    }
    #[test]
    fn test_tuple_as_point() {
        let t = Tuple::tuple(4.3, -4.2, 3.1, 1.0);
        assert!(t.x.approx_eq(&4.3, f64::EPSILON, 0));
        assert!(t.y.approx_eq(&-4.2, f64::EPSILON, 0));
        assert!(t.z.approx_eq(&3.1, f64::EPSILON, 0));
        assert!(t.w.approx_eq(&1.0, f64::EPSILON, 0));
    }

    #[test]
    fn test_tuple_as_vector() {
        let t = Tuple::tuple(4.3, -4.2, 3.1, 0.0);
        assert!(t.x.approx_eq(&4.3, f64::EPSILON, 0));
        assert!(t.y.approx_eq(&-4.2, f64::EPSILON, 0));
        assert!(t.z.approx_eq(&3.1, f64::EPSILON, 0));
        assert!(t.w.approx_eq(&0.0, f64::EPSILON, 0));
    }

    #[test]
    fn test_point_fn_creates_a_point() {
        let t = Tuple::point(4.3, -4.2, 3.1);
        assert!(t.x.approx_eq(&4.3, f64::EPSILON, 0));
        assert!(t.y.approx_eq(&-4.2, f64::EPSILON, 0));
        assert!(t.z.approx_eq(&3.1, f64::EPSILON, 0));
        assert!(t.w.approx_eq(&1.0, f64::EPSILON, 0));
    }

    #[test]
    fn test_vector_fn_creates_a_vector() {
        let t = Tuple::vector(4.3, -4.2, 3.1);
        assert!(t.x.approx_eq(&4.3, f64::EPSILON, 0));
        assert!(t.y.approx_eq(&-4.2, f64::EPSILON, 0));
        assert!(t.z.approx_eq(&3.1, f64::EPSILON, 0));
        assert!(t.w.approx_eq(&0.0, f64::EPSILON, 0));
    }
}
