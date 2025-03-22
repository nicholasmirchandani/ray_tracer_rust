mod test_hack;

#[cfg(test)]
mod tests {
    // Import the necessary modules.
    use crate::test_hack::TEST_PASSED;
    use crate::test_hack::expect_eq;
    use crate::test_hack::setup;
    use crate::test_hack::teardown;

    // This test is a sanity check to show I know what I'm doing
    #[test]
    fn test_something() {
        unsafe { setup() };
        expect_eq!(3, 3);    
        unsafe { teardown() };
    }

    // This test is a sanity check to ensure failing things fail, and that the fail is a non fatal assertion
    #[test]
    fn test_fail(){
        unsafe { setup() };
        expect_eq!(2+2, 3);   
        expect_eq!(3, 3);   
        expect_eq!(2+7, 3);   
        unsafe { assert!(!TEST_PASSED) }; // Note we can't use teardown() here, since this is actually expecting the failure.  
    }

}