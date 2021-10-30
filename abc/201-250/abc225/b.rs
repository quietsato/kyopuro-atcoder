use std::vec;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1]
    }

    let mut deg = vec![0; n];

    for &(a, b) in &ab {
        deg[a] += 1;
        deg[b] += 1;
    }

    if deg.iter().filter(|d| **d > 1).count() == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
