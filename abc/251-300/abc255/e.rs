use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [i64; n - 1],
        x: [i64; m]
    }

    let mut b = vec![0];

    for s in s {
        b.push(s - b.last().unwrap());
    }

    let mut map = BTreeMap::new();

    for (i, b) in b.iter().enumerate() {
        for x in &x {
            let z = (-1i64).pow(i as u32) * (x - b);
            if let Some(v) = map.get_mut(&z) {
                *v += 1;
            } else {
                map.insert(z, 1);
            }
        }
    }

    let ans = map.iter().map(|(_, v)| v).max().unwrap();
    println!("{}", ans);
}
