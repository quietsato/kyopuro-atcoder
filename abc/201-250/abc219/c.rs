use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
        n: usize,
        s: [Chars; n]
    }

    let mut idx = BTreeMap::new();
    for (i, c) in x.iter().enumerate() {
        let i = i as u32;
        idx.insert(c, i);
    }

    let s_ = s
        .iter()
        .map(|v| {
            v.iter()
                .map(|c| std::char::from_u32('a' as u32 + idx.get(c).unwrap()).unwrap())
                .collect::<String>()
        })
        .collect_vec();

    let mut s = s
        .iter()
        .map(|chars| chars.iter().collect::<String>())
        .zip(s_.iter())
        .collect_vec();
    s.sort_by(|a, b| a.1.cmp(b.1));

    for s in s {
        println!("{}", s.0);
    }
}
