#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools,
    whiteread::parse_line
};

fn main() {
    let n: usize = parse_line().unwrap();
    let a: Vec<u64> = parse_line().unwrap();

    let ans = {
        let mut ans = 0;

        for l in 0..n {
            let mut min = a[l];
            for r in l..n {
                min = min.min(a[r]);
                ans = ans.max(min * (r - l + 1) as u64);
            }
        }

        ans
    };

    println!("{}", ans);
}
