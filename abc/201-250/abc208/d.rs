use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, u64); m]
    }

    let con = {
        let mut con = vec![vec![None; n]; n];
        for &(a, b, c) in &abc {
            let (a, b) = (a - 1, b - 1);
            con[a][b] = Some(c);
        }
        con
    };

    let mut dp = vec![vec![vec![None; n]; n]; n + 1];
    dp[0] = con;

    for k in 0..n {
        for s in 0..n {
            for t in 0..n {
                if s == t {
                    continue;
                }

                dp[k + 1][s][t] = dp[k][s][t];
                if let (Some(c1), Some(c2)) = (dp[k][s][k], dp[k][k][t]) {
                    dp[k + 1][s][t] = Some(dp[k + 1][s][t].map_or(c1 + c2, |v| v.min(c1 + c2)));
                }
            }
        }
    }

    println!(
        "{}",
        dp.iter()
            .skip(1)
            .flatten()
            .flatten()
            .map(|v| { v.unwrap_or(0) })
            .sum::<u64>()
    );
}
