use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    let n = s.len();
    println!("0{}", s.iter().take(n - 1).collect::<String>());
}
