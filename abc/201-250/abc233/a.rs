use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32
    }

    if x >= y {
        println!("0");
    } else {
        println!("{}", (y - x) / 10 + (if (y - x) % 10 > 0 { 1 } else { 0 }));
    }
}
