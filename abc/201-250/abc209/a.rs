use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }

    println!("{}", (b - a + 1).max(0));
}
