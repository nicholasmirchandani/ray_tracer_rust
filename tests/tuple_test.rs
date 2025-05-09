#[path = "../src/fast_tuple.rs"]
mod fast_tuple;
mod test_hack;

#[cfg(test)]
mod tuple_test {

    // Import the necessary modules.
    use crate::fast_tuple::FastTuple;
    use crate::test_hack::expect_eq;
    use crate::test_hack::expect_float_eq;
    use crate::test_hack::setup;
    use crate::test_hack::teardown;

    // Ensure that FastTuple works as expected for points (w = 1.0)
    #[test]
    fn test_point_tuple() {
        unsafe { setup() };
        let tuple = FastTuple::new(4.3, -4.2, 3.1, 1.0);
        expect_float_eq!(tuple.x, 4.3);
        expect_float_eq!(tuple.y, -4.2);
        expect_float_eq!(tuple.z, 3.1);
        expect_eq!(tuple.w, 1.0);
        expect_eq!(tuple.is_point(), true);
        expect_eq!(tuple.is_vector(), false);
        unsafe { teardown() };
    }

    // Ensure that FastTuple works as expected for vector (w = 0.0)
    #[test]
    fn test_vector_tuple() {
        unsafe { setup() };
        let tuple = FastTuple::new(4.3, -4.2, 3.1, 1.0);
        expect_float_eq!(tuple.x, 4.3);
        expect_float_eq!(tuple.y, -4.2);
        expect_float_eq!(tuple.z, 3.1);
        expect_eq!(tuple.w, 1.0);
        expect_eq!(tuple.is_point(), true);
        expect_eq!(tuple.is_vector(), false);
        unsafe { teardown() };
    }

    // Ensure that we can create a point as a FastTuple.
    #[test]
    fn test_create_point() {
        unsafe { setup() };
        let point = FastTuple::point(4.0, -4.0, 3.0);
        expect_float_eq!(point.x, 4.0);
        expect_float_eq!(point.y, -4.0);
        expect_float_eq!(point.z, 3.0);
        expect_eq!(point.w, 1.0);
        expect_eq!(point.is_point(), true);
        expect_eq!(point.is_vector(), false);
        unsafe { teardown() };
    }

    // Ensure that we can create a vector as a FastTuple.
    #[test]
    fn test_create_vector() {
        unsafe { setup() };
        let vector = FastTuple::vector(4.0, -4.0, 3.0);
        expect_float_eq!(vector.x, 4.0);
        expect_float_eq!(vector.y, -4.0);
        expect_float_eq!(vector.z, 3.0);
        expect_eq!(vector.w, 0.0);
        expect_eq!(vector.is_point(), false);
        expect_eq!(vector.is_vector(), true);
        unsafe { teardown() };
    }

    // Ensure that tuple addition works.
    #[test]
    fn test_tuple_addition() {
        unsafe { setup() };
        let a1 = FastTuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = FastTuple::new(-2.0, 3.0, 1.0, 0.0);
        let expected = FastTuple::new(1.0, 1.0, 6.0, 1.0);
        expect_eq!(a1 + a2, expected);

        unsafe { teardown() };
    }

    // Ensure that point-vector subtraction works.
    #[test]
    fn test_vector_from_point_subtraction() {
        unsafe { setup() };
        let p1 = FastTuple::point(3.0, 2.0, 1.0);
        let p2 = FastTuple::vector(5.0, 6.0, 7.0);
        let expected = FastTuple::point(-2.0, -4.0, -6.0);
        expect_eq!(p1 - p2, expected);

        unsafe { teardown() };
    }

    // Ensure that vector subtraction works.
    #[test]
    fn test_vector_subtraction() {
        unsafe { setup() };
        let p1 = FastTuple::vector(3.0, 2.0, 1.0);
        let p2 = FastTuple::vector(5.0, 6.0, 7.0);
        let expected = FastTuple::vector(-2.0, -4.0, -6.0);
        expect_eq!(p1 - p2, expected);

        unsafe { teardown() };
    }

