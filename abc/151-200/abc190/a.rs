use whiteread::parse_line;

fn main() {
    let (a, b, c): (i64, i64, i64) = parse_line().unwrap();

    if (c == 0 && a > b) || (c == 1 && a >= b) {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
