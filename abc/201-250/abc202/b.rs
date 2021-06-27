use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        mut s: String
    }

    for c in s.chars().rev() {
        print!(
            "{}",
            match c {
                '6' => '9',
                '9' => '6',
                _ => c,
            }
        );
    }

    println!("");

    Ok(())
}

