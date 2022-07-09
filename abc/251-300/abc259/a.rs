use proconio::input;

fn main() {
    input! {
        _n: u64,
        m: u64,
        x: u64,
        t: u64,
        d: u64,
    }
    if m >= x {
        println!("{}", t);
        return;
    }
    println!("{}", t - d * (x - m));
}
