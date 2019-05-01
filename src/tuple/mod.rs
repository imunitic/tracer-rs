
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64
}

fn point(x: f64, y: f64, z: f64) -> Tuple {
    return Tuple { x: x, y: y, z: z, w: 1.0};
}

fn vector(x: f64, y: f64, z: f64) -> Tuple {
    return Tuple {x: x, y: y, z: z, w: 0.0};
}

#[cfg(test)]
mod tests {
    use crate::tuple::vector;
    use crate::tuple::point;
    use crate::tuple::Tuple;

    #[test]
    fn test_tuple_as_point() {
        let t = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 1.0};
        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
        assert_eq!(t.w, 1.0);
    }

    #[test]
    fn test_tuple_as_vector() {
        let t = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 0.0};
        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
        assert_eq!(t.w, 0.0);
    }

    #[test]
    fn test_point_fn_creates_a_point() {
        let t = point(4.3, -4.2, 3.1);
        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
        assert_eq!(t.w, 1.0);
    }

    #[test]
    fn test_vector_fn_creates_a_vector() {
        let t = vector(4.3, -4.2, 3.1);
        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
        assert_eq!(t.w, 0.0);
    }
}
