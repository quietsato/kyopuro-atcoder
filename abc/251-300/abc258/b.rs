use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n]
    }
    let a = a
        .iter()
        .map(|s| s.iter().map(|c| c.to_digit(10).unwrap() as u64).collect_vec())
        .collect_vec();

    let mut ans = vec![];

    for sx in 0..n {
        for sy in 0..n {
            for (dx, dy) in &[
                (1, 0),
                (0, 1),
                (1, 1),
                (-1, -1),
                (-1, 0),
                (0, -1),
                (1, -1),
                (-1, 1),
            ] {
                let mut x = sx as i64;
                let mut y = sy as i64;
                let mut tmp = a[sy][sx];
                for _ in 1..n {
                    x += dx;
                    y += dy;
                    x += n as i64;
                    y += n as i64;
                    x %= n as i64;
                    y %= n as i64;
                    tmp *= 10;
                    tmp += a[y as usize][x as usize];
                }
                ans.push(tmp);
            }
        }
    }

    println!("{}", ans.iter().max().unwrap());
}
