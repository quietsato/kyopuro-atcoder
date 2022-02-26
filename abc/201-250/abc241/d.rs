use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet},
    slice::SliceIndex,
};

use proconio::input;

fn main() {
    input! {
        q: usize
    }
    let mut map_asc = BTreeMap::<u64, u64>::new();
    let mut map_dsc = BTreeMap::<Reverse<u64>, u64>::new();
    'for_query: for _ in 0..q {
        input! {op: u8}
        match op {
            1 => {
                input! {x: u64}
                if let Some(v) = map_asc.get_mut(&x) {
                    *v += 1;
                } else {
                    map_asc.insert(x, 1);
                }
                if let Some(v) = map_dsc.get_mut(&Reverse(x)) {
                    *v += 1;
                } else {
                    map_dsc.insert(Reverse(x), 1);
                }
            }
            2 => {
                input! {x: u64, mut k: u64}
                for (key, val) in map_dsc.range(Reverse(x)..) {
                    if val >= &k {
                        println!("{}", key.0);
                        continue 'for_query;
                    } else {
                        k -= val;
                    }
                }
                println!("-1");
            }
            3 => {
                input! {x: u64, mut k: u64}
                for (key, val) in map_asc.range(x..) {
                    if val >= &k {
                        println!("{}", key);
                        continue 'for_query;
                    } else {
                        k -= val;
                    }
                }
                println!("-1");
            }
            _ => unreachable!(),
        }
    }
}
