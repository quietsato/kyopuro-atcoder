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

    let s: String = {
        let s: String = parse_line().unwrap();

        s.split_terminator('.').collect::<Vec<&str>>()[0].to_owned()
    };

    println!("{}", s);
}

