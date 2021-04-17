#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools,
    whiteread::parse_line
};

fn main() {
    let n: i64 = parse_line().unwrap();

    let mut ans = 0u64;
    for i in 1..=((2 * n) as f64).sqrt() as i64 {
        if 2 * n % i != 0 {
            continue;
        }

        let (a, b) = (i, 2 * n / i);
        if a & 1 == b & 1 {
            continue;
        }

        ans += 2;
    }

    println!("{}", ans);
}
