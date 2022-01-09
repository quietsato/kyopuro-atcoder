use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h]
    }

    let mut dp = vec![vec![0; w + 1]; h + 1];
    dp[1][1] = 1;

    for y in 0..h {
        for x in 0..w {
            if x == 0 && y == 0 {
                continue;
            }
            if c[y][x] == '.' {
                dp[y + 1][x + 1] += (dp[y][x + 1]).max(dp[y + 1][x]);
                if dp[y + 1][x + 1] > 0 {
                    dp[y + 1][x + 1] += 1;
                }
            }
        }
    }

    println!(
        "{}",
        dp.iter()
            .map(|row| { row.iter().max().unwrap() })
            .max()
            .unwrap()
    );
}
