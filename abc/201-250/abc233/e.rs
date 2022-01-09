use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars
    }

    let mut scan: Vec<u64> = x
        .iter()
        .map(|c| c.to_digit(10).unwrap().into())
        .scan(0, |s, a: u64| {
            *s += a;
            Some(*s)
        })
        .collect_vec();
    scan.reverse();

    let mut ans = vec![0];
    for (i, s) in scan.iter().enumerate() {
        ans[i] += *s;
        ans.push(ans[i] / 10);
        ans[i] %= 10;
    }
    ans.reverse();

    for a in ans.iter().skip_while(|a| **a == 0) {
        print!("{}", a);
    }
    println!();
}
