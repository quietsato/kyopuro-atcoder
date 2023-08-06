use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n - 1]
    };
    let mut graph = Graph::new(n);
    for (i, &p) in p.iter().enumerate() {
        graph.add_edge(p, i + 1);
    }

    let mut nexts = vec![(0usize, 1usize)];
    let mut gen = vec![n + 1; n];

    while let Some((current, g)) = nexts.pop() {
        for &next in &graph.adj[current] {
            if gen[next] < g {
                continue;
            }
            gen[next] = g;
            nexts.push((next, g + 1));
        }
    }

    println!("{}", gen[n - 1]);
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
}
