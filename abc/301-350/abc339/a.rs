use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };
    println!(
        "{}",
        s.iter()
            .rev()
            .take_while(|&&c| c != '.')
            .collect_vec()
            .iter()
            .rev()
            .join("")
    );
}
