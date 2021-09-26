use proconio::input;

fn main() {
    input! {
       a: u64,
       b: u64,
       c: u64,
    }

    for i in a..=b {
        if i % c == 0 {
            println!("{}", i);
            return;
        }
    }

    println!("-1");
}
