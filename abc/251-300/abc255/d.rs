use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i64; n],
        x: [i64; q]
    }

    a.sort_unstable();
    let a = a;

    let mut scan = a
        .iter()
        .scan(0, |s, a| {
            *s += a;
            Some(*s)
        })
        .collect_vec();
    scan.insert(0, 0);

    for x in x {
        let ans = match a.binary_search(&x) {
            Ok(i) => {
                // 大きい方
                let larger = (scan[n] - scan[i]) - (n - i) as i64 * x;
                // 小さい方
                let smaller = i as i64 * x - scan[i];
                larger + smaller
            }
            Err(i) => {
                if i == 0 {
                    scan[n] - (x * n as i64)
                } else if i == n {
                    (x * n as i64) - scan[n]
                } else {
                    // 大きい方
                    let larger = (scan[n] - scan[i]) - (n - i) as i64 * x;
                    // 小さい方
                    let smaller = i as i64 * x - scan[i];
                    larger + smaller
                }
            }
        };

        println!("{}", ans);
    }
}
