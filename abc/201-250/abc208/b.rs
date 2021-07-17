use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut p: u64
    }

    let c = (1..=10u64)
        .scan(1u64, |s, c| {
            *s *= c;
            Some(*s)
        })
        .collect_vec();

    let mut ans = 0;
    for c in c.iter().rev() {
        while c <= &p {
            p -= c;
            ans += 1;
        }
    }

    println!("{}", ans);
}
