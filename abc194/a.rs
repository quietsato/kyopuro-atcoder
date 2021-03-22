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

    let (a, b): (i64, i64) = parse_line().unwrap();

    let (x, y) = (a + b, b);

    if x >= 15 && y >= 8 {
        println!("1");
    } else if x >= 10 && y >= 3 {
        println!("2");
    } else if x >= 3 {
        println!("3");
    } else {
        println!("4");
    }
}
