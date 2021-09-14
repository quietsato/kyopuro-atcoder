use proconio::input;

fn main() {
    input! {
        n: u64
    }

    const MOD: u64 = 998244353;
    let ans: u64 = (1..)
        .take_while(|q| q * q <= n)
        .map(|q| ((n - q * q) / (2 * q) + 1) % MOD) // because q \equiv p mod 2
        .fold(0, |s, a| (s + a) % MOD);

    println!("{}", ans);
}
