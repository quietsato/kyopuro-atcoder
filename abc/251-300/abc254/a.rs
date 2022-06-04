use proconio::input;

fn main() {
    input! {
        n: u64
    }
    println!("{:02}", n % 100);
}
