use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut ab: [(Usize1, Usize1); n]
    };

    let mut v = vec![None; 2 * n];
    for (i, &(a, b)) in ab.iter().enumerate() {
        let (a, b) = (a.min(b), a.max(b));
        // 交差していなければ上書きされない
        v[a] = Some((true, i));
        v[b] = Some((false, i));
    }
    let mut s = VecDeque::new();
    for v in v {
        if let Some((is_begin, id)) = v {
            if is_begin {
                s.push_back(id);
            } else {
                if s.pop_back().unwrap() != id {
                    println!("Yes");
                    return;
                }
            }
        } else {
            continue;
        }
    }
    println!("No");
}
