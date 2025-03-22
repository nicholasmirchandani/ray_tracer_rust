mod test_hack;
#[path = "../src/fast_tuple.rs"] mod fast_tuple;

#[cfg(test)]
mod tuple_test {

        // Import the necessary modules.
        use crate::test_hack::expect_eq;
        use crate::test_hack::expect_float_eq;
        use crate::test_hack::setup;
        use crate::test_hack::teardown;
        use crate::fast_tuple::FastTuple;

        // Ensure that FastTuple works as expected for points (w = 1.0)
        #[test]
        fn test_point_tuple() {
            unsafe { setup() };
            let tuple = FastTuple { x: 4.3, y: -4.2, z: 3.1, w:1.0};
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
            let tuple = FastTuple { x: 4.3, y: -4.2, z: 3.1, w:1.0};
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
}
