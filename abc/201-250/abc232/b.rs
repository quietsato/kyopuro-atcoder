use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let diff = s
        .iter()
        .zip(t.iter())
        .map(|(s, t)| ((*s as u32 + 26) as i64 - (*t as u32) as i64) % 26)
        .collect_vec();

    if diff.iter().unique().count() == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
