use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        g: [Chars; h],
    };
    let mut visited = BTreeSet::new();
    let (mut i, mut j) = (0, 0);
    loop {
        match g[i][j] {
            'U' => {
                if i == 0 {
                    break;
                } else {
                    i -= 1;
                }
            }
            'D' => {
                if i == h - 1 {
                    break;
                } else {
                    i += 1;
                }
            }
            'L' => {
                if j == 0 {
                    break;
                } else {
                    j -= 1;
                }
            }
            'R' => {
                if j == w - 1 {
                    break;
                } else {
                    j += 1;
                }
            }
            _ => unreachable!(),
        }
        if !visited.insert((i, j)) {
            println!("-1");
            return;
        }
    }
    println!("{} {}", i + 1, j + 1);
}
