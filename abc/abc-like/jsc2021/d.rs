use itertools::Itertools;
use num_bigint::BigUint;
use whiteread::parse_line;

fn main() {
    let MOD = BigUint::from(1_000_000_007u64);

    let (n, p): (u64, u64) = parse_line().unwrap();

    let ans = {
        let mut ans = BigUint::from(p - 1);
        ans *= BigUint::from(p - 2).modpow(&BigUint::from(n - 1), &MOD);
        ans %= MOD;

        ans.to_string().parse::<u64>().unwrap()
    };

    println!("{}", ans);
}
