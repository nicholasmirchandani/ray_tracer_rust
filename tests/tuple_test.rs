mod test_hack;
#[path = "../src/fast_tuple.rs"] mod fast_tuple;

#[cfg(test)]
mod tuple_test {

        // Import the necessary modules.
        use crate::test_hack::TEST_PASSED;
        use crate::test_hack::expect_eq;
        use crate::fast_tuple;

        // Ensure that FastTuple works as expected
        #[test]
        fn test_fast_tuple() {
            unsafe { TEST_PASSED = true }; // Hack to initialize TEST_PASSED state.
            let tuple = fast_tuple::FastTuple { x: 3.3, y: 4.4, z: 5.5, w:1.0};
            assert!(true);
            expect_eq!(tuple.x, 3.3);   
            expect_eq!(tuple.y, 4.4);   
            expect_eq!(tuple.z, 5.5);
            expect_eq!(tuple.w, 1.0);   
            expect_eq!(tuple.is_point(), true);
            expect_eq!(tuple.is_vector(), false);
            unsafe { assert!(TEST_PASSED) }; // Hack to check TEST_PASSED state.
        }
}
