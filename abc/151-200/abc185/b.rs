use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (mut n, m, t): (i64, usize, i64) = parse_line().unwrap();
    let ab: Vec<_> = (0..m)
        .map(|_| parse_line::<(i64, i64)>().unwrap())
        .collect();

    let ans: bool = (|| {
        let n_org = n;
        let mut ct = 0;
        for (a, b) in ab.iter() {
            n -= a - ct;
            if n <= 0 {
                return false;
            }

            // charge
            n += b - a;
            n = n.min(n_org);

            ct = *b;
        }

        n - (t - ct) > 0
    })();

    println!("{}", if ans { "Yes" } else { "No" });
}
