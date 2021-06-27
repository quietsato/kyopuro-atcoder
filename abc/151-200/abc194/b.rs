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

    let n: usize = parse_line().unwrap();

    let abs = {
        let mut abs = vec![];

        for _ in 0..n {
            let ab: (i64, i64) = parse_line().unwrap();
            abs.push(ab);
        }

        abs
    };

    let ans = {
        let mut ans = std::i64::MAX;

        for ai in 0..n {
            for bi in 0..n {
                if ai == bi {
                    ans = ans.min(abs[ai].0 + abs[ai].1);
                } else {
                    ans = ans.min(abs[ai].0.max(abs[bi].1));
                }
            }
        }

        ans
    };

    println!("{}", ans);
}
