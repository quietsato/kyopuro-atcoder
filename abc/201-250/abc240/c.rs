use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n]
    }
    let mut dp = vec![vec![false; 10001]; n + 1];

    dp[0][0] = true;
    for (i, &(a, b)) in ab.iter().enumerate() {
        for j in 0..10001 {
            if dp[i][j] {
                if j + a < 10001 {
                    dp[i + 1][j + a] = true;
                }
                if j + b < 10001 {
                    dp[i + 1][j + b] = true;
                }
            }
        }
    }

    if dp[n][x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
