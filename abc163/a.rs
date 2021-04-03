#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools,
    whiteread::parse_line
};

fn main() {
    println!(
        "{}",
        2f64 * parse_line::<f64>().unwrap() * std::f64::consts::PI
    );
}
