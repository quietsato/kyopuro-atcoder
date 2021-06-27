use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    let d: Vec<_> = {
        (0..n)
            .map(|_| {
                let d: (u8, u8) = parse_line().unwrap();
                d
            })
            .collect_vec()
    };

    let ans = {
        let d = d
            .iter()
            .enumerate()
            .filter_map(|(i, (d1, d2))| if d1 == d2 { Some(i) } else { None })
            .collect_vec();
        (2..d.len())
            .filter(|&i| d[i] == d[i - 1] + 1 && d[i - 1] == d[i - 2] + 1)
            .next()
            .is_some()
    };

    println!("{}", if ans { "Yes" } else { "No" });
}
