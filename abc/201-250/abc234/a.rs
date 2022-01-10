use proconio::input;

fn main() {
    input! {
        t: i64
    }
    println!("{}", f(f(f(t) + t) + f(f(t))))
}

fn f(x: i64) -> i64 {
    x * x + 2 * x + 3
}
