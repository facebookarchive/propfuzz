// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Runtime support for propfuzz.

use crate::Propfuzz;
use proptest::test_runner::{TestError, TestRunner};
use std::fmt;

/// Executes a propfuzz target as a standard property-based test.
pub fn execute_as_proptest(fuzz_target: impl Propfuzz) {
    let mut config = fuzz_target.proptest_config();
    config.test_name = Some(fuzz_target.name());

    let mut test_runner = TestRunner::new(config);
    match fuzz_target.execute(&mut test_runner) {
        Ok(()) => (),
        Err(err) => panic!(
            "{}{}",
            TestErrorDisplay::new(&fuzz_target, err),
            test_runner
        ),
    }
}

struct TestErrorDisplay<'a, PF, T> {
    fuzz_target: &'a PF,
    err: TestError<T>,
}

impl<'a, PF, T> TestErrorDisplay<'a, PF, T> {
    fn new(fuzz_target: &'a PF, err: TestError<T>) -> Self {
        Self { fuzz_target, err }
    }
}

impl<'a, PF, T> fmt::Display for TestErrorDisplay<'a, PF, T>
where
    PF: Propfuzz<Value = T>,
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.err {
            TestError::Abort(why) => write!(f, "Test aborted: {}", why),
            TestError::Fail(why, what) => {
                writeln!(f, "Test failed: {}\nminimal failing input:", why)?;
                self.fuzz_target.fmt_value(&what, f)
            }
        }
    }
}
