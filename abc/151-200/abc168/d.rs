use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m]
    }

    let inf = n;
    let mut from = vec![inf; n];
    let mut con = vec![vec![]; n];

    for &(a, b) in &ab {
        con[a].push(b);
        con[b].push(a);
    }

    let mut nexts: VecDeque<_> = VecDeque::new();
    nexts.push_back(0usize);

    while let Some(current) = nexts.pop_front() {
        for &next in &con[current] {
            if from[next] == inf {
                from[next] = current;
                nexts.push_back(next);
            }
        }
    }

    if from[1..].iter().any(|&from| from == inf) {
        println!("No");
    } else {
        println!("Yes");
        for from in &from[1..] {
            println!("{}", from + 1);
        }
    }
}
