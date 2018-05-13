# Shuffle — Rust iterator over permuted references
Copyright (c) 2018 Bart Massey

This Rust library crate implements an iterator that returns
references to the elements of a slice in some shuffled
order. The iterator uses an internal slice of `usize` to maintain
the permutation order, so it is *O(n)* space on a slice of
*n* items, but *O(1)* time.

## License

This program is licensed under the "MIT License".  Please
see the file LICENSE in the source distribution of this
software for license terms.
