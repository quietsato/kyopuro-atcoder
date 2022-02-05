use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }

    let mut a = a.iter().scan(0, |s, a| {
        *s += a;
        Some(*s)
    }).collect_vec();

    a.insert(0, 0);
    a.iter_mut().for_each(|s| *s %= 360);
    a.push(360);
    a.sort();

    let ans = a
        .iter()
        .zip(a.iter().skip(1))
        .map(|(a, b)| b - a)
        .max()
        .unwrap();

    println!("{}", ans);
}
