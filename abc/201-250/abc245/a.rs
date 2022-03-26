use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64,
    }
    if a * 10000 + b * 100 < c * 10000 + d * 100 + 1 {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
