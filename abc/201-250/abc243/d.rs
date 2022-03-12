use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        mut x: u64,
        s: Chars
    }

    let mut v = vec![];
    while x > 0 {
        v.push(x % 2);
        x /= 2;
    }
    v.reverse();

    for c in s {
        match c {
            'U' => {
                v.pop();
            }
            'L' => {
                v.push(0);
            }
            'R' => {
                v.push(1);
            }
            _ => unreachable!(),
        }
    }

    let mut ans = 0u64;
    let mut mul = 1u64;
    for v in v.iter().rev() {
        ans += mul * v;
        mul *= 2;
    }

    println!("{}", ans);
}
