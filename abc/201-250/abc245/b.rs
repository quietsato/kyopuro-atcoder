use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n]
    }
    for i in 0.. {
        if !a.contains(&i) {
            println!("{}", i);
            return;
        }
    }
}
