use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    const MOD: u64 = 998244353;
    const MASK_MAX: usize = 1 << 10;

    let s = s.iter().map(|&c| c as usize - 'A' as usize).collect_vec();

    let mut dp = vec![vec![vec![0u64; 10]; MASK_MAX]; n + 1];

    for i in 1..=n {
        let next = s[i - 1];

        for mask in 0..MASK_MAX {
            for last in 0..10 {
                dp[i][mask][last] = dp[i - 1][mask][last];

                // chained
                if last == next {
                    dp[i][mask][last] += dp[i - 1][mask][last];
                    dp[i][mask][last] %= MOD;
                }
            }
        }

        for mask in 0..MASK_MAX {
            // not chained
            if (mask & (1 << next)) > 0 {
                continue;
            }
            for last in 0..10 {
                dp[i][mask | (1 << next)][next] += dp[i - 1][mask][last];
                dp[i][mask | (1 << next)][next] %= MOD;
            }
        }

        // in cases where first joined contest is the next contest
        dp[i][(1 << next)][next] += 1;
        dp[i][(1 << next)][next] %= MOD;
    }

    let ans = {
        let mut ans = 0;
        for mask in 0..MASK_MAX {
            for last in 0..10 {
                ans += dp[n][mask][last];
                ans %= MOD;
            }
        }
        ans
    };

    println!("{}", ans);
}
