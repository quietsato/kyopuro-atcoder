#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools,
    whiteread::parse_line
};

use std::*;

fn main() {
    let n: u64 = parse_line().unwrap();

    let ans: u64 = (1..=n)
        .into_iter()
        .map(|i| {
            let e = n / i;
            e * (i + i * e) / 2
        })
        .sum();

    println!("{}", ans);
}
