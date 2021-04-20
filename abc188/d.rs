use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (n, c): (usize, i64) = parse_line().unwrap();

    let mut event = vec![];
    for _ in 0..n {
        let (a, b, c): (i64, i64, i64) = parse_line().unwrap();

        event.push((a - 1, c));
        event.push((b, -c));
    }

    event.sort_by(|x, y| x.0.cmp(&y.0));

    let ans = {
        let mut ans = 0;
        let mut fee = 0;
        let mut day = 0;

        for (d, f) in event {
            ans += fee.min(c) * (d - day);
            fee += f;
            day = d;
        }

        ans
    };

    println!("{}", ans);
}
