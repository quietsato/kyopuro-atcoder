use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n]
    }

    let mut set = BTreeSet::new();

    for i in 0..n {
        let (x1, y1) = xy[i];
        for j in 0..n {
            if i == j {
                continue;
            }

            let (x2, y2) = xy[j];

            let mut x = x2 - x1;
            let mut y = y2 - y1;

            let g = gcd(x.abs() as u64, y.abs() as u64).unwrap_or(x.abs().max(y.abs()).max(1) as u64);

            x /= g as i64;
            y /= g as i64;

            set.insert((x, y));
        }
    }

    println!("{}", set.len());
}

fn gcd(x: u64, y: u64) -> Option<u64> {
    let (a, b) = (x.max(y), x.min(y));
    if x == 0 || y == 0 {
        return None;
    }

    if a % b == 0 {
        Some(b)
    } else {
        gcd(b, a % b)
    }
}
