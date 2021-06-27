use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    if parse_line::<(i32, i32)>()
        .map(|(a, b)| (a - b).abs() < 3)
        .unwrap()
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
