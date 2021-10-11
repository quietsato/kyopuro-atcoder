use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }

    println!("{}", 32u128.pow(a - b))
}
