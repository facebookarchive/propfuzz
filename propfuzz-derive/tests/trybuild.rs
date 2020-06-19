// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Trybuild tests for propfuzz-derive.

#[test]
fn trybuild_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/*.rs");
}
