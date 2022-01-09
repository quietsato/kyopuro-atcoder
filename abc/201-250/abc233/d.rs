use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n]
    }

    let mut scan = a
        .iter()
        .scan(0, |s, a| {
            *s += a;
            Some(*s)
        })
        .collect::<Vec<_>>();
    scan.insert(0, 0);

    let mut map = BTreeMap::new();

    let mut ans = 0usize;
    for r in 1..=n {
        let l = r - 1;
        let key = scan[l];
        if let Some(v) = map.get_mut(&key) {
            *v += 1;
        } else {
            map.insert(key, 1);
        }

        ans += map.get(&(scan[r] - k)).unwrap_or(&0);
    }

    println!("{}", ans);
}
