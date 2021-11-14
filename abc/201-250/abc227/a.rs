use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: usize
    }

    println!("{}", ((k + a - 2) % n) + 1);
}
