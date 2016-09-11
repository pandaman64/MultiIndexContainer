// mod btree;
// use btree::Btree;

mod rbtree;
use rbtree::Rbtree;

extern crate rand;
use rand::Rng;
use rand::StdRng;
use rand::SeedableRng;

use std::iter::Iterator;
use std::str;

fn main() {
    let mut tree = Rbtree::<i32>::new();
    let seed: &[_] = &[1];
    let mut rng: StdRng = rand::SeedableRng::from_seed(seed);
    let mut range = (0..100).collect::<Vec<_>>();
    rng.shuffle(&mut range);
    for i in range.into_iter() {
        tree.insert(i);
    }
    for v in tree.iter() {
        println!("{}", v);
    }
}
