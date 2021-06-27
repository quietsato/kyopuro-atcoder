use proconio::{input, marker::Chars};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        a: u64,
        b: Chars
    }

    let b = b
        .iter()
        .filter(|c| **c != '.')
        .collect::<String>()
        .parse::<u64>()?;
    let c = a * b / 100;

    println!("{}", c);

    Ok(())
}
