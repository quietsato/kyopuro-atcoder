use itertools::Itertools;
use whiteread::parse_line;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (a, b, c): (i64, i64, i64) = parse_line()?;

    if a.pow(2) + b.pow(2) < c.pow(2) {
        println!("Yes")
    } else {
        println!("No")
    }

    Ok(())
}

