// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Basic tests for propfuzz-macro.
use pretty_assertions::assert_eq;
use propfuzz::{prelude::*, traits::StructuredTarget};
use proptest::{
    collection::vec,
    test_runner::{FileFailurePersistence, TestError, TestRunner},
};

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

    let config = __PROPFUZZ__add_two.proptest_config();
    assert_eq!(config.cases, 256, "correct case count");
    assert_eq!(config.fork, false, "correct fork setting");
}

/// Pairs should parse just like individual values.
#[propfuzz(fork = true)]
fn add_pair((a, b): (u64, u64)) {
    let sum = a.checked_add(b);
    assert_eq!(sum, a.checked_add(b));
}

#[test]
fn propfuzz_add_pair() {
    assert_eq!(__PROPFUZZ__add_pair.name(), "basic::add_pair");
    assert_eq!(
        __PROPFUZZ__add_pair.description(),
        "Pairs should parse just like individual values."
    );

    let config = __PROPFUZZ__add_pair.proptest_config();
    assert_eq!(config.cases, 256, "correct case count");
    assert_eq!(config.fork, true, "correct fork setting");
}

/// Test that reversing a list twice produces the same results.
#[propfuzz(cases = 1024)]
fn reverse(#[strategy(vec(any::<u32>(), 0..64))] mut list: Vec<u32>) {
    let list2 = list.clone();
    list.reverse();
    list.reverse();
    prop_assert_eq!(list, list2);
}

#[test]
fn propfuzz_reverse() {
    assert_eq!(__PROPFUZZ__reverse.name(), "basic::reverse");
    assert_eq!(
        __PROPFUZZ__reverse.description(),
        "Test that reversing a list twice produces the same results."
    );

    let config = __PROPFUZZ__reverse.proptest_config();
    assert_eq!(config.cases, 1024, "correct case count");
    assert_eq!(config.fork, false, "correct fork setting");
}

/// This test fails. It is ignored by default and can be run with `cargo test -- --ignored`.
#[propfuzz]
#[ignore]
fn failing(#[strategy(vec(any::<u32>(), 0..64))] mut list: Vec<u32>) {
    let list2 = list.clone();
    // The list is only reversed once.
    list.reverse();
    prop_assert_eq!(list, list2);
}

#[test]
fn propfuzz_failing() {
    assert_eq!(__PROPFUZZ__failing.name(), "basic::failing");
    assert_eq!(
        __PROPFUZZ__failing.description(),
        "This test fails. It is ignored by default and can be run with `cargo test -- --ignored`."
    );

    let mut config = __PROPFUZZ__failing.proptest_config();
    assert_eq!(config.cases, 256, "correct case count");
    assert_eq!(config.fork, false, "correct fork setting");

    // Try running the test and ensure it fails with the correct value. (Determinism is ensured
    // through checking in basic-failing-seed.)
    config.failure_persistence = Some(Box::new(FileFailurePersistence::Direct(
        "tests/basic-failing-seed",
    )));
    let mut test_runner = TestRunner::new(config);
    let err = __PROPFUZZ__failing
        .execute(&mut test_runner)
        .expect_err("test should fail");
    assert!(
        matches!(err, TestError::Fail(_, value) if &value.0 == &[0, 1]),
        "minimal test case"
    );
}

/// Test multiple #[propfuzz] annotations.
#[propfuzz(cases = 1024)]
#[propfuzz(fork = true)]
fn multi(_: Vec<u8>) {}

#[test]
fn propfuzz_multi() {
    assert_eq!(__PROPFUZZ__multi.name(), "basic::multi");
    assert_eq!(
        __PROPFUZZ__multi.description(),
        "Test multiple #[propfuzz] annotations."
    );

    let config = __PROPFUZZ__multi.proptest_config();
    assert_eq!(config.cases, 1024, "correct case count");
    assert_eq!(config.fork, true, "correct fork setting");
}

/// Test all proptest options.
#[propfuzz(cases = 1, max_local_rejects = 2, max_global_rejects = 3)]
#[propfuzz(max_flat_map_regens = 4, fork = true, timeout = 5, max_shrink_time = 6)]
#[propfuzz(max_shrink_iters = 7, verbose = 8)]
#[ignore]
fn all_proptest_options(_: u8) {}

#[test]
fn propfuzz_all_proptest_options() {
    let config = __PROPFUZZ__all_proptest_options.proptest_config();
    assert_eq!(config.cases, 1);
    assert_eq!(config.max_local_rejects, 2);
    assert_eq!(config.max_global_rejects, 3);
    assert_eq!(config.max_flat_map_regens, 4);
    assert_eq!(config.fork, true);
    assert_eq!(config.timeout, 5);
    assert_eq!(config.max_shrink_time, 6);
    assert_eq!(config.max_shrink_iters, 7);
    assert_eq!(config.verbose, 8);
}
