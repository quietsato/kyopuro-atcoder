use std::{
    collections::{self, HashSet},
    iter::FromIterator,
};

use proconio::input;

fn main() {
    input! {
        ss: [String; 4]
    }

    let set = collections::HashSet::<&String>::from_iter(ss.iter());
    if set.len() == 4 {
        println!("Yes");
    } else {
        println!("No");
    }
}
