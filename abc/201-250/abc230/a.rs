use proconio::input;

fn main() {
    input! {
        n: u64
    }

    if n >= 42 {
        println!("AGC{:03}", n + 1);
    } else {
        println!("AGC{:03}", n);
    }
}

