use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut h: [i64; n],
        w: [i64; m]
    }
    h.sort();

    let w: BTreeSet<_> = w.iter().collect();

    let mut left = vec![0];
    let mut right = vec![0];
    for i in 0..(n / 2) {
        left.push((h[2 * i] - h[2 * i + 1]).abs() + left[i]);
        right.push((h[2 * i + 1] - h[2 * i + 2]).abs() + right[i]);
    }

    let mut ans = std::i64::MAX;
    for i in 0..n {
        let mut local = 0;
        let with_teacher = {
            let mut with_teacher = std::i64::MAX;
            if let Some(&&smaller) = w.range(..=h[i]).last() {
                with_teacher = with_teacher.min((h[i] - smaller).abs());
            }
            if let Some(&&taller) = w.range(h[i]..).next() {
                with_teacher = with_teacher.min((h[i] - taller).abs());
            }
            with_teacher
        };
        local += with_teacher;
        if i % 2 == 1 {
            local += (h[i - 1] - h[i + 1]).abs();
        }

        local += (left[i / 2] - left[0]) + (right[n / 2] - right[i / 2]);

        ans = ans.min(local);
    }

    println!("{}", ans);
}
