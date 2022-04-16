use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        k: u64
    }
    let mut x = a;
    for i in 0.. {
        if x >= b {
            println!("{}", i);
            return;
        }
        x *= k;
    }
}
