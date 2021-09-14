use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars
    }

    let c1 = {
        let mut set = HashSet::new();
        x.iter().for_each(|x| {
            set.insert(x);
        });
        set.len() == 1
    };
    let c2 = {
        let mut bool = true;
        let x = x.iter().map(|c| c.to_digit(10).unwrap()).collect_vec();
        for i in 1..4 {
            bool &= if x[i] == 0 {
                x[i - 1] == 9
            } else {
                x[i] == x[i - 1] + 1
            };
        }
        bool
    };

    if c1 || c2 {
        println!("Weak");
    } else {
        println!("Strong");
    }
}
