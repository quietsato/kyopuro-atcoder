use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars
    }

    println!(
        "{}",
        match n.last().unwrap() {
            '2' | '4' | '5' | '7' | '9' => "hon",
            '0' | '1' | '6' | '8' => "pon",
            '3' => "bon",
            _ => unreachable!(),
        }
    )
}
