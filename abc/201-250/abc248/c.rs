use proconio::input;

fn main() {
    const MOD: u64 = 998244353;

    input! {
        n: usize,
        m: usize,
        k: usize
    }

    let mut dp = vec![vec![0; k as usize + 1]; n];
    for j in 1..=m.min(k) {
        dp[0][j] = 1;
    }
    for i in 1..n {
        for j in i..=k {
            for t in 1..=m {
                if j + t > k {
                    continue;
                }
                dp[i][j + t] += dp[i - 1][j];
                dp[i][j + t] %= MOD;
            }
        }
    }

    let mut ans = 0;
    for &val in &dp[n - 1] {
        ans += val;
        ans %= MOD;
    }
    println!("{}", ans);
}
