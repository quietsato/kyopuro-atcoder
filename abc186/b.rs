use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (h, w): (usize, usize) = parse_line().unwrap();
    let a: Vec<_> = (0..h)
        .map(|_| parse_line::<Vec<usize>>().unwrap())
        .collect();

    let ans: usize = {
        let m = a.iter().flatten().min().unwrap();
        a.iter().flatten().map(|x| x - m).sum()
    };

    println!("{}", ans);
}
