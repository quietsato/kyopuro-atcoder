use std::{collections::BinaryHeap, fmt::Binary};

use proconio::{input, marker::Usize1};

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
}

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [i64; n],
        ab: [(Usize1, Usize1); n - 1],
        vk: [(Usize1, usize); q]
    }

    let mut g = Graph::new(n);
    for (a, b) in ab {
        g.add_edge(a, b);
        g.add_edge(b, a);
    }

    let mut subs = vec![BinaryHeap::new(); n];
    let mut visited = vec![false; n];

    update(0, &x, &g, &mut subs, &mut visited);

    for &(v, k) in &vk {
        let mut s = subs[v].clone();
        for _ in 0..k - 1 {
            s.pop();
        }
        println!("{}", s.peek().unwrap().0);
    }
}

fn update(
    i: usize,
    x: &[i64],
    g: &Graph,
    subs: &mut Vec<BinaryHeap<(i64, usize)>>,
    visited: &mut Vec<bool>,
) {
    visited[i] = true;
    subs[i].push((x[i], i));

    for &c in &g.adj[i] {
        if visited[c] {
            continue;
        }
        update(c, x, g, subs, visited);
        let mut subs_c = subs[c].clone();
        (0..20).for_each(|_| {
            if let Some(sub) = subs_c.pop() {
                subs[i].push(sub);
            }
        });
    }

    let mut s = BinaryHeap::new();
    (0..20).for_each(|_| {
        if let Some(item) = subs[i].pop() {
            s.push(item);
        }
    });
    subs[i] = s;
}
