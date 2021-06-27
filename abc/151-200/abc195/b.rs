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

    let (a, b, w): (i64, i64, i64) = parse_line().unwrap();

    let mut min = std::i64::MAX;
    let mut max = std::i64::MIN;

    for n in 0.. {
        if a * n <= 1000 * w && 1000 * w <= b * n {
            min = n.min(min);
            max = n.max(max);
        }
        if a * n > 1000 * w {
            break;
        }
    }

    if min == std::i64::MAX && max == std::i64::MIN {
        println!("UNSATISFIABLE");
    } else {
        println!("{} {}", min, max);
    }
}

