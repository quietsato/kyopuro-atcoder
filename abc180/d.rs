#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools,
    whiteread::parse_line
};

fn main() {
    let (x, y, a, b): (u64, u64, u64, u64) = parse_line().unwrap();

    let ans = {
        let mut ans = 0;

        let n = f64::log(y as f64, a as f64) as u32;
        for i in 0..n {
            let mut expr = 0u64;
            let mut s = x;

            if y / a.pow(i) < s {
                continue;
            }

            expr += i as u64;
            s *= a.pow(i);

            expr += (y - s) / b;
            if (y - s) % b == 0 {
                expr -= 1;
            }

            ans = ans.max(expr);
        }
        ans
    };

    println!("{}", ans);
}
