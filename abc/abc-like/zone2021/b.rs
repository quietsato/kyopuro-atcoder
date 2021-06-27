use itertools::Itertools;
use whiteread::parse_line;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (n, d, h): (usize, f64, f64) = parse_line()?;
    let mut dh = (0..n)
        .map(|_| parse_line::<(i64, i64)>().unwrap())
        .collect_vec();
    dh.sort_by(|a, b| a.1.cmp(&b.1));
    let dh = dh.iter().map(|(d, h)| (*d as f64, *h as f64)).collect_vec();

    let a = dh
        .iter()
        .map(|(di, hi)| (h - hi) / (d - di))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    println!("{}", (h - a * d).max(0.0));
    Ok(())
}
