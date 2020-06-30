// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Procedural macros for propfuzz tests.
//!
//! This library is **not** meant to be used directly. Use it through the `propfuzz` library.

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

mod config;
mod errors;
mod propfuzz_impl;

/// The main propfuzz macro.
#[proc_macro_attribute]
pub fn propfuzz(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as AttributeArgs);
    let item = parse_macro_input!(item as ItemFn);

    propfuzz_impl::propfuzz_impl(attr, item)
        .unwrap_or_else(|err| err)
        .into()
}
