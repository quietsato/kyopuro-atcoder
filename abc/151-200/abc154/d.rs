use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [f64; n],
    };
    let accm = p
        .iter()
        .scan(0.0, |s, a| {
            *s += a + 1.0;
            Some(*s)
        })
        .collect_vec();

    let ans = accm
        .iter()
        .zip(accm.iter().skip(k))
        .map(|(a, b)| (b - a) / 2.0)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(accm.last().unwrap() / 2.0);

    println!("{}", ans);
}
