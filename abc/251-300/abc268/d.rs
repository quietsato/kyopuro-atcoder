use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    };

    let t = BTreeSet::from_iter(t);

    for words in s.iter().permutations(n) {
        let words_len_sum = words.iter().map(|w| w.len()).sum::<usize>();

        if 16 - words_len_sum < words.len() - 1 {
            // max length
            continue;
        }
        if words.len() == 1 && (3..=16).contains(&words[0].len()) {
            if !t.contains(words[0]) {
                println!("{}", words[0]);
                return;
            }
        }

        for underlines in (0..=16 - words_len_sum).combinations(n) {
            let mut words = words.iter().map(|&w| w.clone()).collect_vec();
            for (i, (u0, u1)) in underlines.iter().zip(underlines.iter().skip(1)).enumerate() {
                words.insert(2 * i + 1, "_".repeat(u1 - u0));
            }
            let s = words.join("");
            if s.len() < 3 {
                continue;
            }
            if !t.contains(&s) {
                println!("{}", s);
                return;
            }
        }
    }

    println!("-1");
}
