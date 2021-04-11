use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let s: String = parse_line::<String>()
        .unwrap()
        .trim_end_matches('0')
        .to_string();
    let rs = s.clone().chars().rev().collect::<String>();

    if s == rs {
        println!("Yes");
    } else {
        println!("No");
    }
}

