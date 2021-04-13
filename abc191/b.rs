use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (_, x): (u64, u64) = parse_line().unwrap();
    let a: Vec<u64> = parse_line().unwrap();

    println!(
        "{}",
        a.into_iter()
            .filter_map(|n| if n == x { None } else { Some(n.to_string()) })
            .join(" ")
    );
}
