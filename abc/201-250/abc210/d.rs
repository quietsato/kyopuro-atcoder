use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: u64,
        a: [[u64; w];h]
    }

    let mut b = a.clone();
    b.iter_mut().for_each(|v| v.reverse());

    let inf = 10u64.pow(10);

    println!("{}", solve(h, w, c, inf, &a).min(solve(h, w, c, inf, &b)));
}

fn solve(h: usize, w: usize, c: u64, inf: u64, a: &Vec<Vec<u64>>) -> u64 {
    let mut dp = vec![vec![inf; w + 1]; h + 1];
    for i in 1..=h {
        for j in 1..=w {
            dp[i][j] = a[i - 1][j - 1].min(dp[i - 1][j] + c).min(dp[i][j - 1] + c);
        }
    }

    let mut x = vec![vec![inf; w + 1]; h + 1];
    for i in 1..=h {
        for j in 1..=w {
            x[i][j] = dp[i - 1][j].min(dp[i][j - 1]) + c + a[i - 1][j - 1];
        }
    }

    *x.iter().flatten().min().unwrap()
}
