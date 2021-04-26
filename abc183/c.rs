use itertools::Itertools;
use whiteread::parse_line;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (n, k): (usize, u64) = parse_line()?;
    let t = (0..n)
        .map(|_| parse_line::<Vec<u64>>().unwrap())
        .collect_vec();

    let ans = {
        (1..n)
            .permutations(n - 1)
            .filter_map(|path| {
                let path = {
                    let mut path = path;
                    path.insert(0, 0);
                    path.push(0);
                    path
                };
                if path
                    .iter()
                    .zip(path.iter().skip(1))
                    .map(|(&i, &j)| -> u64 { t[i][j] })
                    .sum::<u64>()
                    == k - t[*path.last()? as usize][0]
                {
                    Some(())
                } else {
                    None
                }
            })
            .count()
    };

    println!("{}", ans);

    Ok(())
}