    // Vector subtraction from zero.
    #[test]
    fn test_vector_from_zero_subtraction() {
        unsafe { setup() };
        let zero = FastTuple::vector(0.0, 0.0, 0.0);
        let v1 = FastTuple::vector(1.0, -2.0, 3.0);
        expect_eq!(zero - v1, FastTuple::vector(-1.0, 2.0, -3.0));
        unsafe { teardown() };
    }

    // Ensure that vector negation works.
    #[test]
    fn test_vector_negation() {
        unsafe { setup() };
        let a = FastTuple::new(1.0, -2.0, 3.0, -4.0);
        expect_eq!(-a, FastTuple::new(-1.0, 2.0, -3.0, 4.0));
        unsafe { teardown() };
    }

    // Ensure scalar multiplication works.
    #[test]
    fn test_scalar_multiplication() {
        unsafe { setup() };
        let a = FastTuple::new(1.0, -2.0, 3.0, -4.0);
        expect_eq!(a * 3.5, FastTuple::new(3.5, -7.0, 10.5, -14.0));
        expect_eq!(a * 0.5, FastTuple::new(0.5, -1.0, 1.5, -2.0));
        unsafe { teardown() };
    }

    // Ensure scalar division works.
    #[test]
    fn test_scalar_division() {
        unsafe { setup() };
        let a = FastTuple::new(1.0, -2.0, 3.0, -4.0);
        expect_eq!(a / 2.0, FastTuple::new(0.5, -1.0, 1.5, -2.0));
        unsafe { teardown() };
    }

    // Ensure vector magnitude works as expected.
    #[test]
    fn test_vector_magnitude() {
        unsafe { setup() };
        let v1 = FastTuple::vector(1.0, 0.0, 0.0);
        expect_float_eq!(v1.magnitude(), 1.0);
        let v2 = FastTuple::vector(0.0, 1.0, 0.0);
        expect_float_eq!(v2.magnitude(), 1.0);
        let v3 = FastTuple::vector(0.0, 0.0, 1.0);
        expect_float_eq!(v3.magnitude(), 1.0);
        let v4 = FastTuple::vector(1.0, 2.0, 3.0);
        expect_float_eq!(v4.magnitude(), (14.0_f32).sqrt());
        let v5 = FastTuple::vector(-1.0, -2.0, -3.0);
        expect_float_eq!(v5.magnitude(), (14.0_f32).sqrt());
        unsafe { teardown() };
    }

    // Ensure normalization works as expected.
    #[test]
    fn test_normalization() {
        unsafe { setup() };
        let v1 = FastTuple::vector(4.0, 0.0, 0.0);
        expect_eq!(FastTuple::normalize(v1), FastTuple::vector(1.0, 0.0, 0.0));

        let v2 = FastTuple::vector(1.0, 2.0, 3.0);
        expect_eq!(
            FastTuple::normalize(v2),
            FastTuple::vector(0.26726, 0.53452, 0.80178)
        );
        unsafe { teardown() };
    }

    // Ensure dot product works as expected.
    #[test]
    fn test_dot_product() {
        unsafe { setup() };
        let a = FastTuple::vector(1.0, 2.0, 3.0);
        let b = FastTuple::vector(2.0, 3.0, 4.0);
        expect_float_eq!(FastTuple::dot(a, b), 20.0);

        unsafe { teardown() };
    }

    // Ensure cross product works as expected.
    #[test]
    fn test_cross_product() {
        unsafe { setup() };
        let a = FastTuple::vector(1.0, 2.0, 3.0);
        let b = FastTuple::vector(2.0, 3.0, 4.0);

        expect_eq!(FastTuple::cross(a, b), FastTuple::vector(-1.0, 2.0, -1.0));
        expect_eq!(FastTuple::cross(b, a), FastTuple::vector(1.0, -2.0, 1.0));
        unsafe { teardown() };
    }
}
