use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        s: Usize1,
        t: Usize1,
        x: Usize1,
        uv: [(Usize1, Usize1); m]
    }

    const MOD: u64 = 998244353;

    let mut g = Graph::new(n);
    for &(u, v) in &uv {
        g.add_edge(u, v);
        g.add_edge(v, u);
    }

    let mut dp = vec![vec![vec![0; 2]; n]; k + 1];
    dp[0][s][0] = 1;

    for i in 0..k {
        for j in 0..n {
            for b in 0..=1 {
                for &next in &g.adj[j] {
                    let next_b = if next == x { b + 1 } else { b } % 2;
                    dp[i + 1][next][next_b] += dp[i][j][b];
                    dp[i + 1][next][next_b] %= MOD;
                }
            }
        }
    }

    println!("{}", dp[k][t][0]);
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
}
