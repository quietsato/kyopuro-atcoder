use proconio::input;

fn main() {
    input! {
        n: usize,
        p: i64,
        a: [i64; n]
    }

    println!("{}", a.iter().filter(|a| a < &&p).count());
}
