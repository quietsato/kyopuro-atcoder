use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i64,
        r: i64,
        a: [i64; n]
    };

    let mut dp = vec![vec![0; 3]; n + 1];

    for i in 1..=n {
        dp[i][0] = l + dp[i - 1][0]; // L
        dp[i][1] = a[i - 1] + (dp[i - 1][0]).min(dp[i - 1][1]); // A_i
        dp[i][2] = r + (dp[i - 1][0]).min(dp[i - 1][1]).min(dp[i - 1][2]) // R
    }

    println!("{}", dp[n].iter().min().unwrap());
}
