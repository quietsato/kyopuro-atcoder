use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        n: usize,
        xy: [(i64, i64); n]
    }

    let is = (0..n).collect_vec();

    let x = {
        let mut x = is.clone();
        x.sort_by(|&a, &b| xy[a].0.cmp(&xy[b].0));
        x
    };
    let y = {
        let mut y = is.clone();
        y.sort_by(|&a, &b| xy[a].1.cmp(&xy[b].1));
        y
    };

    let mut set = BTreeSet::new();
    set.insert(&x[0]);
    set.insert(&x[1]);
    set.insert(&x[n - 2]);
    set.insert(&x[n - 1]);
    set.insert(&y[0]);
    set.insert(&y[1]);
    set.insert(&y[n - 2]);
    set.insert(&y[n - 1]);

    let mut v = vec![];
    for is in set.iter().combinations(2) {
        v.push(
            (xy[**is[0]].0 - xy[**is[1]].0)
                .abs()
                .max((xy[**is[0]].1 - xy[**is[1]].1).abs()),
        );
    }
    v.sort();
    v.reverse();

    println!("{}", v[1]);

    Ok(())
}
