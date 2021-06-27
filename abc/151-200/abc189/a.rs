use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let c: Vec<char> = parse_line::<String>().unwrap().chars().collect_vec();
    if c[0] == c[1] && c[1] == c[2] {
        println!("Won")
    } else {
        println!("Lost")
    }
}
