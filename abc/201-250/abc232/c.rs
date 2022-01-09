use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
        cd: [(Usize1, Usize1); m],
    }

    let mut x = Graph::new(n);
    let mut y = Graph::new(n);

    for (a, b) in ab {
        x.add_edge(a, b);
        x.add_edge(b, a);
    }
    for (c, d) in cd {
        y.add_edge(c, d);
        y.add_edge(d, c);
    }

    'outer: for perm in (0..n).into_iter().permutations(n) {
        for (xi, &yi) in perm.iter().enumerate() {
            for j in 0..n {
                if x.has_edge(xi, j) != y.has_edge(yi, perm[j]) {
                    continue 'outer;
                }
            }
        }
        println!("Yes");
        return;
    }

    println!("No");
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

    fn edges_from(&self, i: usize) -> &[usize] {
        &self.adj[i]
    }

    fn add_edge(&mut self, i: usize, j: usize) {
        self.adj[i].push(j)
    }

    fn has_edge(&self, from: usize, to: usize) -> bool {
        self.adj[from].iter().find(|&&v| v == to).is_some()
    }
}
