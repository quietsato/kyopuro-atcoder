use std::collections::BTreeMap;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a:[u64; n]
    }

    let mut set = BTreeMap::new();

    for i in a {
        if let Some(s) = set.get_mut(&i) {
            *s += 1;
        } else {
            set.insert(i, 1u64);
        }
    }

    let mut ans = (n * (n - 1) / 2) as u64;
    for (_, m) in set {
        ans -= m * (m - 1) / 2;
    }

    println!("{}", ans);
}
