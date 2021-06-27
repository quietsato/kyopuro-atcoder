use itertools::Itertools;
use whiteread::parse_line;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut n, k): (String, usize) = parse_line()?;

    for _ in 0..k {
        if n.parse::<u64>()? % 200 == 0 {
            n = (n.parse::<u64>()? / 200).to_string();
        } else {
            n = format!("{}200", n);
        }
    }

    println!("{}", n);

    Ok(())
}
