use proconio::input;
use std::{self, collections};

fn main() {
    input! {
        n: usize,
        c: [u64; n]
    }

    const MOD: u64 = 1_000_000_007;

    let mut map = collections::BTreeMap::new();
    for c in c {
        if let Some(v) = map.get_mut(&c) {
            *v += 1;
        } else {
            map.insert(c, 1u64);
        }
    }

    let mut ans = 1;
    let mut tmp_sum = 0;
    for (x, v) in map.iter() {
        if v > x {
            ans = 0;
        } else {
            for i in 1..=*v {
                ans *= (x - tmp_sum) - i + 1;
                ans %= MOD;
            }
        }
        tmp_sum += v;
    }

    println!("{}", ans);
}
