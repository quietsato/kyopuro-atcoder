use std::collections::BTreeMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
        xk: [(u64, Usize1); q]
    }

    let mut map = BTreeMap::<u64, Vec<usize>>::new();

    for (i, a) in a.into_iter().enumerate() {
        if let Some(v) = map.get_mut(&a) {
            v.push(i + 1);
        } else {
            map.insert(a, vec![i + 1]);
        }
    }

    for &(x, k) in &xk {
        if let Some(v) = map.get(&x) {
            if let Some(ans) = v.get(k) {
                println!("{}", ans);
            } else {
                println!("-1");
            }
        } else {
            println!("-1");
        }
    }
}
