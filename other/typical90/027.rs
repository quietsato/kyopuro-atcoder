use std::collections::BTreeSet;

use proconio::input;

// 027 Sign Up Requests (★2)
fn main() {
    input! {
        n: usize,
        ss: [String; n]
    }
    let mut set = BTreeSet::new();
    for (i, s) in ss.iter().enumerate() {
        if set.insert(s) {
            println!("{}", i + 1);
        }
    }
}
