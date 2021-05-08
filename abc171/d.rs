use whiteread::parse_line;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _n: usize = parse_line()?;
    let a: Vec<i64> = parse_line()?;

    let q: usize = parse_line()?;

    let mut map = vec![0i64; 100_000];
    for x in &a {
        map[*x as usize - 1] += 1;
    }

    let mut ans: i64 = a.iter().sum();

    for _ in 0..q {
        let (b, c): (i64, i64) = parse_line()?;

        ans += (c - b) * map[(b - 1) as usize];
        map[(c - 1) as usize] += map[(b - 1) as usize];
        map[(b - 1) as usize] = 0;

        println!("{}", ans);
    }

    Ok(())
}
