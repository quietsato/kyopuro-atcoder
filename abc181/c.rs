use itertools::Itertools;
use whiteread::parse_line;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n: usize = parse_line()?;
    let mut p = (0..n)
        .map(|_| parse_line::<(i64, i64)>().unwrap())
        .collect_vec();
    p.sort_by(|p1, p2| p1.0.cmp(&p2.0));

    let ans = {
        (0..n)
            .combinations(3)
            .map(|comb| -> bool {
                if p[comb[0]].0 == p[comb[1]].0 && p[comb[1]].0 == p[comb[2]].0 {
                    true
                } else if (p[comb[1]].1 - p[comb[0]].1) as f64
                    / (p[comb[1]].0 - p[comb[0]].0) as f64
                    == (p[comb[2]].1 - p[comb[1]].1) as f64 / (p[comb[2]].0 - p[comb[1]].0) as f64
                {
                    true
                } else {
                    false
                }
            })
            .filter(|x| *x)
            .next()
            .is_some()
    };

    println!("{}", if ans { "Yes" } else { "No" });

    Ok(())
}
