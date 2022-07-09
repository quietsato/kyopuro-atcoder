use std::collections::{BTreeSet, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        sx: i64,
        sy: i64,
        tx: i64,
        ty: i64,
        xyr: [(i64, i64, i64); n],
    }
    let mut g = Graph::new(n);

    for (i, (x1, y1, r1)) in xyr.iter().enumerate() {
        for (j, (x2, y2, r2)) in xyr.iter().enumerate() {
            if i == j {
                continue;
            }
            let d2 = (x2 - x1).pow(2) + (y2 - y1).pow(2);
            if (r2 - r1).pow(2) <= d2 && d2 <= (r1 + r2).pow(2) {
                g.add_edge(i, j);
            }
        }
    }

    let mut visited = vec![false; n];
    let mut nexts = VecDeque::new();
    let mut goals = BTreeSet::new();

    for (i, (x1, y1, r1)) in xyr.iter().enumerate() {
        if (sx - x1).pow(2) + (sy - y1).pow(2) == r1.pow(2) {
            nexts.push_back(i);
        }
        if (tx - x1).pow(2) + (ty - y1).pow(2) == r1.pow(2) {
            goals.insert(i);
        }
    }

    assert!(!nexts.is_empty());
    assert!(!goals.is_empty());

    // dbg!(&nexts);
    // dbg!(&goals);
    // dbg!(&g.adj);

    while let Some(current) = nexts.pop_front() {
        if visited[current] {
            continue;
        }
        visited[current] = true;
        if goals.contains(&current) {
            println!("Yes");
            return;
        }

        for &next in g.nexts(current) {
            if visited[next] {
                continue;
            }
            nexts.push_back(next);
        }
    }

    println!("No");
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
