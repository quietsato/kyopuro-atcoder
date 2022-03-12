use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
        s: String
    }

    let mut map = BTreeMap::<i64, Vec<(i64, char)>>::new();
    for (c, (x, y)) in s.chars().zip(xy.iter()) {
        if let Some(v) = map.get_mut(y) {
            v.push((*x, c));
        } else {
            map.insert(*y, vec![(*x, c)]);
        }
    }

    let v = map
        .iter_mut()
        .map(|(_y, row)| {
            row.sort_unstable();
            row
        })
        .collect_vec();

    for row in v {
        let r_first = row.iter().find_position(|&&c| c.1 == 'R');
        if let Some((r_first, _)) = r_first {
            if row.iter().skip(r_first).any(|&c| c.1 == 'L') {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
