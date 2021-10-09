use std::cmp::Reverse;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; 2 * n]
    }

    let mut lb = Vec::with_capacity(2 * n);
    for i in 0..2 * n {
        lb.push((i, 0));
    }

    for i in 0..m {
        for k in 0..n {
            let (a_hand, b_hand) = (a[lb[2 * k].0][i], a[lb[2 * k + 1].0][i]);
            if (a_hand == 'G' && b_hand == 'C')
                || (a_hand == 'C' && b_hand == 'P')
                || (a_hand == 'P' && b_hand == 'G')
            {
                lb[2 * k].1 += 1;
            } else if (b_hand == 'G' && a_hand == 'C')
                || (b_hand == 'C' && a_hand == 'P')
                || (b_hand == 'P' && a_hand == 'G')
            {
                lb[2 * k + 1].1 += 1;
            }
        }
        lb.sort_by_key(|&(i, win)| (Reverse(win), i));
    }

    for (i, _) in &lb {
        println!("{}", i + 1);
    }
}
