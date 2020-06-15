// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A propfuzz with no arguments.

use propfuzz_derive::propfuzz;

#[propfuzz]
fn no_args() {}

fn main() {}
