use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [u64; n]
    }

    let mut bigger = BinaryHeap::new();
    let mut smaller = BinaryHeap::new();

    for i in 0..k {
        bigger.push(Reverse(p[i]));
    }
    let mut ans = bigger.pop().unwrap().0;

    for i in k..n {
        println!("{}", ans);

        if p[i] > ans {
            bigger.push(Reverse(p[i]));
            smaller.push(ans);
            ans = bigger.pop().unwrap().0;
        } else {
            smaller.push(p[i]);
        }
    }

    println!("{}", ans);
}
