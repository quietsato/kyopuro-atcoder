use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (v, t, s, d): (u64, u64, u64, u64) = parse_line().unwrap();

    let (min, max) = (v * t, v * s);

    println!("{}", if min <= d && d <= max { "No" } else { "Yes" });
}
