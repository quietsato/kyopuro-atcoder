use std::collections::BTreeSet;
use std::ops::BitXor;

use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

struct Graph {
    edges: Vec<Vec<(usize, usize)>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        return Self {
            edges: vec![vec![]; n],
        };
    }

    fn add_edge(&mut self, i: usize, j: usize, w: usize) {
        self.edges[i].push((j, w));
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abw: [(Usize1, Usize1, usize); m]
    };

    let mut dp = vec![vec![false; 1024]; n];
    let mut g = Graph::new(n);

    for (a,b,w) in abw {
        g.add_edge(a, b, w);
    }

    let mut nexts = BTreeSet::<(usize, usize)>::new();
    nexts.insert((0, 0));

    while let Some((cur_node, cur_xor)) = nexts.pop_first() {
        dp[cur_node][cur_xor] = true;

        for (next_node, edge_weight) in &g.edges[cur_node] {
            let next_node = *next_node;
            let next_xor = cur_xor ^ edge_weight;
            if !dp[next_node][next_xor] {
                nexts.insert((next_node, next_xor));
            }
        }
    }

    let ans = dp[n-1].iter().find_position(|&&b| b).map(|(p, _)| p as i64).unwrap_or(-1);
    println!("{}", ans);
}

