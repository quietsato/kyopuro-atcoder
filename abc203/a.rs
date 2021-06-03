use itertools::Itertools;
use proconio::input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        a: i64,
        (b, c): (i64, i64),
    }

    if a == b {
        println!("{}", c);
    } else if b == c {
        println!("{}", a);
    } else if a == c {
        println!("{}", b);
    } else {
        println!("0");
    }

    Ok(())
}
