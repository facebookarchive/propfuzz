// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Propfuzz with bad arguments.

use propfuzz::propfuzz;

/// Arguments of the wrong type (note multiple errors).
#[propfuzz(cases = true, fork = 1024)]
fn wrong_type1(_: u8) {}

/// Basic test for repeated arguments.
#[propfuzz(cases = 1024, cases = 2048)]
fn repeated1(_: Vec<u8>) {}

// Test lack of description for repeated2 as well.

#[propfuzz(cases = 1024)]
#[propfuzz(cases = 2048)]
fn repeated2(_: Vec<u8>) {}

/// Unsupported attribute kinds.
#[propfuzz]
#[propfuzz(wat)]
#[propfuzz = "wat"]
fn unsupported_kinds(_: u8) {}

/// Repeated arguments and strategies.
#[propfuzz(cases = 256)]
#[propfuzz(cases = 512)]
fn repeated_strategy(
    #[propfuzz(strategy = "any::<u8>()")]
    #[propfuzz(strategy = "any::<u8>()")]
    _: u8,
) {
}

fn main() {}
