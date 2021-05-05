use itertools::Itertools;
use whiteread::parse_line;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s: char = parse_line()?;
    let t: char = parse_line()?;

    if s == 'Y' {
        println!("{}", t.to_ascii_uppercase());
    } else {
        println!("{}", t);
    }

    Ok(())
}

