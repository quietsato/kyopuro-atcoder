use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (l, r): (Usize1, Usize1),
        s: String
    }

    print!("{}", &s[..l]);
    print!("{}", &s[l..=r].chars().rev().collect::<String>());
    println!("{}", &s[r + 1..]);
}
