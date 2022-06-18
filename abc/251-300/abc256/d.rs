use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        lr: [(Usize1, Usize1); n]
    }

    let mut a = vec![0; 200_001];
    for (l, r) in lr {
        a[l] += 1;
        a[r] -= 1;
    }

    let a = a
        .iter()
        .scan(0, |s, a| {
            *s += a;
            Some(*s)
        })
        .collect_vec();

    let mut l: Option<usize> = None;
    for (i, &a) in a.iter().enumerate() {
        if a == 0 {
            if let Some(ll) = l {
                println!("{} {}", ll + 1, i + 1);
                l = None;
            } else {
                // Do Nothing
            }
        }
        if a > 0 && l.is_none() {
            l = Some(i);
        }
    }
}
