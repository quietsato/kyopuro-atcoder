use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        x: i64,
        y: i64,
    }

    println!("{}", a.min(n) * x + (n - a).max(0) * y);
}
