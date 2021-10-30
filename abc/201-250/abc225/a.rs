use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String
    }

    let s = s.chars().unique().collect_vec();

    let ans = match s.len() {
        1 => 1,
        2 => 3,
        3 => 6,
        _ => 0
    };
    println!("{}", ans);
}
