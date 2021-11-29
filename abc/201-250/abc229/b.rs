use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut a: Chars,
        mut b: Chars
    }

    a.reverse();
    b.reverse();

    for (a, b) in a.iter().zip(b.iter()) {
        if a.to_digit(10).unwrap() + b.to_digit(10).unwrap() >= 10 {
            println!("Hard");
            return;
        }
    }

    println!("Easy");
}
