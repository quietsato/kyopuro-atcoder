use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut v = VecDeque::new();
    v.push_back(n);

    for (i, c) in s.iter().rev().enumerate() {
        match c {
            'L' => v.push_back(n - i - 1),
            'R' => v.push_front(n - i - 1),
            _ => unreachable!(),
        }
    }

    println!("{}", v.iter().join(" "));
}
