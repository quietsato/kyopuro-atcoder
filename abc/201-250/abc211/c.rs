use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    const MOD: u64 = 1_000_000_007;

    let chokudai = "chokudai".chars().collect_vec();

    let mut dp = vec![vec![0u64; s.len() + 1]; 8];

    for i in 0..s.len() {
        dp[0][i + 1] = dp[0][i];
        if s[i] == chokudai[0] {
            dp[0][i + 1] += 1;
        }
    }

    for j in 0..7 {
        for i in j..s.len() {
            dp[j + 1][i + 1] = dp[j + 1][i];
            if s[i] == chokudai[j + 1] {
                dp[j + 1][i + 1] = dp[j + 1][i] + dp[j][i];
            }
            dp[j + 1][i + 1] %= MOD;
        }
    }

    println!("{}", dp[7][s.len()]);
}
