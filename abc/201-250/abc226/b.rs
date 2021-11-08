use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut arr = BTreeSet::new();

    for _ in 0..n {
        input! {
            li: usize,
            ai: [i64; li]
        }

        arr.insert(ai);
    }

    println!("{}", arr.len());
}
