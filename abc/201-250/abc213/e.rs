use std::{collections::VecDeque, vec};

use proconio::{input, marker::Chars};

const DIFF: &[(i32, i32)] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];
const BREAK_X: &[(i32, i32)] = &[(1, 2), (-1, 1), (-2, -1), (-1, 1)];
const BREAK_Y: &[(i32, i32)] = &[(-1, 1), (1, 2), (-1, 1), (-2, -1)];

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    let mut con = vec![vec![]; h * w];
    for y in 0..h as i32 {
        for x in 0..w as i32 {
            for (i, &(dx, dy)) in DIFF.iter().enumerate() {
                if !(0..w as i32).contains(&(x + dx)) || !(0..h as i32).contains(&(y + dy)) {
                    continue;
                }

                let (nx, ny) = ((x + dx) as usize, (y + dy) as usize);
                let (x, y) = (x as usize, y as usize);

                let cost = if s[ny][nx] == '#' { 1 } else { 0 };
                con[x + y * w].push((cost, (nx, ny)));

                if cost == 1 {
                    let (bx, by) = (BREAK_X[i], BREAK_Y[i]);
                    for dx in bx.0..=bx.1 {
                        for dy in by.0..=by.1 {
                            if !(0..w as i32).contains(&(x as i32 + dx))
                                || !(0..h as i32).contains(&(y as i32 + dy))
                            {
                                continue;
                            }
                            let (mx, my) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                            con[x + y * w].push((1, (mx, my)));
                        }
                    }
                }
            }
        }
    }

    const INF: i32 = 500 * 500;

    let mut min_cost = vec![vec![INF; w]; h];
    min_cost[0][0] = 0;

    let mut nexts = VecDeque::new();
    nexts.push_back((0usize, 0usize));
    while let Some((x, y)) = nexts.pop_front() {
        for &(nc, (nx, ny)) in &con[x + y * w] {
            if min_cost[ny][nx] > min_cost[y][x] + nc {
                min_cost[ny][nx] = min_cost[y][x] + nc;
                match nc {
                    0 => nexts.push_front((nx, ny)),
                    1 => nexts.push_back((nx, ny)),
                    _ => unreachable!(),
                }
            }
        }
    }

    // for c in &min_cost {
    //     println!("{:?}", c);
    // }

    println!("{}", min_cost[h - 1][w - 1]);
}
