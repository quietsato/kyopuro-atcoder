use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
    }
    let mut set = BTreeSet::new();    

    let x_diff = vec![-2, -1, 1, 2, 2, 1, -1, -2];
    let y_diff = vec![1, 2, 2, 1, -1, -2, -2, -1];

    for (dx, dy) in x_diff.iter().zip(y_diff.iter()) {
        set.insert((x1 + dx, y1 + dy));
    }

    for (dx, dy) in x_diff.iter().zip(y_diff.iter()) {
        if set.contains(&(x2 + dx, y2 + dy)) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
