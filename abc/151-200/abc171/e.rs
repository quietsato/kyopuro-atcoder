use std::ops::BitXor;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }

    let all = a.iter().fold(0, |s, a| s.bitxor(a));
    for a in a {
        println!("{}", a.bitxor(all));
    }
}
