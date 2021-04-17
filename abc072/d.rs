#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools,
    whiteread::parse_line
};

use std::*;

fn main() {
    let n: usize = parse_line().unwrap();
    let mut p: Vec<u64> = parse_line().unwrap();

    let ans = (0..n)
        .filter_map(|i| {
            if p[i] == (i + 1) as u64 {
                if i < n - 1 {
                    p.swap(i, i + 1);
                } else {
                    p.swap(i, i - 1);
                }
                Some(())
            } else {
                None
            }
        })
        .count();

    println!("{}", ans);
}
