# Mutable Binary Heap where the keys are indices

this is modified version of [mut-binary-heap](https://github.com/Wasabi375/mut-binary-heap) crate. The main difference is that the keys are indices which improve performance

----

# mut-binary-heap

[![Crates.io](https://img.shields.io/crates/v/mut-binary-heap.svg)](https://crates.io/crates/mut-binary-heap)
[![Documentation](https://docs.rs/mut-binary-heap/badge.svg)](https://docs.rs/mut-binary-heap/)
[![Codecov](https://codecov.io/github/Wasabi375/mut-binary-heap/coverage.svg?branch=master)](https://codecov.io/gh/Wasabi375/mut-binary-heap)
[![Dependency status](https://deps.rs/repo/github/Wasabi375/mut-binary-heap/status.svg)](https://deps.rs/repo/github/Wasabi375/mut-binary-heap)

This create provides a binary heap implementation that stores key-value pairs.
The main advantage of that is that unlike with an implementation like
[`std::collections::BinaryHeap`] checking if any given key exist is `O(1)` instead of `O(n)`.
Same for getting the value for a given key. This allows for cheap modification of
values within the binary heap. Updating a value is `O(log n)` iff you have direct access to the value.
For a binary heap that does not store key-value pairs update operations would be `O(n)` because
they first have to find the value to update. The disadvantage is the additional storage space
required to store a hash map that provides indices into the heap for each key.


## MSRV (Minimum Supported Rust Version)

The minimum supported Rust version is 1.56.0.

# Changes

This crate is based on [binary-heap-plus](https://github.com/sekineh/binary-heap-plus-rs) 
which is itself a based on the standard library's implementation of
[`BinaryHeap`](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html).

Version 0.1.0 provides a thorough refactor to allow storage of key-value pairs as well as 
retrieval and modification of values.

For more see: 
[CHANGELOG.md](https://github.com/Wasabi375/mut-binary-heap/blob/master/CHANGELOG.md).


# License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
