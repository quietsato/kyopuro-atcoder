use proconio::{input, marker::Usize1};

macro_rules! chmin {
    ($target:expr, $($v: expr),+) => {{
        $target = $target$(.min($v))*;
    }};
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, u64); m]
    }

    const INF: u64 = 1 << 60;

    let mut dp = vec![vec![INF; n]; n];

    for &(a, b, c) in &abc {
        dp[a][b] = c;
        dp[b][a] = c;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                chmin!(dp[i][j], dp[i][k] + dp[k][j]);
            }
        }
    }

    let mut ans = 0;
    'outer: for &(a, b, c) in &abc {
        for i in 0..n {
            if dp[a][i] + dp[i][b] <= c {
                ans += 1;
                continue 'outer;
            }
        }
    }
    println!("{}", ans);
}
