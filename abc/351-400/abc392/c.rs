use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        q: [u64; n],
    };

    // i がかかれたゼッケンをつけている人
    let mut v = vec![0; n];
    for (i, &q) in q.iter().enumerate() {
        v[(q - 1) as usize] = i;
    }

    let ans = v.iter().map(|&v| format!("{}", q[p[v]])).join(" ");
    println!("{}", ans);
}
