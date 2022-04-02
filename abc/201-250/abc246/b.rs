use proconio::input;

fn main() {
    input! {
        x: f64,
        y: f64
    }

    let mul = (x.powi(2) + y.powi(2)).sqrt();

    println!("{} {}", x / mul, y / mul);
}
