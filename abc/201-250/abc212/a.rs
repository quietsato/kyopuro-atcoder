use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64
    };

    if a > 0 && b == 0 {
        println!("Gold");
    } else if a == 0 && b > 0 {
        println!("Silver");
    } else {
        println!("Alloy");
    }
}
