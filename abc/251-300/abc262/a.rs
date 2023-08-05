use proconio::input;

fn main() {
    input! {
        y: u32
    };
    println!("{}", (y - 2) + (4 - ((y - 2) % 4)) % 4 + 2);
}
