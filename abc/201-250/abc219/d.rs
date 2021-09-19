use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        ab: [(usize, usize); n]
    }

    const INF: u64 = 301;

    let mut dp = vec![vec![vec![INF; y + 2]; x + 2]; n + 1];
    dp[0][0][0] = 0;
    for i in 0..n {
        let (a, b) = ab[i];
        for j in (0..=x).rev() {
            for k in (0..=y).rev() {
                dp[i + 1][j][k] = dp[i + 1][j][k].min(dp[i][j][k]);
                dp[i + 1][(j + a).min(x)][(k + b).min(y)] =
                    dp[i + 1][(j + a).min(x)][(k + b).min(y)].min(dp[i][j][k] + 1);
            }
        }
    }

    println!(
        "{}",
        if dp[n][x][y] >= INF {
            "-1".to_string()
        } else {
            dp[n][x][y].to_string()
        }
    );
}
