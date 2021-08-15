use proconio::input;

fn main() {
    input! {
        n: u64
    }
    match n {
        _ if n <= 125 => println!("4"),
        _ if n <= 211 => println!("6"),
        _ => println!("8"),
    }
}
