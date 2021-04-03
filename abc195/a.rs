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

    let (c, b): (i64, i64) = parse_line().unwrap();

    println!("{}", if b % c == 0 { "Yes" } else { "No" });
}

