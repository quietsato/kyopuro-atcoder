use std::{collections::BTreeSet, iter::FromIterator};

use proconio::input;

fn main() {
    input! {
        l: usize,
        q: usize,
        cx: [(char, usize); q]
    }

    let mut xs = BTreeSet::from_iter([0, l].iter().cloned());
    dbg!(&xs);

    for &(c, x) in &cx {
        match c {
            '1' => {
                xs.insert(x);
            }
            '2' => {
                let (&l, &r) = (xs.range(..x).last().unwrap(), xs.range(x..).next().unwrap());
                println!("{}", r - l);
            }
            _ => unreachable!(),
        }
    }
}
