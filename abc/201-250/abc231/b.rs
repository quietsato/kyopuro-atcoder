use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }
    let mut ans = BTreeMap::new();
    for s in s {
        if let Some(v) = ans.get_mut(&s) {
            *v += 1;
        } else {
            ans.insert(s, 0);
        }
    }
    let a = ans.into_iter().max_by_key(|(_, c)| *c).unwrap().0;

    println!("{}", a);
}
