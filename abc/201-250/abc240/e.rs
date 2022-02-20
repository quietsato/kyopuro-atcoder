use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1]
    }
    let mut g = Graph::new(n);
    for (u, v) in uv {
        g.add_edge(u, v);
        g.add_edge(v, u);
    }
    let mut visited = vec![false; n];
    let mut lr: Vec<(i64, i64)> = vec![(-1, -1); n];
    lr[0].1 = solve(&mut lr, &mut visited, &g, 0, 1);

    for (l, r) in lr {
        println!("{} {}", l, r);
    }
}

fn solve(lr: &mut Vec<(i64, i64)>, visited: &mut Vec<bool>, g: &Graph, i: usize, l: i64) -> i64 {
    visited[i] = true;
    lr[i].0 = l;
    let mut r = l - 1;
    for &c in g.adj[i].iter() {
        if visited[c] {
            continue;
        }
        r += 1;
        r = solve(lr, visited, g, c, r);
    }
    let r = l.max(r);
    lr[i].1 = r;
    r
}

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
