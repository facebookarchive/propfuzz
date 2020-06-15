// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Propfuzz is not supported for methods, only for top-level functions.

use propfuzz::propfuzz;

struct Foo;

impl Foo {
    /// Method.
    #[propfuzz]
    fn fuzz(&self, _: Vec<u8>) {}

    /// Method 2.
    #[propfuzz]
    fn fuzz_mut(&mut self, _: Vec<u8>) {}

    /// Method 3.
    #[propfuzz]
    fn fuzz_consume(self, _: Vec<u8>) {}

    /// Don't know how to detect this one, but it fails.
    #[propfuzz]
    fn fuzz_static(_: Vec<u8>) {}
}

fn main() {}
