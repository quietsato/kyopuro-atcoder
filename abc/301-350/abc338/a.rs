use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let b1 = s.first().unwrap().is_ascii_uppercase();
    let b2 = s.iter().skip(1).all(|c| c.is_ascii_lowercase());
    println!("{}", if b1 && b2 { "Yes" } else { "No" });
}
