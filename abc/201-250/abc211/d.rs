use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    const MOD: u64 = 1_000_000_007;

    let mut con = vec![vec![]; n];
    ab.iter().for_each(|(a, b)| {
        con[a - 1].push(b - 1);
        con[b - 1].push(a - 1)
    });

    let mut dist = vec![200_000; n];
    dist[0] = 0;
    let mut paths = vec![0u64; n];
    paths[0] = 1;
    let mut nexts = VecDeque::new();
    nexts.push_back(0usize);

    // BFS: O(N + M)
    while let Some(v) = nexts.pop_front() {
        for e in &con[v] {
            if dist[*e] > dist[v] + 1 {
                dist[*e] = dist[v] + 1;
                paths[*e] = paths[v];
                nexts.push_back(*e);
            } else if dist[*e] == dist[v] + 1 {
                paths[*e] += paths[v];
                paths[*e] %= MOD;
            }
        }
    }

    println!("{}", paths.last().unwrap());
}
