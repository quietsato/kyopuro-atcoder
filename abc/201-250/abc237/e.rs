use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        h: [i64; n],
        uv: [(Usize1, Usize1); m]
    }

    let mut g = WeightedGraph::new(n);
    for (u, v) in uv {
        g.add_edge(u, v, if h[u] >= h[v] { 0 } else { h[v] - h[u] });
        g.add_edge(v, u, if h[v] >= h[u] { 0 } else { h[u] - h[v] });
    }

    const INF: i64 = std::i64::MAX;

    let mut dist = vec![INF; n];
    let mut visited = vec![false; n];
    dist[0] = 0;

    // Minimum priority queue
    let mut nexts = BinaryHeap::new(); // (dist, next)
    nexts.push((Reverse(0), 0));

    while let Some((Reverse(d), current)) = nexts.pop() {
        if visited[current] {
            continue;
        }
        visited[current] = true;
        dist[current] = d;

        for &(next, weight) in &g.adj[current] {
            if visited[next] {
                continue;
            }
            nexts.push((Reverse(d + weight), next));
        }
    }

    let ans = dist
        .iter()
        .zip(h.iter())
        .map(|(d, hi)| h[0] - hi - d)
        .max()
        .unwrap();
    println!("{}", ans);
}

// -------------------------------------------------------------------------------
// WeightedGraph
pub struct WeightedGraph {
    pub adj: Vec<Vec<(usize, i64)>>,
}

impl WeightedGraph {
    fn new(n: usize) -> Self {
        Self {
            adj: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, i: usize, j: usize, w: i64) {
        self.adj[i].push((j, w))
    }
}
