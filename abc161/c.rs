#[allow(unused_imports)]
use {itertools, proconio::input, std::collections::*};

#[allow(dead_code)]
mod util {}

#[cfg(test)]
mod test {}

#[allow(unused_imports)]
fn main() {
    use util::*;

    input! {
        (n, k): (i64, i64),
    };

    let m: i64 = n % k;
    println!("{}", std::cmp::min(m, (m - k).abs()));
}

