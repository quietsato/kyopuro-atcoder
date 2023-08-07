use std::collections::{BTreeSet, VecDeque};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String
    };
    let mut seen = BTreeSet::new();
    let mut nexts = VecDeque::from(vec![(s, 0)]);
    while let Some((current, cnt)) = nexts.pop_front() {
        if current == "atcoder" {
            println!("{}", cnt);
            return;
        }
        for i in 0..6 {
            let mut s = current.chars().collect_vec();
            s.swap(i, i + 1);
            let s = s.iter().join("");
            if seen.insert(s.clone()) {
                nexts.push_back((s, cnt + 1));
            }
        }
    }
}
