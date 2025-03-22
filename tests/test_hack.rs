// Note that this is kind of a mega sin, sidestepping all of Rust's strict guarantees...
// Tests can run in parallel which can break this entirely.
// So we have to force them to run on a single thread to prevent any edge cases, while supporting my non fatal test failures.

// cargo test -- --test-threads=1 (--show-output if desired)

pub static mut TEST_PASSED : bool = true;

macro_rules! expect_eq {
    ($a:expr, $b:expr) => {
        if $a != $b {
            println!("Test Failed: {} not equal to {}.  [{}:{}]", $a, $b, module_path!(), line!());
        }
        unsafe { TEST_PASSED = TEST_PASSED && ($a == $b); } 
    };
}

pub(crate) use expect_eq;