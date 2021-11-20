use num_bigint::BigUint;
use num_traits::ToPrimitive;
use proconio::input;

fn main() {
    input! {
        n: u64,
        k: u64,
        m: u64
    }

    let p = BigUint::from(998244353u64);

    let n = BigUint::from(n);
    let k = BigUint::from(k);
    let m = BigUint::from(m);

    let q = &p - &BigUint::from(1u64);
    let r = if &k % &q == BigUint::from(0u64) {
        BigUint::from(0u64)
    } else {
        k.modpow(&n, &q)
    };

    if &m % &p == BigUint::from(0u64) {
        println!("0");
    } else {
        println!("{}", m.modpow(&r, &p).to_u64().unwrap());
    }
}
