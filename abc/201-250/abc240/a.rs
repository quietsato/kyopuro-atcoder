use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64
    }

    if (a == 1 && b == 10) || (a == b - 1) {
        println!("Yes");
    } else {
        println!("No");
    }
}
