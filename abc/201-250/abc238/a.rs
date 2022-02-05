use proconio::input;

fn main() {
    input! {
        n: u64
    }

    YesNo(n == 1 || n >= 5);
}
#[allow(non_snake_case)]
fn yesno(b: bool) {
    println!("{}", if b { "yes" } else { "no" });
}
#[allow(non_snake_case)]
fn YesNo(b: bool) {
    println!("{}", if b { "Yes" } else { "No" });
}
#[allow(non_snake_case)]
fn YESNO(b: bool) {
    println!("{}", if b { "YES" } else { "NO" });
}