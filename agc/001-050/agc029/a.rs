#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools,
    whiteread::parse_line
};

use std::*;

fn main() {
    let s: String = parse_line::<String>()
        .unwrap()
        .trim_end_matches('B')
        .trim_start_matches('W')
        .to_string();
    let l = s.len();

    let ans: u64 = {
        let b: Vec<u64> = s
            .chars()
            .enumerate()
            .filter_map(|(i, c)| {
                if c == 'B' {
                    Some((l - i - 1) as u64)
                } else {
                    None
                }
            })
            .collect();
        let x: u64 = b.iter().sum();
        let m = f(b.len());
        x - m
    };

    println!("{}", ans);
}

fn f(n: usize) -> u64 {
    if n <= 1 {
        0
    } else {
        (n - 1) as u64 + f(n - 1)
    }
}
