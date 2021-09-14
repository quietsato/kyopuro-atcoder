use std::{collections::BTreeSet, iter::FromIterator};

use proconio::input;

fn main() {
    input! {
        s: [String; 3]
    }

    let mut mid = BTreeSet::from_iter("BRGH".chars().into_iter());

    for s in s {
        mid.remove(&s.chars().nth(1).unwrap());
    }

    println!("A{}C", mid.iter().next().unwrap());
}
