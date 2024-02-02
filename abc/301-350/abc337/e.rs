use std::io::Write;

use itertools::Itertools;
use proconio::{input_interactive, marker::Chars};

fn main() {
    input_interactive! {
        n: u32
    };
    let n = n - 1;
    let mut ask = vec![];
    for i in 0.. {
        let b = 2u32.pow(i);
        if b > n {
            break;
        }
        ask.push((1..=n).filter(|x| x & b != 0).collect_vec());
    }
    println!("{}", ask.len());
    for a in ask {
        println!("{} {}", a.len(), a.iter().join(" "));
    }

    input_interactive! {
        s: Chars,
    }

    let mut ans = 0;
    for (i, c) in s.iter().enumerate() {
        ans += 2u32.pow(i as u32) * (*c == '1').then(|| 1).unwrap_or(0);
    }
    println!("{}", (ans == 0).then(|| n + 1).unwrap_or(ans));
}
