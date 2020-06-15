// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Combine property-based testing with fuzzing.

mod propfuzz;
pub mod runtime;

pub use crate::propfuzz::*;

// Re-export proptest to give access to its data structures.
#[doc(no_inline)]
pub use proptest as test;
