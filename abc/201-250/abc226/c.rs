use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize
    }

    let mut g = vec![vec![]; n];
    let mut t = vec![0; n];

    for i in 0..n {
        input! {
            ti: u64,
            ki: usize,
            ai: [Usize1; ki]
        }

        t[i] = ti;
        for ai in ai {
            g[i].push(ai);
        }
    }

    let mut dep = BTreeSet::new();
    let mut learned = vec![false; n];

    let mut nexts = vec![n - 1];
    while !nexts.is_empty() {
        let current = nexts.pop().unwrap();
        learned[current] = true;
        dep.insert(current);

        for &next in &g[current] {
            if learned[next] {
                continue;
            }
            nexts.push(next);
        }
    }

    println!("{}", dep.iter().map(|&d| t[d]).sum::<u64>());
}
