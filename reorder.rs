// Copyright © 2018 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! Iterator that produces references to the items of a
//! slice (or thing convertible from one) in some specified order.
//!
//! # Examples
//!
//! Produce the elements of a slice in shuffled order.
//!
//! ```
//! # extern crate reorder;
//! # use reorder::*;
//! extern crate rand;
//!
//! # fn main() {
//! use rand::Rng;
//! let elems = ['a', 'b', 'c', 'd', 'e'];
//! let mut posn: Vec<usize> = (0..elems.len()).collect();
//! rand::thread_rng().shuffle(&mut posn);
//! let shuffle = elems.iter_reorder(&posn);
//! for (index, val) in shuffle.enumerate() {
//!     assert_eq!(*val, elems[posn[index]]);
//! }
//! # }
//! ```

/// Iterator for returning references to elements of a sequence
/// in a specified order, without disturbing the sequence.
#[derive(Clone, Debug)]
pub struct Reorder<'a, T: 'a> {
    slice: &'a [T],
    posn: &'a [usize],
    index: usize,
}

impl<'a, T> Reorder<'a, T> {
    /// Create a new reorder iterator instance. The reorder
    /// iterator will return references to the elements of
    /// the sequence `slice`, in the order specified by the
    /// sequence `posn` of `usize` indices into `slice`. Any
    /// number of indices is allowed; repeated and omitted
    /// indices are allowed.
    ///
    /// # Panics
    ///
    /// This iterator panics with an index error if any
    /// index is larger than the slice allows.
    /// 
    /// # Examples
    ///
    /// ```
    /// let check = |p: &[usize], x: &[&char]| {
    ///     let chars = ['a', 'b', 'c'];
    ///     let r:Vec<&char> = reorder::Reorder::new(&chars, p).collect();
    ///     assert_eq!(r, x);
    /// };
    /// check(&[1, 0, 2], &[&'b', &'a', &'c']);
    /// check(&[0, 2], &[&'a', &'c']);
    /// check(&[1, 0, 1, 1], &[&'b', &'a', &'b', &'b']);
    /// ```
    pub fn new(slice: &'a [T], posn: &'a [usize]) -> Reorder<'a, T> {
        Reorder {
            slice,
            posn,
            index: 0,
        }
    }
}

impl<'a, T> Iterator for Reorder<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.posn.len() {
            return None;
        }
        let result = &self.slice[self.posn[self.index]];
        self.index += 1;
        Some(result)
    }
}

#[test]
#[should_panic]
fn index_error() {
    let _ = Reorder::new(&['a'], &[1]).next();
}

/// Utility trait for being able to say
/// *collection*`.iter_reorder()`.
pub trait IterReorder<T> {
    /// Return references to the elements of a collection in
    /// the specified order, without disturbing the
    /// collection.
    fn iter_reorder<'a>(&'a self, posn: &'a [usize]) -> Reorder<T>;
}

impl<T, S: AsRef<[T]>> IterReorder<T> for S {
    fn iter_reorder<'a>(&'a self, posn: &'a [usize]) -> Reorder<T> {
        Reorder::new(self.as_ref(), posn)
    }
}
