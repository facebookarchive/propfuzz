// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::ValueGenerator;
use std::fmt;

/// Represents a generic fuzz target for unstructured data.
pub trait FuzzTarget: Send + Sync + fmt::Debug {
    /// Returns the name of the fuzz target.
    fn name(&self) -> &'static str;

    /// Returns a description for this fuzz target.
    fn description(&self) -> &'static str;

    /// Generates a new example for this target to store in the corpus. `idx` is the current index
    /// of the item being generated, starting from 0.
    ///
    /// Returns `Some(bytes)` if a value is generated, or `None` if no value can be generated.
    fn generate(&self, ctx: &mut GenerateContext) -> Option<Vec<u8>>;

    /// Fuzz the target using the given random data. The fuzzer tests for panics or OOMs with this
    /// method.
    fn fuzz(&self, data: &[u8]);
}

/// Context for generating values in `FuzzTarget` instances.
#[derive(Debug)]
pub struct GenerateContext {
    index: usize,
    value_generator: ValueGenerator,
}

impl GenerateContext {
    /// Returns the current index of the item being generated, starting from 0.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns a `ValueGenerator`, which can be used with a proptest `Strategy` to generate random
    /// values.
    ///
    /// This is best suited for *unstructured* fuzzing, where the fuzzer is expected to identify
    /// patterns by using a corpus.
    pub fn value_generator(&mut self) -> &mut ValueGenerator {
        &mut self.value_generator
    }

    // ---
    // Helper methods
    // ---

    /// Creates a new instance.
    pub(crate) fn new() -> Self {
        Self {
            index: 0,
            value_generator: ValueGenerator::new(),
        }
    }

    /// Increments the index.
    pub(crate) fn increment(&mut self) {
        self.index += 1;
    }
}
