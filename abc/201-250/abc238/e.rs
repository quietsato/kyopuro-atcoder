use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        lr: [(Usize1, Usize1); q]
    }
    let mut g = Graph::new(n + 1);
    for (l, r) in lr {
        g.add_edge(l, r + 1);
        g.add_edge(r + 1, l);
    }
    let mut visited = vec![false; n + 1];
    let mut nexts = VecDeque::new();
    nexts.push_back(0);
    while let Some(current) = nexts.pop_front() {
        if visited[current] {
            continue;
        }
        visited[current] = true;
        for &next in g.nexts(current) {
            if !visited[next] {
                nexts.push_back(next);
            }
        }
    }
    println!("{}", if visited[n] { "Yes" } else { "No" });
}

// -------------------------------------------------------------------------------
// Graph
pub struct Graph {
    pub adj: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Self {
            adj: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, i: usize, j: usize) {
        self.adj[i].push(j)
    }

    fn nexts(&self, i: usize) -> std::slice::Iter<'_, usize> {
        self.adj[i].iter()
    }
}
