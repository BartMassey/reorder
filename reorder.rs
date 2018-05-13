// Copyright © 2018 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! Iterator that produces references to the items of a
//! sequence in some specified order.

#[cfg(test)]
extern crate rand;

/// Iterator for returning references to elements of a sequence
/// in a specified order, without disturbing the sequence. See
/// the documentation at `Reorder::new()` for more details.
pub struct Reorder<'a, T: 'a> {
    slice: &'a [T],
    posn: &'a [usize],
    index: usize,
}

impl <'a, T> Reorder<'a, T> {

    /// Create a new reorder iterator instance. The reorder
    /// iterator will return references to the elements of
    /// the sequence `slice`, in the order specified by the
    /// sequence `posn` of `usize` indices into
    /// `slice`. This iterator will `panic` if an index is
    /// out-of-bounds; any number of indices is allowed;
    /// repeated and omitted indices are allowed.
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
    pub fn new<S, P>(slice: &'a S, posn: &'a P) -> Reorder<'a, T>
        where S: AsRef<[T]> + 'a, P: AsRef<[usize]> + 'a
    {
        let slice = slice.as_ref();
        let posn = posn.as_ref();
        Reorder {
            slice,
            posn,
            index: 0,
        }
    }
}

impl <'a, T> Iterator for Reorder<'a, T> {
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

/// Utility trait for being able to say
/// *collection*`.iter_reorder()`.
pub trait IterReorder<T> {
    /// Return references to the elements of a collection in
    /// the specified order, without disturbing the
    /// collection.
    fn iter_reorder<'a, P>(&'a self, posn: &'a P) -> Reorder<T>
        where P: AsRef<[usize]> + 'a;
}

impl <T, S> IterReorder<T> for S
    where S: AsRef<[T]>
{
    fn iter_reorder<'a, P>(&'a self, posn: &'a P) -> Reorder<T>
        where P: AsRef<[usize]> + 'a
    {
        Reorder::new(self, posn)
    }
}

// Run a basic shuffle iterator and check that
// postconditions are satisfied.
#[test]
fn shuffle_test() {
    use rand::Rng;
    let elems = ['a', 'b', 'c', 'd', 'e'];
    let mut posn: Vec<usize> = (0..elems.len()).collect();
    rand::thread_rng().shuffle(&mut posn);
    let shuffle = Reorder::new(&elems, &posn);
    for (index, val) in shuffle.enumerate() {
        assert_eq!(*val, elems[posn[index]]);
    }
}
