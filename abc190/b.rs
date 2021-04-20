use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (n, s, d): (usize, u64, u64) = parse_line().unwrap();
    let xy: Vec<_> = (0..n)
        .into_iter()
        .map(|_| parse_line::<(u64, u64)>().unwrap())
        .collect();

    let ans: bool = {
        xy.iter()
            .filter_map(|(x, y)| if x < &s && y > &d { Some(()) } else { None })
            .count()
            > 0
    };

    println!("{}", if ans { "Yes" } else { "No" });
}
