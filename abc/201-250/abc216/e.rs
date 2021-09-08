use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: u64,
        mut a: [u64; n]
    }

    a.push(0);
    a.sort();
    a.reverse();

    let b = a
        .iter()
        .zip(a.iter().skip(1))
        .map(|(x, y)| x - y)
        .collect_vec();

    let mut ans = 0;
    for i in 0..b.len() {
        let (j, count, rem) = {
            let c = k.min(b[i] * (i + 1) as u64);
            let j = (i + 1) as u64;
            (j, c / j * j, c % j)
        };

        if count > 0 {
            ans += count * (2 * a[i] - count / j + 1) / 2;
        }

        if rem > 0 {
            ans += rem * (a[i] - count / j);
        }

        k -= count + rem;
    }

    println!("{}", ans);
}
