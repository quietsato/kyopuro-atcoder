use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        abx: [(i64, i64, Usize1); n - 1]
    };

    let mut g = WeightedGraph::new(n);
    let mut visited = vec![None; n];
    for (i, &(a, b, x)) in abx.iter().enumerate() {
        g.add_edge(i, i + 1, a);
        g.add_edge(i, x, b);
    }

    let mut nexts = std::collections::BTreeSet::<(i64, usize)>::new();
    nexts.insert((0, 0));

    while let Some((dist, current)) = nexts.pop_first() {
        if visited[current].is_none() {
            visited[current] = Some(dist);
            for &(next, w) in &g.adj[current] {
                if visited[next].is_some() {
                    continue;
                }
                nexts.insert((dist + w, next));
            }
        }
    }

    println!("{}", visited[n - 1].unwrap());
}

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
        self.adj[i].push((j, w));
    }
}
