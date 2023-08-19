use std::collections::BTreeMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut t: u64,
        a: [u64; n - 1],
        xy: [(Usize1, u64); m]
    };
    let xy = {
        let mut map = BTreeMap::new();
        for (x, y) in xy {
            map.insert(x, y);
        }
        map
    };
    for (i, a) in a.iter().enumerate() {
        t += xy.get(&i).unwrap_or(&0);
        if a >= &t {
            println!("No");
            return;
        }
        t -= a;
    }
    println!("Yes");
}
