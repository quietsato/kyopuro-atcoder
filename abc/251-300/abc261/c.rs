use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String; n]
    }

    let mut map = BTreeMap::new();

    for s in ss {
        if let Some(cnt) = map.get_mut(&s) {
            *cnt += 1;
            println!("{}({})", &s, cnt);
        } else {
            map.insert(s.clone(), 0);
            println!("{}", &s);
        }
    }
}
