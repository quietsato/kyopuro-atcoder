use std::collections::BTreeMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        w: [u64;n],
    }

    let mut map = BTreeMap::<u64, i64>::new();
    for (s, w) in s.iter().zip(w.iter()) {
        if !map.contains_key(w) {
            map.insert(*w, 0);
        }
        *map.get_mut(w).unwrap() += if *s == '0' { -1 } else { 1 };
    }

    let mut ans = vec![s.iter().filter(|&&s| s == '0').count() as i64];
    for (_, &diff) in map.iter().rev() {
        ans.push(ans.last().unwrap() + diff);
    }

    println!("{}", ans.iter().max().unwrap());
}
