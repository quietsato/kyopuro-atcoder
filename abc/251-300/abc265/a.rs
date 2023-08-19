use proconio::input;

fn main() {
    input! {
        x: u32,
        y: u32,
        n: u32,
    };
    println!(
        "{}",
        if 3 * x < y {
            x * n
        } else {
            (n / 3) * y + (n % 3) * x
        }
    );
}
