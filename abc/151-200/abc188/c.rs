use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: u32 = parse_line().unwrap();
    let a: Vec<u64> = parse_line().unwrap();

    let (l, r) = a.split_at(2usize.pow(n - 1));

    let ml = l.iter().enumerate().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    let mr = r.iter().enumerate().max_by(|a, b| a.1.cmp(b.1)).unwrap();

    println!(
        "{}",
        if ml.1 < mr.1 {
            ml.0 + 1
        } else {
            mr.0 + 2usize.pow(n - 1) + 1
        }
    )
}
