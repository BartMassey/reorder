// Copyright © 2018 Bart Massey

//! Iterator that produces the items of a `Vec` in shuffled
//! order.

extern crate rand;
use rand::Rng;

/// Shuffle iterator state.
pub struct ShuffleIter<'a, T: 'a> {
    vec: &'a Vec<T>,
    posn: Vec<usize>,
    index: usize,
}

impl <'a, T> ShuffleIter<'a, T> {

    /// Create a new shuffle iterator instance.
    fn new(vec: &Vec<T>) -> ShuffleIter<T> {
        let mut posn: Vec<usize> = (0..vec.len()).collect();
        rand::thread_rng().shuffle(&mut posn);
        ShuffleIter {
            vec,
            posn,
            index: 0,
        }
    }
}

impl <'a, T> Iterator for ShuffleIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.posn.len() {
            return None;
        }
        let result = &self.vec[self.posn[self.index]];
        self.index += 1;
        Some(result)
    }
}

/// Utility trait for being able to say
/// *collection*`.iter_shuffle()`.
pub trait Shuffle<T> {
    /// Return references to the elements of a collection in
    /// shuffled order, without disturbing the collection.
    fn iter_shuffle(&self) -> ShuffleIter<T>;
}

impl <T> Shuffle<T> for Vec<T> {
    fn iter_shuffle(&self) -> ShuffleIter<T> {
        ShuffleIter::new(&self)
    }
}

// This test cheats by inspecting the iterator state,
// because otherwise testing becomes hard and expensive.
#[test]
fn basic_test() {
    let vec: Vec<usize> = (1..6).collect();
    let shuffle = ShuffleIter::new(&vec);
    assert_eq!(vec, (1..6).collect::<Vec<usize>>());
    let posn = shuffle.posn.clone();
    for (index, val) in shuffle.enumerate() {
        assert_eq!(*val, vec[posn[index]]);
    }
}
