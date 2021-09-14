use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
        mut b: [u32; n],
    }
    b.sort();
    let b = b;

    let xs = b.iter().map(|b| a[0] ^ b).collect_vec();

    let mut ans = BTreeSet::new();
    xs.iter()
        .filter_map(|x| {
            let c = a.iter().map(|a| a ^ x).sorted().collect_vec();
            if c == b {
                Some(x)
            } else {
                None
            }
        })
        .for_each(|x| {
            ans.insert(x);
        });

    println!("{}", &ans.len());
    for x in ans {
        println!("{}", x);
    }
}
