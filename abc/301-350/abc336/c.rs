use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        mut n: Usize1
    };
    let mut ans = vec![];
    loop {
        ans.push(2 * (n % 5));
        n /= 5;
        if n == 0 {
            break;
        }
    }
    println!("{}", ans.iter().rev().join(""));
}
