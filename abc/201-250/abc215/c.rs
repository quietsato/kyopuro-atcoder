use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        k: usize
    }

    let mut set = BTreeSet::new();

    s.iter()
        .permutations(s.len())
        .map(|s| s.into_iter().collect::<String>())
        .for_each(|perm| {
            set.insert(perm);
        });

    println!("{}", set.iter().skip(k - 1).next().unwrap())
}
