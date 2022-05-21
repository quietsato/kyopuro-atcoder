use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    const INF: u64 = 1 << 60;

    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, u64); m]
    }

    let mut con = vec![vec![]; n];
    for (i, &(a, b, c)) in abc.iter().enumerate() {
        con[a].push((b, c, i + 1));
        con[b].push((a, c, i + 1));
    }

    let mut visited = vec![false; n];
    let mut dist = vec![INF; n];
    let mut edge_from = vec![0; n];

    let mut nexts = BinaryHeap::new();
    nexts.push((Reverse(0u64), 0usize, 0usize)); // distance, vertex, edge_from
    while let Some((Reverse(c_dist), current, e_from)) = nexts.pop() {
        if visited[current] {
            continue;
        } else {
            visited[current] = true;
        }
        dist[current] = c_dist;
        edge_from[current] = e_from;
        for &(next, edge_dist, edge_index) in &con[current] {
            let n_dist = dist[current] + edge_dist;
            if dist[next] > n_dist {
                nexts.push((Reverse(n_dist), next, edge_index));
            }
        }
    }
    println!(
        "{}",
        edge_from
            .iter()
            .skip(1)
            .map(ToString::to_string)
            .collect_vec()
            .join(" ")
    );
}
