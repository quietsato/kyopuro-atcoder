use whiteread::parse_line;

fn main() {
    let (r, x, y): (f64, f64, f64) = parse_line().unwrap();

    if r > (x.powi(2) + y.powi(2)).sqrt() {
        println!("2");
    } else {
        println!("{}", ((x.powi(2) + y.powi(2)).sqrt() / r).ceil() as i64);
    }
}
