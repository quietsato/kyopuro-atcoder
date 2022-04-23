use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n]
    }

    let mut map = BTreeMap::new();
    for a in &a {
        if let Some(v) = map.get_mut(a) {
            *v += 1u64;
        } else {
            map.insert(a, 1u64);
        }
    }

    let mut ans = 0u64;
    for x in a.iter() {
        for d in 1..=((*x as f64).sqrt() as u64) {
            if x % d == 0 {
                if let (Some(v1), Some(v2)) = (map.get(&d), map.get(&(x / d))) {
                    ans += v1 * v2;
                    if d != (x / d) {
                        ans += v1 * v2;
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
