// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Propfuzz with bad strategies.

use propfuzz::prelude::*;
use proptest::collection::vec;
use std::collections::HashSet;

/// Incorrect strategy format.
#[propfuzz]
fn wrong_format(#[propfuzz(strategy(foo))] _: u8) {}

/// Invalid expression (_ is not a valid Rust identifier).
#[propfuzz]
fn invalid_expr(#[propfuzz(strategy = "_")] _: u8) {}

/// Unknown function name.
#[propfuzz]
fn unknown_name(#[propfuzz(strategy = "xxxxxxx(any::<u64>(), 0..8)")] _: HashSet<u64>) {}

/// Unknown attribute.
#[propfuzz]
fn unknown_attr(#[foo] _: u8) {}

/// Wrong type of strategy.
#[propfuzz]
fn wrong_strategy_type(#[propfuzz(strategy = "vec(any::<u64>(), 0..8)")] _: HashSet<u64>) {}

fn main() {}
