use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
        x: f64,
    }
    if x <= a {
        println!("{}", 1.0);
    } else if x <= b {
        println!("{}", (c / (b - a)).min(1.0));
    } else {
        println!("0");
    }
}

