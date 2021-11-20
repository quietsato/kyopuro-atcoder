use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [[i64; 3]; n]
    }

    let p = p.iter().map(|p| p.iter().sum::<i64>()).collect_vec();

    let mut map = BTreeMap::new();
    for &p in &p {
        if let Some(v) = map.get_mut(&p) {
            *v += 1;
        } else {
            map.insert(p, 1);
        }
    }

    for p in p {
        if map
            .range(p + 301..)
            .into_iter()
            .map(|(_, v)| v)
            .sum::<usize>()
            >= k
        {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
