use std::cmp::Ordering::{Equal, Greater, Less};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        h: [u64; n],
        ab: [(usize, usize); m]
    }

    let mut good = vec![true; n];
    for (a, b) in &ab {
        match h[a - 1].cmp(&h[b - 1]) {
            Less => good[a - 1] = false,
            Equal => {
                good[a - 1] = false;
                good[b - 1] = false
            }
            Greater => good[b - 1] = false,
        }
    }

    println!("{}", good.iter().filter(|&&g| g).count())
}
