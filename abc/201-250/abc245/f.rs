use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m]
    }

    let mut g = Graph::new(n);
    for (u, v) in uv {
        g.add_edge(u, v);
    }

    let scc = g.scc();
    let mut dp = vec![false; scc.len()];

    // どの頂点がどの強連結成分に属しているのか
    let idx = {
        let mut idx = vec![0; n];
        for (i, scc) in scc.iter().enumerate() {
            for &v in scc {
                idx[v] = i;
            }
        }
        idx
    };

    let mut ans = 0;
    for (i, scc) in scc.iter().enumerate().rev() {
        if scc.len() >= 2 {
            dp[i] = true;
        } else {
            for &next in &g.adj[scc[0]] {
                dp[i] |= dp[idx[next]];
            } 
        }
        if dp[i] {
            ans += scc.len();
        }
    }
    println!("{}", ans);
}

// -------------------------------------------------------------------------------
// Graph
pub struct Graph {
    pub n: usize,
    pub adj: Vec<Vec<usize>>,
    pub edges: Vec<(usize, usize)>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![vec![]; n],
            edges: vec![],
        }
    }

    fn add_edge(&mut self, i: usize, j: usize) {
        self.adj[i].push(j);
        self.edges.push((i, j));
    }
}

// -------------------------------------------------------------------------------
// 強連結成分分解
impl Graph {
    fn scc(&self) -> Vec<Vec<usize>> {
        // これ以上進むことができなくなった順に頂点を格納
        let order = {
            let mut visited = vec![false; self.n];
            let mut order = vec![];
            for i in 0..self.n {
                if visited[i] {
                    continue;
                }
                _scc_dfs1(self, i, &mut visited, &mut order);
            }
            order
        };

        // 逆グラフを作成
        let gt = {
            let mut gt = Graph::new(self.n);
            for &(i, j) in &self.edges {
                gt.add_edge(j, i);
            }
            gt
        };

        // 強連結成分分解
        let sccs = {
            let mut visited = vec![false; self.n];
            let mut sccs = vec![];
            for &o in order.iter().rev() {
                let mut scc = vec![];
                if visited[o] {
                    continue;
                }
                _scc_dfs2(&gt, o, &mut visited, &mut scc);
                sccs.push(scc);
            }
            sccs
        };
        sccs
    }
}

fn _scc_dfs1(g: &Graph, v: usize, visited: &mut [bool], order: &mut Vec<usize>) {
    if visited[v] {
        return;
    }
    visited[v] = true;
    for &next in &g.adj[v] {
        if visited[next] {
            continue;
        }
        _scc_dfs1(g, next, visited, order);
    }
    order.push(v);
}

fn _scc_dfs2(g: &Graph, v: usize, visited: &mut [bool], scc: &mut Vec<usize>) {
    if visited[v] {
        return;
    }
    visited[v] = true;
    scc.push(v);
    for &next in &g.adj[v] {
        if visited[next] {
            continue;
        }
        _scc_dfs2(g, next, visited, scc);
    }
}
