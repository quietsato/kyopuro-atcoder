use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(u32, u32); n]
    }

    let mut map: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();

    for (x, y) in xy {
        if let Some(v) = map.get_mut(&y) {
            v.insert(x);
        } else {
            let mut set = BTreeSet::new();
            set.insert(x);
            map.insert(y, set);
        }
    }

    let mut ans = 0usize;
    for comb in map.iter().combinations(2) {
        let (s, t) = (comb[0].1, comb[1].1);
        let sum = s.intersection(t).count();
        if sum >= 2 {
            ans += sum * (sum - 1) / 2; // sum \choose 2
        }
    }

    println!("{}", ans);
}
