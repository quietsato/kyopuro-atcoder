use proconio::input;

fn main() {
    const MOD: i64 = 998244353;

    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let mut dp = vec![vec![0i64; m + 2]; n];
    // Init
    dp[0][1] = 1;

    for i in 0..n - 1 {
        // Restore
        for j in 1..=m {
            dp[i][j] += dp[i][j - 1];
            dp[i][j] %= MOD;
        }

        // Next
        for j in 1..=m {
            let l = if k == 0 {
                j
            } else if j > k - 1 {
                j - (k - 1)
            } else {
                0
            };
            let r = if k == 0 {
                j
            } else if j + k <= m {
                j + k
            } else {
                m + 1
            };
            dp[i + 1][0] += dp[i][j];
            dp[i + 1][l] -= dp[i][j];
            dp[i + 1][r] += dp[i][j];
        }
    }

    // Restore
    for j in 1..=m {
       dp[n - 1][j] += dp[n - 1][j - 1];
       dp[n - 1][j] %= MOD;
    }

    // println!("{:?}", &dp);

    let mut ans = 0i64;
    for j in 1..=m {
        ans += dp[n - 1][j];
        ans %= MOD;
    }
    ans %= MOD;
    if ans < 0 {
        ans += MOD;
    }
    println!("{}", ans);
}
