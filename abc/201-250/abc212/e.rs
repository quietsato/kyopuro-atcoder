use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uv: [(Usize1, Usize1); m]
    }

    const MOD: i64 = 998244353;

    let mut broken = vec![vec![]; n];
    for (u, v) in uv {
        broken[u].push(v);
        broken[v].push(u);
    }
    for i in 0..n {
        broken[i].push(i);
    }

    let mut dp = vec![vec![0; n]; k + 1];
    dp[0][0] = 1;

    for day in 1..=k {
        let total: i64 = dp[day - 1].iter().fold(0, |s, v| (s + v) % MOD);
        for next in 0..n {
            dp[day][next] += total;
            for &b in &broken[next] {
                dp[day][next] -= dp[day - 1][b];
                dp[day][next] %= MOD;
            }
        }
    }

    println!("{}", ((dp[k][0] % MOD) + MOD) % MOD);
}
