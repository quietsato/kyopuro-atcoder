use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [String; 3],
        t: Chars
    }

    println!(
        "{}",
        t.iter()
            .map(|c| { s[c.to_digit(10).unwrap() as usize - 1].clone() })
            .join("")
    );
}
