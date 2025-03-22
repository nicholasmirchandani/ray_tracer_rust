mod test_hack;

#[cfg(test)]
mod tests {
    // Import the necessary modules.
    use crate::test_hack::TEST_PASSED;
    use crate::test_hack::expect_eq;

    // This test is a sanity check to show I know what I'm doing
    #[test]
    fn test_something() {
        unsafe { TEST_PASSED = true }; // Hack to initialize TEST_PASSED state.
        expect_eq!(3, 3);   
        unsafe { assert!(TEST_PASSED) }; // Hack to check TEST_PASSED state.
    }

    // This test is a sanity check to ensure failing things fail, and that the fail is a non fatal assertion
    #[test]
    fn test_fail(){
        unsafe { TEST_PASSED = true }; // Hack to initialize TEST_PASSED state.
        expect_eq!(2+2, 3);   
        expect_eq!(3, 3);   
        expect_eq!(2+7, 3);   
        unsafe { assert!(!TEST_PASSED) }; // Hack to TEST_PASSED fails.  
    }

}