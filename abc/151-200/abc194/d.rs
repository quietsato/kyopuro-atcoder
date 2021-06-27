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

    let n: i32 = parse_line().unwrap();

    let ans: f64 = {
        let n_ = f64::from(n);
        (1..n).map(|s| n_ / (n_ - f64::from(s))).sum()
    };

    println!("{}", ans);
}
