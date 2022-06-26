use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        mut a: [usize; k],
        l: [Usize1; q],
    }

    for l in l {
        let pos = a[l];
        if pos == n {
            continue;
        }
        if !a.iter().any(|&p| p == pos + 1) {
            a[l] += 1;
        }
    }

    println!(
        "{}",
        a.iter().map(|p| p.to_string()).collect_vec().join(" ")
    );
}
