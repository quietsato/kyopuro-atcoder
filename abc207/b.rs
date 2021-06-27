use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
        d: f64,
    }
    println!("{}", ((a / (d * c - b)).ceil() as i64).max(-1))
}
