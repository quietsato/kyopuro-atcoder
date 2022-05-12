use std::{collections::BTreeSet, ops::Range};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        b: [u64; n],
        q: usize,
        xy: [(Usize1, Usize1); q]
    }

    let mut both = BTreeSet::new();
    let mut a_only = BTreeSet::new();
    let mut b_only = BTreeSet::new();
    let mut yes: Vec<Range<usize>> = vec![];

    let mut j = 0;
    (0..n).for_each(|i| {
        if !both.contains(&a[i]) {
            if b_only.contains(&a[i]) {
                b_only.remove(&a[i]);
                both.insert(&a[i]);
            } else {
                a_only.insert(&a[i]);
            }
        }

        if a_only.is_empty() && b_only.is_empty() {
            yes.push(yes.last().unwrap().clone());
            return;
        }

        while j < n {
            if !both.contains(&b[j]) {
                if a_only.contains(&b[j]) {
                    a_only.remove(&b[j]);
                    both.insert(&b[j]);
                } else {
                    b_only.insert(&b[j]);
                }
            }
            if a_only.is_empty() && b_only.is_empty() {
                break;
            }
            if b_only.is_empty() {
                j += 1;
            } else {
                break;
            }
        }

        if !a_only.is_empty() || !b_only.is_empty() {
            yes.push(n..n);
            return;
        }

        let l = j;
        let mut r = j + 1;

        while r < n {
            if both.contains(&b[r]) {
                r += 1;
            } else {
                break;
            }
        }

        yes.push(l..r);
    });

    for (x, y) in xy {
        if yes[x].start <= y && y < yes[x].end {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
