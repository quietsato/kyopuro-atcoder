use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    let ab: Vec<(i64, i64)> = (0..n)
        .map(|_| parse_line::<(i64, i64)>().unwrap())
        .collect();

    let xs: Vec<_> = ab.iter().map(|(a, b)| 2 * *a + *b).sorted().rev().collect();

    let mut x: i64 = ab.iter().map(|t| t.0).sum();
    x *= -1;

    let mut ans = 0u64;
    for i in 0..xs.len() {
        x += xs[i];

        ans += 1;
        if x > 0 {
            break;
        }
    }

    println!("{}", ans);
}
