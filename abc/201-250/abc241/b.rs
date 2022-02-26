use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u64; n],
        b: [u64; m]
    }
    let mut a = a.into_iter().map(|a| (a, true)).collect_vec();

    for b in b {
        if let Some(i) = a.iter().position(|(l, available)| *l == b && *available) {
            a[i].1 = false;
        } else {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
