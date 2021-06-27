use proconio::input;
use std::{collections::BTreeSet, iter::FromIterator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        x: i64,
        n: usize,
        p: [i64; n]
    }

    let set = BTreeSet::from_iter(p.iter());

    'outer: for i in 0i64.. {
        for sign in &[-1, 1] {
            if !set.contains(&(x + sign * i)) {
                println!("{}", x + sign * i);
                break 'outer;
            }
        }
    }

    Ok(())
}
