use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        c: [u64; n]
    }

    // `v = vec![0; 1_000_000_001]` はさすがに怒られたので，
    // `BTreeMap` を使いましょう．
    let mut v = BTreeMap::new();
    let mut tmp = 0u64;
    for c in c.iter().take(k) {
        if let Some(v) = v.get_mut(c) {
            *v += 1;
        } else {
            v.insert(c, 1);
            tmp += 1;
        }
    }
    let mut ans: u64 = tmp;

    for (c0, c1) in c.iter().zip(c.iter().skip(k)) {
        {
            if let Some(w) = v.get_mut(c0) {
                if *w == 1 {
                    v.remove(c0);
                    tmp -= 1;
                } else {
                    *w -= 1;
                }
            }
        }
        {
            if let Some(w) = v.get_mut(c1) {
                *w += 1;
            } else {
                v.insert(c1, 1);
                tmp += 1;
            }
        }

        ans = ans.max(tmp);
    }

    println!("{}", ans);
}
