use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    };
    let mut map = BTreeMap::new();
    for (i, a) in a.iter().enumerate() {
        map.insert(a, i as i64 + 1);
    }
    let mut ans = vec![];

    let mut current = -1;
    while ans.len() < n {
        let &i = map.get(&current).unwrap();
        ans.push(i);
        current = i;
    }

    println!("{}", ans.iter().join(" "));
}
