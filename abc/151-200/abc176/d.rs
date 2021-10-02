use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize,
        w: usize,
        (ch, cw): (Usize1, Usize1),
        (dh, dw): (Usize1, Usize1),
        s: [Chars; h]
    }

    let inf = h * w + 1;

    let mut dp = vec![vec![inf; w]; h];
    dp[ch][cw] = 0;

    let mut nexts = VecDeque::new();
    nexts.push_front((cw, ch));

    while let Some((x, y)) = nexts.pop_front() {
        // walk
        for ny in (y as i64 - 1).max(0)..=(y as i64 + 1).min(h as i64 - 1) {
            for nx in (x as i64 - 1).max(0)..=(x as i64 + 1).min(w as i64 - 1) {
                let (nx, ny) = (nx as usize, ny as usize);
                if !(nx == x || ny == y) {
                    continue;
                }
                if s[ny][nx] == '.' && dp[ny][nx] > dp[y][x] {
                    dp[ny][nx] = dp[y][x];
                    nexts.push_front((nx, ny));
                }
            }
        }

        // warp
        for ny in (y as i64 - 2).max(0)..=(y as i64 + 2).min(h as i64 - 1) {
            for nx in (x as i64 - 2).max(0)..=(x as i64 + 2).min(w as i64 - 1) {
                let (nx, ny) = (nx as usize, ny as usize);
                if s[ny][nx] == '.' && dp[ny][nx] > dp[y][x] + 1 {
                    dp[ny][nx] = dp[y][x] + 1;
                    nexts.push_back((nx, ny));
                }
            }
        }
    }

    println!(
        "{}",
        if dp[dh][dw] == inf {
            "-1".to_string()
        } else {
            dp[dh][dw].to_string()
        }
    );
}
