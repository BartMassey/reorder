extern crate rand;
use rand::Rng;

/// Iterator that produces the items of a `Vec` in shuffled
/// order.
struct Shuffle<'a, T: 'a> {
    vec: &'a Vec<T>,
    posn: Vec<usize>,
    index: usize,
}

trait Shuffled<T> {
    fn iter_shuffle(&self) -> Shuffle<T>;
}

impl <T> Shuffled<T> for Vec<T> {
    fn iter_shuffle(&self) -> Shuffle<T> {
        Shuffle::new(&self)
    }
}

impl <'a, T> Shuffle<'a, T> {
    fn new(vec: &Vec<T>) -> Shuffle<T> {
        let mut posn: Vec<usize> = (0..vec.len()).collect();
        rand::thread_rng().shuffle(&mut posn);
        Shuffle {
            vec,
            posn,
            index: 0,
        }
    }
}

impl <'a, T> Iterator for Shuffle<'a, T> {
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

fn main() {
    let vec: Vec<usize> = (1..6).collect();
    for val in vec.iter_shuffle() {
        println!("{}", val);
    }
    println!("{:?}", vec);
}
