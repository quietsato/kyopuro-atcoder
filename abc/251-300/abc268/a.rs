use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: [u32; 5]
    };

    let set = BTreeSet::from_iter(n);

    println!("{}", set.len());
}
