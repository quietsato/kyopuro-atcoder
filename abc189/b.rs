use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (n, x): (usize, u64) = parse_line().unwrap();
    let vp: Vec<_> = (0..n)
        .into_iter()
        .map(|_| parse_line::<(u64, u64)>().unwrap())
        .collect();

    let ans: Option<i64> = {
        vp.iter()
            .scan(0u64, |state, (v, p)| {
                *state += v * p;
                Some(*state)
            })
            .find_position(|a| a > &(x * 100))
            .map(|(i, _)| (i + 1) as i64)
    };

    println!("{}", ans.unwrap_or(-1));
}
