use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    };

    const INF: u64 = u64::MAX >> 1;

    // initial pos
    let [x1, y1, x2, y2] = {
        let mut i = 0;
        let mut pos = [0; 4];
        for (y, s) in s.iter().enumerate() {
            for (x, &c) in s.iter().enumerate() {
                if c == 'P' {
                    pos[i] = x;
                    pos[i + 1] = y;
                    i += 2;
                }
            }
        }
        pos
    };

    let mut dp = vec![vec![vec![vec![INF; n]; n]; n]; n];

    let mut nexts = VecDeque::new();
    nexts.push_back((x1, y1, x2, y2));
    dp[x1][y1][x2][y2] = 0;

    while let Some((x1, y1, x2, y2)) = nexts.pop_front() {
        let current_step = dp[x1][y1][x2][y2];

        let next_pos = [
            next_pos(x1, y1, x2, y2, n, &s, -1, 0),
            next_pos(x1, y1, x2, y2, n, &s, 1, 0),
            next_pos(x1, y1, x2, y2, n, &s, 0, 1),
            next_pos(x1, y1, x2, y2, n, &s, 0, -1),
        ];
        for (x1, y1, x2, y2) in next_pos {
            if dp[x1][y1][x2][y2] < INF {
                continue;
            } else {
                dp[x1][y1][x2][y2] = current_step + 1;
                nexts.push_back((x1, y1, x2, y2));
            }
        }
    }

    // find ans
    let ans = (0..n)
        .flat_map(|y| (0..n).map(|x| dp[x][y][x][y]).collect_vec())
        .min()
        .unwrap();
    println!(
        "{}",
        (ans == INF)
            .then(|| "-1".to_string())
            .unwrap_or_else(|| ans.to_string())
    );
}

fn next_pos(
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    n: usize,
    s: &Vec<Vec<char>>,
    dx: i32,
    dy: i32,
) -> (usize, usize, usize, usize) {
    let x1_ = x1 as i32 + dx;
    let y1_ = y1 as i32 + dy;
    let x2_ = x2 as i32 + dx;
    let y2_ = y2 as i32 + dy;

    // Player 1
    let x1_ = if (0..n as i32).contains(&x1_) {
        x1_ as usize
    } else {
        x1
    };
    let y1_ = if (0..n as i32).contains(&y1_) {
        y1_ as usize
    } else {
        y1
    };
    let (x1_, y1_) = if s[y1_][x1_] == '#' {
        (x1, y1)
    } else {
        (x1_, y1_)
    };

    // Player 2
    let x2_ = if (0..n as i32).contains(&x2_) {
        x2_ as usize
    } else {
        x2
    };
    let y2_ = if (0..n as i32).contains(&y2_) {
        y2_ as usize
    } else {
        y2
    };
    let (x2_, y2_) = if s[y2_][x2_] == '#' {
        (x2, y2)
    } else {
        (x2_, y2_)
    };

    (x1_, y1_, x2_, y2_)
}
