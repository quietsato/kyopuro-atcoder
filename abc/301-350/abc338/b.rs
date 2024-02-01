use proconio::{input, marker::Chars};
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    input! {
        s: Chars
    };
    let mut map = BTreeMap::new();
    for c in s {
        match map.get_mut(&c) {
            Some(v) => {
                *v += 1;
            }
            None => {
                map.insert(c, 1);
            }
        }
    }

    let mut revmap = BTreeMap::new();
    for (key, val) in map.iter() {
        match revmap.get_mut(val) {
            None => {
                revmap.insert(val, BTreeSet::from([key]));
            }
            Some(v) => {
                v.insert(key);
            }
        }
    }

    let (_, keys) = revmap.iter().next_back().unwrap();
    println!("{}", keys.iter().next().unwrap());
}
