use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    let a: Vec<i32> = parse_line().unwrap();
    let b: Vec<i32> = parse_line().unwrap();

    println!(
        "{}",
        if a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<i32>() == 0 {
            "Yes"
        } else {
            "No"
        }
    )
}
