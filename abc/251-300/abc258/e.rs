use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        x: u64,
        w: [u64; n],
        k: [Usize1; q]
    }
    let s = w.iter().sum::<u64>();
    let w = w.repeat(2);
    let offset = (x / s) * n as u64;

    let mut c = vec![0; n];
    let mut g = Graph::new(n);

    {
        let x = x % s;
        let mut r = 0;
        let mut s = 0;
        for l in 0..n {
            if r < l {
                r = l;
                s = 0;
            }
            while s < x {
                s += w[r];
                r += 1;
            }
            c[l] = (r - l) as u64 + offset;
            g.add_edge(l, r % n);
            s -= w[l];
        }
    }

    let mut m = 0;
    let mut p = vec![];
    let mut visited = vec![n; n];
    {
        let mut current = 0;
        for i in 0..=n {
            p.push(c[current]);
            visited[current] = i;
            let next = g.next(current);
            if visited[next] < n {
                m = visited[current];
                break;
            }
            current = next;
        }
    }

    let cycle_len = p.len() - m;
    for k in k {
        if k < m {
            println!("{}", p[k]);
        } else {
            let cycle_pos = k - m;
            println!("{}", p[m + cycle_pos % cycle_len]);
        }
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

    fn next(&self, i: usize) -> usize {
        self.adj[i][0]
    }
}
