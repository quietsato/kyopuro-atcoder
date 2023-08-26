use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    };
    let cumsum = [0]
        .iter()
        .chain(a.iter())
        .scan(0, |s, a| {
            *s += a;
            Some(*s)
        })
        .collect_vec();
    dbg!(&cumsum);

    let mut current: i64 = { (0..m).map(|i| a[i] * (i + 1) as i64).sum() };
    let mut ans = current;

    for i in 1..=n - m {
        // update
        current -= cumsum[i + m - 1] - cumsum[i - 1];
        current += a[i + m - 1] * m as i64;
        ans = ans.max(current);
    }

    println!("{}", ans);
}
