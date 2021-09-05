use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n]
    }

    let mut q = vec![0; n];

    for (i, p) in p.iter().enumerate() {
        q[*p - 1] = i + 1;
    }

    println!("{}", q.iter().map(|q| q.to_string()).join(" "));
}
