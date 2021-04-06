#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools,
    whiteread::parse_line
};

use std::*;

fn main() {
    let (n, m, k): (usize, usize, u64) = parse_line().unwrap();
    let a: Vec<u64> = Iterator::chain([0].iter(), parse_line::<Vec<u64>>().unwrap().iter())
        .scan(0u64, |x, e| {
            *x += *e;
            Some(*x)
        })
        .collect();
    let b: Vec<u64> = Iterator::chain([0].iter(), parse_line::<Vec<u64>>().unwrap().iter())
        .scan(0u64, |x, e| {
            *x += *e;
            Some(*x)
        })
        .collect();

    let ans = {
        let mut ans = 0;

        let mut b_count = 0;

        for a_count in (0..=n).rev() {
            if a[a_count] > k {
                continue;
            }

            while b_count <= m && a[a_count] + b[b_count] <= k {
                ans = ans.max(a_count + b_count);

                b_count += 1;
            }
        }

        ans
    };

    println!("{}", ans);
}
