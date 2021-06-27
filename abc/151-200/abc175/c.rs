#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools,
    whiteread::parse_line
};

use std::*;

fn main() {
    let (x, k, d): (i64, i64, i64) = parse_line().unwrap();
    let x = x.abs();

    let ans = {
        let a = x / d;
        if a > k {
            x - d * k
        } else {
            if (k - a) & 1 == 0 {
                x - d * a
            } else {
                (x - d * (a + 1)).abs()
            }
        }
    };

    println!("{}", ans);
}
