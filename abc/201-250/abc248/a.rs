use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    let mut val = vec![false; 10];
    for c in s {
        let i = c.to_digit(10).unwrap() as usize;
        val[i] = true;
    }
    for (i, v) in val.iter().enumerate() {
        if !v {
            println!("{}", i);
            return;
        }
    }
}
