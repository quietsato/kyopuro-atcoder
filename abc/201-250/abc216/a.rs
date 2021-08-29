use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String
    }

    let xy = s.split('.').collect_vec();
    let x = xy[0].parse::<i64>().unwrap();
    let y = xy[1].parse::<i64>().unwrap();

    println!(
        "{}{}",
        x,
        if y <= 2 {
            "-"
        } else if y >= 7 {
            "+"
        } else {
            ""
        }
    )
}
