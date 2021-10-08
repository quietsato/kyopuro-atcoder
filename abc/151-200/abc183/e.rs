use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    let m: u64 = 10u64.pow(9) + 7;

    let mut dp = vec![vec![0; w]; h];
    let mut xs = vec![vec![0; w]; h];
    let mut ys = vec![vec![0; w]; h];
    let mut zs = vec![vec![0; w]; h];

    dp[0][0] = 1;
    for y in 0..h {
        for x in 0..w {
            if x == 0 && y == 0 {
                continue;
            }
            if s[y][x] == '#' {
                continue;
            }
            if x > 0 {
                xs[y][x] = (xs[y][x - 1] + dp[y][x - 1]) % m;
            }
            if y > 0 {
                ys[y][x] = (ys[y - 1][x] + dp[y - 1][x]) % m;
            }
            if x > 0 && y > 0 {
                zs[y][x] = (zs[y - 1][x - 1] + dp[y - 1][x - 1]) % m;
            }
            dp[y][x] = (xs[y][x] + ys[y][x] + zs[y][x]) % m;
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}
