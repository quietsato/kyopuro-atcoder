use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    let a: Vec<_> = {
        let mut a: Vec<i64> = parse_line().unwrap();
        a.sort();
        a
    };
    let b: Vec<_> = {
        a.iter()
            .scan(0i64, |state, n| {
                *state += n;
                Some(*state)
            })
            .collect()
    };

    let last = *b.last().unwrap();
    let ans: u64 = (0..n)
        .map(|i| -> u64 { ((last - b[i]) - ((n as i64 - 1 - i as i64) * a[i])).abs() as u64 })
        .sum();

    println!("{}", ans);
}
