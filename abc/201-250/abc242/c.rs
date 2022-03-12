use proconio::input;

fn main() {
    input! {
        n: usize
    }

    const MOD: u64 = 998244353;

    let mut dp = vec![vec![0; 9]; n];
    for i in 0..9 {
        dp[0][i] = 1;
    }

    for i in 1..n {
        dp[i][0] = (dp[i - 1][0] + dp[i - 1][1]) % MOD;
        dp[i][8] = (dp[i - 1][7] + dp[i - 1][8]) % MOD;
        for j in 1..8 {
            dp[i][j] = (dp[i - 1][j - 1] + dp[i - 1][j] + dp[i - 1][j + 1]) % MOD;
        }
    }

    println!("{}", dp[n - 1].iter().sum::<u64>() % MOD);
}

