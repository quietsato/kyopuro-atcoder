use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut set = BTreeSet::new();

    let has_upper = s.iter().any(|c| c.is_uppercase());
    let has_lower = s.iter().any(|c| c.is_lowercase());
    if has_upper && has_lower {
        for c in s {
            if set.contains(&c) {
                println!("No");
                return;
            }
            set.insert(c);
        }
        println!("Yes");
    } else {
        println!("No");
    }
}
