use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        _n: u64
    }
    const X: u64 = 100;

    println!("{}", X * 3);
    let mut ans = vec![];
    for i in 0..3 {
        let base = X.pow(i);
        for j in 1..=X {
            ans.push(base * j);
        }
    }
    assert_eq!(ans.len(), (X * 3) as usize);

    println!(
        "{}",
        ans.iter().map(ToString::to_string).collect_vec().join(" ")
    );
}
