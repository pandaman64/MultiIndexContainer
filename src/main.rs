mod btree;
use btree::Btree;

extern crate rand;
use rand::Rng;
use rand::StdRng;
use rand::SeedableRng;

use std::iter::Iterator;
use std::str;

fn main() {
    let mut tree = Btree::<i32>::new();
    let seed: &[_] = &[1];
    let mut rng: StdRng = rand::SeedableRng::from_seed(seed);
    let mut range = (0..std::env::args().nth(1).unwrap().parse::<i32>().unwrap()).collect::<Vec<_>>();
    rng.shuffle(&mut range);
    for i in range.into_iter() {
        tree.insert(i);
    }
    for v in tree.iter() {
        println!("{}", v);
    }
}
