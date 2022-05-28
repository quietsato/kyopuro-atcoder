use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut map = BTreeMap::new();

    for _ in 0..q {
        input! {
            c: u8
        }

        match c {
            1 => {
                input! {
                    x: u64
                }
                if let Some(v) = map.get_mut(&x) {
                    *v += 1;
                } else {
                    map.insert(x, 1);
                }
            }
            2 => {
                input! {
                    x: u64,
                    c: u64,
                }
                if let Some(v) = map.get_mut(&x) {
                    if *v <= c {
                        map.remove(&x);
                    } else {
                        *v -= c;
                    }
                }
            }
            3 => {
                let max = map.keys().next_back().unwrap();
                let min = map.keys().next().unwrap();
                println!("{}", max - min);
            }
            _ => unreachable!(),
        }
    }
}
