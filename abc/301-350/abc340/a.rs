use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        d: usize,
    };
    println!("{}", (a..=b).step_by(d).join(" "));
}
