use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m]

    };

    let mut g = Graph::new(n);

    for &(u, v) in &uv {
        g.add_edge(u, v);
        g.add_edge(v, u);
    }

    let mut ans = 0;
    for a in 0..n {
        for b in a + 1..n {
            for c in b + 1..n {
                ans += if g.has_edge(a, b) && g.has_edge(b, c) && g.has_edge(c, a) {
                    1
                } else {
                    0
                };
            }
        }
    }

    println!("{}", ans);
}

pub struct Graph {
    pub adj: Vec<Vec<usize>>,
    pub indeg: Vec<usize>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Self {
            adj: vec![vec![]; n],
            indeg: vec![0; n],
        }
    }

    fn add_edge(&mut self, i: usize, j: usize) {
        self.adj[i].push(j);
        self.indeg[j] += 1;
    }

    fn has_edge(&self, from: usize, to: usize) -> bool {
        self.adj[from].iter().find(|&&v| v == to).is_some()
    }
}
