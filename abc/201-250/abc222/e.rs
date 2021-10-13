use proconio::{input, marker::Usize1};

const MOD: i64 = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: i64,
        a: [Usize1; m],
        uv: [(Usize1, Usize1); n-1]
    }
    let con = {
        let mut con = vec![vec![]; n];
        for (i, &(u, v)) in uv.iter().enumerate() {
            con[u].push((v, i));
            con[v].push((u, i));
        }
        con
    };

    let mut edge_count = vec![0; n - 1];
    for (&start, &goal) in a.iter().zip(a.iter().skip(1)) {
        let mut visited = vec![false; n];
        visited[start] = true;
        dfs(&con, &mut visited, &mut edge_count, start, goal);
    }

    let mut dp = vec![vec![0; 200_001]; 2];
    const BASE: i64 = 100_000;
    dp[0][BASE as usize] = 1;
    for i in 0..(n - 1) {
        let current = i % 2;
        let next = (i + 1) % 2;
        dp[next].iter_mut().for_each(|v| {
            *v = 0;
        });
        for j in 0..=200_000 {
            if dp[current][j as usize] > 0 {
                dp[next][(j - edge_count[i]) as usize] += dp[current][j as usize];
                dp[next][(j - edge_count[i]) as usize] %= MOD;
                dp[next][(j + edge_count[i]) as usize] += dp[current][j as usize];
                dp[next][(j + edge_count[i]) as usize] %= MOD;
            }
        }
    }

    println!("{}", dp[(n - 1) % 2][(k + BASE) as usize]);
}

fn dfs(
    con: &Vec<Vec<(usize, usize)>>,
    visited: &mut Vec<bool>,
    edge_count: &mut Vec<u64>,
    current: usize,
    goal: usize,
) -> bool {
    if current == goal {
        return true;
    }
    for &(next, i) in &con[current] {
        if visited[next] {
            continue;
        }
        visited[next] = true;
        if dfs(con, visited, edge_count, next, goal) {
            edge_count[i] += 1;
            return true;
        }
    }

    return false;
}
