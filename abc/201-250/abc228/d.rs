use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

fn main() {
    const N: u64 = 1 << 20;

    input! {
        q: usize,
        tx: [(u64, u64); q]
    }

    let mut map = BTreeMap::new();
    let mut set = BTreeSet::new();
    for i in 0..N {
        set.insert(i);
    }

    for &(t, x) in &tx {
        match t {
            1 => {
                if let Some(&key) = set.range(x % N..).next() {
                    map.insert(key, x);
                    set.remove(&key);
                } else {
                    let &key = set.range(0..).next().unwrap();
                    map.insert(key, x);
                    set.remove(&key);
                }
            }
            2 => {
                if let Some(&v) = map.get(&(x % N)) {
                    println!("{}", v);
                } else {
                    println!("-1");
                }
            }
            _ => unreachable!(),
        }
    }
}
