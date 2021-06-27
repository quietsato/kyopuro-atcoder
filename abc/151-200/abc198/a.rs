use whiteread::parse_line;

fn main() {
    let n: i64 = parse_line().unwrap();

    println!("{}", n - 1);
}
