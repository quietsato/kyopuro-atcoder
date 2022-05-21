use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u64; n],
        b: [Usize1; k]
    }
    let max = a.iter().max().unwrap();
    let available = b.iter().any(|i| a[*i] == *max);

    println!("{}", if available { "Yes" } else { "No" });
}
