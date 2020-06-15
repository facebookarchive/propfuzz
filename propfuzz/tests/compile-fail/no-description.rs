// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A propfuzz with no description.

use propfuzz::propfuzz;

#[propfuzz]
fn no_description(_: usize) {}

fn main() {}
