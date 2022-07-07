use std::collections::{VecDeque, BTreeSet};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
        q: usize,
        xk: [(Usize1, usize); q]
    }

    let g = {
        let mut g = Graph::new(n);
        for (a, b) in ab {
            g.add_edge(a, b);
            g.add_edge(b, a);
        }
        g
    };

    for (x, k) in xk {
        let mut nexts = VecDeque::new();
        nexts.push_back((x, 0));
        let mut visited = BTreeSet::new();

        while let Some((current, dist)) = nexts.pop_front() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);
            if dist == k {
                continue;
            }
            for next in g.nexts(current) {
                if visited.contains(next) {
                    continue;
                }
                nexts.push_back((*next, dist + 1));
            }
        }

        let ans: u64 = visited
            .iter()
            .map(|&i| i as u64 + 1)
            .sum();
        println!("{}", ans);
    }
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
