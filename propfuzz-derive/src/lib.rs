// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Procedural macros for propfuzz tests.

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{ItemFn, parse_macro_input};

mod propfuzz_impl;

/// The main propfuzz macro.
#[proc_macro_attribute]
pub fn propfuzz(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);

    propfuzz_impl::propfuzz_impl(item).unwrap_or_else(|err| err).into()
}
