use itertools::Itertools;
use proconio::{input, marker::Usize1};

// 010 Score Sum Queries (★2)
fn main() {
    input! {
        n: usize,
        cp: [(u32, u32); n],
        q: usize,
        lr: [(Usize1, Usize1); q]
    }

    let seq: Vec<(u32, u32)> = cp
        .into_iter()
        .scan((0u32, 0u32), |(s1, s2), (c, p)| {
            match c {
                1 => *s1 += p,
                2 => *s2 += p,
                _ => unreachable!(),
            }
            Some((*s1, *s2))
        })
        .collect_vec();

    dbg!(&seq);

    for &(l, r) in &lr {
        if l == 0 {
            println!("{} {}", seq[r].0, seq[r].1);
        } else {
            println!("{} {}", seq[r].0 - seq[l - 1].0, seq[r].1 - seq[l - 1].1);
        }
    }
}
