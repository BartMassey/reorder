# Reorder — Rust iterator over reordered references
Copyright (c) 2018 Bart Massey

This Rust library crate implements an iterator that returns
references to the elements of a slice in some specified
order, without disturbing the slice. See the crate
documentation for details.

This iterator was primarily written as a classroom demo of
iterator implementation. As such, style and substantative
comments, issues and PRs are very welcome.

## To Do

Provide a version of the reorder iterator that takes the
positions from an iterator. Haven't provided yet, because
it's hard to see a use case of this functionality, but it
seems like it might be useful.

## License

This program is licensed under the "MIT License".  Please
see the file LICENSE in the source distribution of this
software for license terms.
