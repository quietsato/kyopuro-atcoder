use whiteread::parse_line;

fn main() {
    let (n, w): (i64, i64) = parse_line().unwrap();

    println!("{}", n / w);
}
