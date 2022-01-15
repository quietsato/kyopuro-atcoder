use std::{
    collections::{BTreeMap, BTreeSet},
    slice::SliceIndex,
};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }

    let mut map = BTreeMap::new();
    let mut vec = vec![];
    for a in a {
        if let Some(v) = map.get_mut(&a) {
            *v = false;
        } else {
            map.insert(a, true);
            vec.push(a);
        }
    }

    for a in vec {
        for m in 2.. {
            if a * m > 1_000_000 {
                break;
            }
            if let Some(v) = map.get_mut(&(a * m)) {
                *v = false;
            }
        }
    }

    println!("{}", map.iter().filter(|(_key, &val)| val).count())
}
