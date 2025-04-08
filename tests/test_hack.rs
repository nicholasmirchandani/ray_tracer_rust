// Note that this is kind of a mega sin, sidestepping all of Rust's strict guarantees...
// Tests can run in parallel which can break this entirely.
// So we have to force them to run on a single thread to prevent any edge cases, while supporting my non fatal test failures.

// cargo test -- --test-threads=1 (--show-output if desired)

pub static mut TEST_PASSED: bool = true;

// A non-fatal assertion for unit tests (allows test to run to completion, but logs error).  Requires teardown() to be called (or TEST_PASSED to be checked directly)
macro_rules! expect_eq {
    ($a:expr, $b:expr) => {
        if $a != $b {
            println!(
                "Test Failed: {} not equal to {}.  [{}:{}]",
                $a,
                $b,
                module_path!(),
                line!()
            );
        }
        unsafe {
            crate::test_hack::TEST_PASSED = crate::test_hack::TEST_PASSED && ($a == $b);
        }
    };
}

pub static EPSILON: f32 = 0.00001;

// A non-fatal assertion for unit tests, using floating point comparison.  Requires teardown() to be called (or TEST_PASSED to be checked directly)
macro_rules! expect_float_eq {
    ($a:expr, $b:expr) => {
        let val = ($a - $b);
        let passed = (val.abs() < crate::test_hack::EPSILON);
        if !passed {
            println!(
                "Test Failed: {} not equal (per floating point comparison) to {}.  [{}:{}]",
                $a,
                $b,
                module_path!(),
                line!()
            );
        }
        unsafe {
            crate::test_hack::TEST_PASSED = crate::test_hack::TEST_PASSED && passed;
        }
    };
}

// Note: can probably replace setup and teardown with a procedural attribute macro and be all fancy.  I just wanna write a ray tracer though ;-;

// Meant to be called at the start of every test.
pub unsafe fn setup() {
    unsafe { TEST_PASSED = true }; // Hack to initialize TEST_PASSED state.
}

// Meant to be called at the end of every test.  Note that if a fatal assertion fails, teardown may never run.
pub unsafe fn teardown() {
    unsafe { assert!(TEST_PASSED) }; // Hack to check TEST_PASSED state.\
}

pub(crate) use expect_eq;
pub(crate) use expect_float_eq;
