use proconio::input;

fn main() {
    input! {
        n: u64,
        x: u64
    }

    let x = x - 1;
    let i = x / n;

    println!("{}", (b'A' + (i as u8)) as char);
}
