use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    const INF: u64 = 1 << 30;

    input! {
        n: usize,
        ar: Usize1, // row
        ac: Usize1, // column
        br: Usize1, // row
        bc: Usize1, // column
        s: [Chars; n],
    }

    let dc = [-1, -1, 1, 1];
    let dr = [-1, 1, -1, 1];

    let mut dist = vec![vec![vec![INF; 5]; n]; n];
    let mut nexts = VecDeque::new();
    dist[ar][ac][4] = 0;
    nexts.push_back(((ar, ac), 4));

    while let Some(((cr, cc), prev_move)) = nexts.pop_front() {
        if (cr, cc) == (br, bc) {
            println!("{}", dist[cr][cc][prev_move]);
            return;
        }

        for i in 0..4 {
            let (nr, nc) = (cr as i64 + dr[i], cc as i64 + dc[i]);
            if !(0..n as i64).contains(&nc) || !(0..n as i64).contains(&nr) {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if s[nr][nc] == '#' {
                continue;
            }

            let nd = dist[cr][cc][prev_move] + if i == prev_move { 0 } else { 1 };
            if dist[nr][nc][i] > nd {
                dist[nr][nc][i] = nd;
                if i == prev_move {
                    nexts.push_front(((nr, nc), i));
                } else {
                    nexts.push_back(((nr, nc), i));
                }
            }
        }
    }

    println!("-1");
}
