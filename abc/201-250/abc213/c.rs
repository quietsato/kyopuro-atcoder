use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

fn main() {
    input! {
        _h: usize,
        _w: usize,
        n: usize,
        ab: [(usize, usize); n]
    }

    let (mut xs, mut ys) = (BTreeSet::new(), BTreeSet::new());
    for (a, b) in &ab {
        xs.insert(a);
        ys.insert(b);
    }

    let (mut xmap, mut ymap) = (BTreeMap::new(), BTreeMap::new());
    xs.iter().enumerate().for_each(|(i, &&x)| {
        xmap.insert(x, i + 1);
    });
    ys.iter().enumerate().for_each(|(i, &&y)| {
        ymap.insert(y, i + 1);
    });

    for (a, b) in &ab {
        println!("{} {}", xmap.get(a).unwrap(), ymap.get(b).unwrap());
    }
}
