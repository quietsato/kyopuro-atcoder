use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
        x3: i64,
        y3: i64,
    }
    let mut x_set = BTreeSet::new();
    let mut y_set = BTreeSet::new();

    let x = vec![x1, x2, x3];
    let y = vec![y1, y2, y3];

    for x in x {
        if x_set.contains(&x) {
            x_set.remove(&x);
        } else {
            x_set.insert(x);
        }
    }
    for y in y {
        if y_set.contains(&y) {
            y_set.remove(&y);
        } else {
            y_set.insert(y);
        }
    }
    println!(
        "{} {}",
        x_set.iter().next().unwrap(),
        y_set.iter().next().unwrap()
    );
}
