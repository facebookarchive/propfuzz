// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Runtime support for propfuzz.

use crate::Propfuzz;
use proptest::prelude::*;
use proptest::test_runner::{TestCaseResult, TestRunner};

/// Executes a propfuzz target as a standard property-based test.
pub fn execute_as_proptest(propfuzz: &dyn Propfuzz) {
    let mut config = propfuzz.proptest_config();
    config.test_name = Some(propfuzz.name());

    let mut test_runner = TestRunner::new(config);
    // TODO: is this the right level of abstraction?
    propfuzz.execute(&mut test_runner);
}

pub fn propfuzz_outer<S: Strategy>(
    test_runner: &mut TestRunner,
    strategy: &S,
    test: impl Fn(S::Value) -> TestCaseResult,
) {
    match test_runner.run(strategy, test) {
        Ok(()) => (),
        Err(e) => panic!("{}\n{}", e, test_runner),
    }
}

// Re-export NamedArguments from proptest::sugar.
// TODO: implement our own version?
pub use proptest::sugar::NamedArguments;
