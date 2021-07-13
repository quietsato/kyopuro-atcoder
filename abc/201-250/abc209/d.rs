use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize, usize); n-1],
        cd: [(usize, usize); q]
    }

    let mut d = vec![n; n];
    let mut edges = vec![vec![]; n];
    for (a, b) in ab {
        edges[a - 1].push(b - 1);
        edges[b - 1].push(a - 1);
    }
    dfs(&mut d, 0, n, 0, &edges);

    for (tc, td) in cd {
        if (d[tc - 1] & 1) == (d[td - 1] & 1) {
            println!("Town");
        } else {
            println!("Road");
        }
    }
}

fn dfs(dists: &mut Vec<usize>, d: usize, n: usize, v: usize, edges: &Vec<Vec<usize>>) {
    dists[v] = d;
    for e in &edges[v] {
        if dists[*e] == n {
            dfs(dists, d + 1, n, *e, edges);
        }
    }
}
