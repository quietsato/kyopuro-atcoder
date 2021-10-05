use itertools::Itertools;
use proconio::input;

const INF: u64 = 100_000_000_000;

fn main() {
    input! {
        n: usize,
        xyz: [(i64, i64, i64); n]
    }

    let bit_max = 1 << n;

    let cost = xyz
        .iter()
        .map(|&(a, b, c)| {
            xyz.iter()
                .map(|&(x, y, z)| ((x - a).abs() + (y - b).abs() + (z - c).max(0)) as u64)
                .collect_vec()
        })
        .collect_vec();

    let mut dp = vec![vec![INF; n]; bit_max];
    dp[0][0] = 0;

    for bit in 0..(1 << n) {
        for current in 0..n {
            for next in 0..n {
                // Already visited
                if bit & (1 << next) > 0 {
                    continue;
                }
                dp[bit | (1 << next)][next] =
                    (dp[bit | (1 << next)][next]).min(dp[bit][current] + cost[current][next]);
            }
        }
    }

    // for row in &dp {
    //     println!("{:?}", row);
    // }

    println!("{}", dp[(1 << n) - 1][0]);
}
