# propfuzz: Rust tools to combine coverage-guided fuzzing with property-based testing

This repository contains the source code for:

* [`propfuzz`](propfuzz): a Rust library for adapting the [`proptest`](https://github.com/AltSysrq/proptest/) framework
  with coverage-guided fuzzers. 
* [`propfuzz-macro`](propfuzz-macro): procedural macros to make writing `propfuzz` tests easy 

The code in this repository is in a **very early** state and is under active development.

## Vision

The overarching goal of `propfuzz` is to make it easy to write new fuzz targets, whether *unstructured* or *structured*.

**`propfuzz` does not reinvent the wheel.** It adapts the existing property-based test framework `proptest` to also work
for fuzzing. If your project has an existing investment in `proptest`, all of it should be reusable for coverage-guided
fuzzing with minimal work.

(Read about the [differences between `proptest` and
`quickcheck`](https://altsysrq.github.io/proptest-book/proptest/vs-quickcheck.html)).

Here's what we expect `proptest` to look like when it's ready:

### Developer interface

**As a developer, you should generally not have to think in terms of adding fuzz targets or using particular fuzz
engines.** You should just think about what properties you want your system to satisfy, and simply annotate them with
`#[propfuzz]`.

For fuzzing methods that take unstructured inputs, you should be able to use `proptest` to generate a high-quality
corpus.

```rust
fn generate(generator: &mut ValueGenerator) -> Vec<u8> {
    // Generate a new value using a proptest strategy.
    let new_value = generator.new_value(any::<MyType>());
    // Serialize this value into a byte array.
    new_value.serialize()
}

/// Test that deserializing a value doesn't cause crashes or OOMs.
#[propfuzz(corpus_generator = "generate")]
fn fuzz_deserialize(data: &[u8]) {
    MyType::deserialize(data)
}
```

For fuzzing methods that take structured inputs, you should be able to use `proptest` to write property-based tests,
then have them automatically converted to fuzz targets. For example:

```rust
/// Test that reversing a list twice produces the same result.
#[propfuzz(cases = 1024)]
fn reverse(#[propfuzz(strategy = "vec(any::<u32>(), 0..64)")] mut list: Vec<u32>) {
    let list2 = list.clone();
    list.reverse();
    list.reverse();
    prop_assert_eq!(list, list2);
}
```

### Running fuzzers

Every test with `#[propfuzz]` annotated to it should also become a fuzz target. It should be easy to run `#[propfuzz]`
targets in one of two modes:
* as a standard property-based test, for local test runs and immediate CI feedback
* using a coverage-guided fuzzing engine such as [libFuzzer](https://llvm.org/docs/LibFuzzer.html),
[AFL](https://github.com/google/AFL), or [honggfuzz](https://github.com/google/honggfuzz).

Furthermore, it should be easy to integrate into fuzzing services like
[Google's oss-fuzz](https://github.com/google/oss-fuzz).

## Contributing

See the [CONTRIBUTING](CONTRIBUTING.md) file for how to help out.

## License

This project is available under the terms of either the [Apache 2.0 license](LICENSE-APACHE) or the
[MIT license](LICENSE-MIT).
