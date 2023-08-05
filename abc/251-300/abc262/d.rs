use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    const MOD: usize = 998244353;

    let mut dp = vec![vec![vec![0; n + 1]; n + 1]; n + 1];

    let mut ans = 0;
    for m in 1..=n {
        for i in 1..=n {
            dp[i - 1][0][0] = 1;
            for j in 1..=m {
                for k in 0..m {
                    dp[i][j][k] =
                        dp[i - 1][j][k] + dp[i - 1][j - 1][((k + m) - (a[i - 1] % m)) % m];
                    dp[i][j][k] %= MOD;
                }
            }
        }
        ans += dp[n][m][0];
        ans %= MOD;
    }

    println!("{}", ans);
}
