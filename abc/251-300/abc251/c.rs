use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, u64); n]
    }
    let mut known = BTreeSet::new();
    let mut award_t = 0;
    let mut ans = 0;
    for (i, (s, t)) in st.into_iter().enumerate() {
        if known.contains(&s) {
            continue;
        }
        known.insert(s.clone());
        if award_t < t {
            award_t = t;
            ans = i + 1;
        }
    }
    println!("{}", ans);
}
