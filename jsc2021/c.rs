use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (a, b): (u32, u32) = parse_line().unwrap();

    let (fa, fb) = (a as f32, b as f32);
    let mut ans = 1;
    for c in 1..200_000 {
        if (fa / c as f32).ceil() < (fb / c as f32).floor() {
            ans = c;
        }
    }

    println!("{}", ans);
}
