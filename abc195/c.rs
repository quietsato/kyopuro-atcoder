#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools::*,
    whiteread::*,
    std::collections::*
};

#[allow(dead_code)]
mod util {}

#[cfg(test)]
mod test {}

#[allow(unused_imports)]
fn main() {
    use util::*;

    let n: u64 = parse_line().unwrap();

    let ans = {
        let mut ans = 0u64;

        for p in (3u32..).step_by(3) {
            dbg!(10u64.pow(p));
            if n < 10u64.pow(p) {
                ans += (p as u64 / 3 - 1) * (n - (10u64.pow(p - 3) - 1));
                dbg!(ans);
                break;
            }
            ans += (p as u64 / 3 - 1) * (10u64.pow(p) - 10u64.pow(p - 3));
            dbg!(ans);
        }
        ans
    };

    println!("{}", ans);
}

