// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Re-exports for the most commonly used APIs of `propfuzz`.
//!
//! This includes:
//! * the `propfuzz` macro from this crate
//! * the entire prelude of `proptest`, so existing tests can be migrated with minimal hassle.
//!
//! ## Examples
//!
//! There's no need to specify `proptest` as a separate dependency, since you can write:
//!
//! ```
//! use propfuzz::prelude::*;
//! use proptest::collection::vec;
//!
//! /// Example test.
//! #[propfuzz]
//! fn test(#[propfuzz(strategy = "vec(any::<u8>(), 0..64)")] v: Vec<u8>) {
//!     // ...
//! }
//! ```

#[doc(no_inline)]
pub use crate::propfuzz;

#[doc(no_inline)]
pub use proptest;
#[doc(no_inline)]
pub use proptest::prelude::*;
