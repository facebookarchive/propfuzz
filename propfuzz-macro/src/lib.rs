// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Procedural macros for propfuzz tests.
//!
//! This crate is an implementation detail of `propfuzz` and is **not** meant to be used directly.
//! Use it through [`propfuzz`](https://crates.io/crates/propfuzz) instead.

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

mod config;
mod errors;
mod propfuzz_impl;

/// The core macro, used to annotate test methods.
///
/// Annotate a function with this in order to have it run as a property-based test using the
/// [`proptest`](https://docs.rs/proptest) framework. In the future, such tests will also be
/// available as fuzz targets.
///
/// # Examples
///
/// ```
/// // The prelude imports the `propfuzz` macro.
///
/// use propfuzz::prelude::*;
/// use proptest::collection::vec;
///
/// /// Reversing a list twice produces the same result.
/// #[propfuzz]
/// fn reverse(
///     #[propfuzz(strategy = "vec(any::<u32>(), 0..64)")]
///     mut list: Vec<u32>,
/// ) {
///     let list2 = list.clone();
///     list.reverse();
///     list.reverse();
///     prop_assert_eq!(list, list2);
/// }
/// ```
///
/// # Arguments
///
/// `propfuzz` supports a number of arguments which can be used to customize test behavior.
///
/// Attributes can be broken up with commas and split up across multiple lines like so:
///
/// ```
/// use propfuzz::prelude::*;
/// use proptest::collection::vec;
///
/// /// Options broken up across multiple lines.
/// #[propfuzz(cases = 1024, max_local_rejects = 10000)]
/// #[propfuzz(fork = true)]
/// fn reverse(
///     #[propfuzz(strategy = "vec(any::<u32>(), 0..64)")]
///     mut list: Vec<u32>,
/// ) {
///     let list2 = list.clone();
///     list.reverse();
///     list.reverse();
///     prop_assert_eq!(list, list2);
/// }
/// ```
///
/// ## Proptest configuration
///
/// The following `proptest`
/// [configuration options](https://docs.rs/proptest/0.10/proptest/test_runner/struct.Config.html)
/// are supported:
///
/// * `cases`
/// * `max_local_rejects`
/// * `max_global_rejects`
/// * `max_flat_map_regens`
/// * `fork`
/// * `timeout`
/// * `max_shrink_time`
/// * `max_shrink_iters`
/// * `verbose`
///
/// ## Argument configuration
///
/// The following configuration options are supported on individual arguments:
///
/// * `strategy`: A strategy to generate and shrink values of the given type. The value must be a
///   string that parses as a Rust expression which evaluates to an implementation of
///   [`Strategy`](https://docs.rs/proptest/0.10/proptest/strategy/trait.Strategy.html)
///   for the given type. Defaults to [the
///   canonical strategy](https://docs.rs/proptest/0.10/proptest/arbitrary/trait.Arbitrary.html)
///   for the type.
#[proc_macro_attribute]
pub fn propfuzz(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as AttributeArgs);
    let item = parse_macro_input!(item as ItemFn);

    propfuzz_impl::propfuzz_impl(attr, item)
        .unwrap_or_else(|err| err)
        .into()
}
