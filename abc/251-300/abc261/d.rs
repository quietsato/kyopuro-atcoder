macro_rules! chmin {
    ($target:expr, $($v: expr),+) => {{
        $target = $target$(.min($v))*;
    }};
}

macro_rules! chmax {
    ($target:expr, $($v: expr),+) => {{
        $target = $target$(.max($v))*;
    }};
}

use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize; n],
        cy: [(usize, usize); m],
    };

    let mut dp = vec![vec![0; n + 1]; n + 1];
    let mut bonus = vec![0; n + 1];
    for (c, y) in cy {
        bonus[c] = y;
    }

    for i in 1..=n {
        for j in 1..=i {
            chmax!(dp[i][j], dp[i - 1][j - 1] + x[i - 1] + bonus[j]); // i 番目に表
            chmax!(dp[i][0], dp[i - 1][j - 1]); // i番目に裏
        }
    }

    println!("{}", dp.last().unwrap().iter().max().unwrap());
}
