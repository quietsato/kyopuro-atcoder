use std::collections::BTreeSet;

use proconio::input;

// 007 CP Class (★3)
fn main() {
    input! {
        n: usize,
        a: [i64; n],
        q: usize,
        b: [i64; q]
    }

    let a_set: BTreeSet<_> = a.iter().collect();
    for &b in &b {
        let l = a_set.range(..b).last();
        let r = a_set.range(b..).next();
        let ans = if l.is_none() {
            (b - **r.unwrap()).abs()
        } else if r.is_none() {
            (b - **l.unwrap()).abs()
        } else {
            (b - **l.unwrap()).abs().min((b - **r.unwrap()).abs())
        };

        println!("{}", ans);
    }
}
