use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n]
    }

    let mut dp = vec![vec![false; 2]; n];
    dp[0][0] = true;
    dp[0][1] = true;

    for i in 0..n - 1 {
        dp[i + 1][0] = (dp[i][0] && (a[i + 1] - a[i]).abs() <= k) || (dp[i][1] && (a[i + 1] - b[i]).abs() <= k);
        dp[i + 1][1] = (dp[i][0] && (b[i + 1] - a[i]).abs() <= k) || (dp[i][1] && (b[i + 1] - b[i]).abs() <= k);
    }

    if dp[n - 1][0] || dp[n - 1][1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
