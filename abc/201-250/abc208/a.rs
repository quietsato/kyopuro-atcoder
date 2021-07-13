use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }

    if a <= b && b <= 6 * a {
        println!("Yes");
    } else {
        println!("No");
    }
}
