use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: u64,
        a: [u64; n]
    }

    let i = a.iter().sorted().collect_vec();

    let offset = k / n as u64;
    k %= n as u64;

    let key = *i[k as usize];

    for a in a {
        println!("{}", if a < key { offset + 1 } else { offset });
    }
}
