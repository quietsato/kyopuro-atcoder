use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i64,
        q: usize,
        abcd: [(usize, usize, i64, i64); q]
    }

    let ans = {
        let mut ans = 0;
        for arr in (1..=m).combinations_with_replacement(n) {
            let mut point = 0;

            for (a, b, c, d) in &abcd {
                if arr[b - 1] - arr[a - 1] == *c {
                    point += d;
                }
            }

            ans = ans.max(point);
        }
        ans
    };

    println!("{}", ans);
}
