use whiteread::parse_line;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..parse_line::<usize>()? {
        println!("{}", {
            let n = parse_line::<u64>()?;
            if n & 1 == 1 {
                "Odd"
            } else if n % 4 == 0 {
                "Even"
            } else {
                "Same"
            }
        });
    }

    Ok(())
}
