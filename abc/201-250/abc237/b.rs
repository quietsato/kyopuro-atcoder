use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i64; w]; h]
    }

    let s = (0..w).map(|i| (0..h).map(|j| a[j][i]).join(" ")).join("\n");

    println!("{}", s);
}
