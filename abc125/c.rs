use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    let a: Vec<u64> = parse_line().unwrap();

    let ans = {
        let mut memo = vec![(0, 0); n];
        let (mut l, mut r) = (0, 0);
        for i in 0..n {
            if i >= 1 {
                l = my_gcd(l, a[i - 1]);
                memo[i].0 = l;
                r = my_gcd(r, a[n - i]);
                memo[n - 1 - i].1 = r;
            }
        }

        (0..n).map(|i| my_gcd(memo[i].0, memo[i].1)).max().unwrap()
    };

    println!("{}", ans);
}

fn my_gcd(x: u64, y: u64) -> u64 {
    let (a, b) = (x.max(y), x.min(y));

    if b == 0 {
        a
    } else if a % b == 0 {
        b
    } else {
        my_gcd(b, a % b)
    }
}
