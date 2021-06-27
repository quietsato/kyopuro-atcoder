use whiteread::parse_line;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n: usize = parse_line()?;

    let mut ans = std::collections::BTreeSet::new();
    for x in (1..=(n as f64).sqrt().ceil() as usize).filter(|x| n % x == 0) {
        ans.insert(x);
        ans.insert(n / x);
    }

    ans.iter().for_each(|i| println!("{}", i));

    Ok(())
}
