use itertools::Itertools;
use num_bigint::BigUint;
use proconio::input;

fn main() {
    input! {
        s: u64
    }

    dbg!(s / 3);

    let ans = (1..=(s / 3))
        .map(|k| {
            let mut v = BigUint::from(1u64);
            for i in 1..k {
                v *= BigUint::from(s - 2 * k - i);
                v /= BigUint::from(i);
            }
            v.modpow(&BigUint::from(1u64), &BigUint::from(1_000_000_007u64))
                .to_string()
                .parse::<u64>()
                .unwrap()
        })
        .fold1(|x, y| (x + y) % 1_000_000_007)
        .unwrap_or(0);

    println!("{}", ans);
}
