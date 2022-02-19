use proconio::input;

fn main() {
    input! {
        x: i128
    }
    if x >= 0 || x.abs() % 10 == 0 {
        println!("{}", x / 10);
    } else {
        println!("{}", x / 10 - 1);
    }
}
