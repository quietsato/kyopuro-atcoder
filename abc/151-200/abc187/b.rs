use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    let xy: Vec<_> = (0..n)
        .map(|_| parse_line::<(i32, i32)>().unwrap())
        .collect();

    let ans = {
        xy.iter()
            .combinations(2)
            .filter_map(|cmb| {
                let (p0, p1) = (cmb[0], cmb[1]);
                if ((p1.1 - p0.1).abs() - (p1.0 - p0.0).abs()) <= 0 {
                    Some(())
                } else {
                    None
                }
            })
            .count()
    };

    println!("{}", ans);
}
