use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        a: [i64; n]
    }

    let a = a
        .iter()
        .enumerate()
        .map(|(i, a)| *a - (i & 1) as i64)
        .collect_vec();

    if a.iter().sum::<i64>() <= x {
        println!("Yes");
    } else {
        println!("No");
    }
}
