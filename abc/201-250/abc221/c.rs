use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let s = s.iter().map(|c| c.to_digit(10).unwrap()).collect_vec();

    let mut ans = 0;

    for s in s.iter().permutations(s.len()) {
        if *s[0] == 0 {
            continue;
        }

        for i in 1..s.len() {
            if *s[i] == 0 {
                continue;
            }
            let x = (s[..i]).iter().fold(0, |s, v| s * 10 + **v);
            let y = (s[i..]).iter().fold(0, |s, v| s * 10 + **v);
            ans = ans.max(x * y);
        }
    }

    println!("{}", ans);
}
