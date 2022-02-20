use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    println!("{}", a.iter().unique().count());
}
