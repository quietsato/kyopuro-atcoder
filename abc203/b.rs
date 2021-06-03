use itertools::Itertools;
use proconio::input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        (n, k): (usize, usize),
    }

    let mut ans = 0;
    for i in 1..=n {
        for j in 1..=k {
            ans += 100 * i + j;
        }
    }

    println!("{}", ans);

    Ok(())
}
