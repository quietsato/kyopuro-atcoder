use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars
    };
    let ans = if s.iter().zip(s.iter().skip(1)).all(|(x, y)| x <= y) {
        "Yes"
    } else {
        "No"
    };
    println!("{ans}");
}
