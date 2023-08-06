use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        a: [u32; 5]
    };

    let mut cnt = BTreeMap::new();

    for a in &a {
        if let Some(v) = cnt.get_mut(a) {
            *v += 1;
        } else {
            cnt.insert(a, 1);
        }
    }

    let cnt = cnt.values().sorted().collect_vec();
    println!(
        "{}",
        if cnt.len() == 2 && *cnt[0] == 2 && *cnt[1] == 3 {
            "Yes"
        } else {
            "No"
        }
    );
}
