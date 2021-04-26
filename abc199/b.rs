use itertools::Itertools;
use whiteread::parse_line;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n: usize = parse_line()?;
    let a: Vec<i64> = parse_line()?;
    let b: Vec<i64> = parse_line()?;

    let min = a.iter().max().unwrap();
    let max = b.iter().min().unwrap();

    println!("{}", (max - min + 1).max(0));

    Ok(())
}

