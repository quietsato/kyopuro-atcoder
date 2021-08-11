use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }

    println!(
        "{}",
        a.iter()
            .enumerate()
            .sorted_by(|x, y| y.1.cmp(x.1))
            .skip(1)
            .next()
            .unwrap()
            .0
            + 1
    );
}
