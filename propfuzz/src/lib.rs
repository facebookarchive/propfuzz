// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Rust toolkit to combine property-based testing with fuzzing.
//!
//! For more, see the [`README`](https://github.com/facebookincubator/propfuzz/blob/trunk/README.md)
//! at the root of the `propfuzz` repository.

mod propfuzz;
pub mod runtime;

pub use crate::propfuzz::*;

// Re-export the propfuzz macro -- this is expected to be the primary interface.
#[cfg(feature = "macro")]
pub use propfuzz_macro::propfuzz;

// Re-export proptest to give access to its data structures.
#[doc(no_inline)]
pub use proptest as test;
