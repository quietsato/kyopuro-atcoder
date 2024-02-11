use std::collections::BTreeMap;

use proconio::input;

fn f(n: u64, memo: &mut BTreeMap<u64, u64>) -> u64 {
    memo.get(&n).map(ToOwned::to_owned).unwrap_or_else(|| {
        let (a, b) = (n / 2, (n + 1) / 2);
        let ans = n + if a != 1 { f(a, memo) } else { 0 } + if b != 1 { f(b, memo) } else { 0 };
        memo.insert(n, ans);
        ans
    })
}

fn main() {
    input! {
        n: u64
    };
    let mut memo = BTreeMap::new();
    println!("{}", f(n, &mut memo));
}
