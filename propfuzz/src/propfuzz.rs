// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use proptest::prelude::*;
use proptest::test_runner::TestRunner;
use std::fmt;

/// Represents a structured fuzz target.
///
/// A trait that implements `Propfuzz` can be used both as a standard property-based test, and
/// as a target for structured, mutation-based fuzzing.
///
/// Structured, mutation-based fuzzers use random byte sequences as a pass-through RNG. In other
/// words, the random byte sequences act as something like a DNA for random values.
pub trait Propfuzz: Send + Sync + fmt::Debug {
    /// Returns the name of this structured fuzz target.
    fn name(&self) -> &'static str;

    /// Returns a description for this structured fuzz target.
    fn description(&self) -> &'static str;

    /// Returns the proptest config for this fuzz target.
    ///
    /// Note that the config is modified for the
    fn proptest_config(&self) -> ProptestConfig {
        ProptestConfig::default()
    }

    /// Executes this test using the given test runner.
    ///
    /// This is where the main body of the test goes.
    fn execute(&self, test_runner: &mut TestRunner);
}
