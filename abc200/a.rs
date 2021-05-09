use whiteread::parse_line;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n: u64 = parse_line()?;

    println!("{}", (n - 1) / 100 + 1);

    Ok(())
}
