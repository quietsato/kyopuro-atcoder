use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        q: usize
    }

    let mut map = BTreeMap::new();
    let mut offset = 0;

    for _ in 0..q {
        input! {i: u8}

        match i {
            1 => {
                input! {x: i64}
                let x = x - offset;
                if let Some(v) = map.get_mut(&x) {
                    *v += 1;
                } else {
                    map.insert(x, 1);
                }
            }
            2 => {
                input! {x: i64}
                offset += x;
            }
            3 => {
                let (k, v) = {
                    let (k, v) = map.iter_mut().next().unwrap();
                    println!("{}", k + offset);
                    *v -= 1;
                    (*k, *v)
                };

                if v == 0 {
                    map.remove(&k);
                }
            }
            _ => unreachable!(),
        }
    }
}
