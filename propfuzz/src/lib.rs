// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Rust toolkit to combine property-based testing with fuzzing.
//!
//! For more, see the [`README`](https://github.com/facebookincubator/propfuzz/blob/main/README.md)
//! at the root of the `propfuzz` repository.

pub mod prelude;
pub mod runtime;
pub mod traits;

// Re-export the propfuzz macro -- this is expected to be the primary interface.
#[cfg(feature = "macro")]
pub use propfuzz_macro::propfuzz;

pub use proptest;
