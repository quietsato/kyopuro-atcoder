use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
        ns: [(usize, Chars); t]
    }
    const MOD: u64 = 998244353;
    for (n, s) in &ns {
        let mut ans = 0;
        for c in s.iter().take((n + 1) / 2) {
            ans *= 26;
            ans %= MOD;
            ans += ((*c as u8) - b'A') as u64;
            ans %= MOD;
        } 
        // AAA...AA の場合
        ans += 1;
        ans %= MOD;

        let target = s.iter().take(n / 2).chain(
            s.iter().take((n + 1) / 2).rev()
        ).join("");

        // s=DCBA -> DCCD <= DCBA は成り立たないが， DBBD <= DCBA は成り立つ
        // s=DAAC -> DAAD <= DAAC は成り立たないが， CZZC <= DAAC は成り立つ
        if target > s.iter().join("") {
            ans += MOD - 1;
            ans %= MOD;
        }

        println!("{}", ans);
    }
}
