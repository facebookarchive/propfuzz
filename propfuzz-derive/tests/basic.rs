// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Basic tests for propfuzz-derive.
use pretty_assertions::assert_eq;
use propfuzz::Propfuzz;
use propfuzz_derive::propfuzz;

/// Basic test for foo.
///
/// This is a simple test which ensures that adding two numbers returns the expected result.
#[propfuzz]
fn add_two(a: u64, b: u64) {
    let sum = a.checked_add(b);
    assert_eq!(sum, a.checked_add(b));
}

#[test]
fn propfuzz_add_two() {
    assert_eq!(__PROPFUZZ__add_two.name(), "basic::add_two");
    assert_eq!(
        __PROPFUZZ__add_two.description(),
        "Basic test for foo.\n\
        \n\
        This is a simple test which ensures that adding two numbers returns the expected result."
    );
}
