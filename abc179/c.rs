#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools::*,
    whiteread::*,
    std::*,
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
        let mut ans = 0;

        for a in 1..n {
            for b in 1.. {
                if a * b < n {
                    ans += 1;
                } else {
                    break;
                }
            }
        }

        ans
    };

    println!("{}", ans);
}

