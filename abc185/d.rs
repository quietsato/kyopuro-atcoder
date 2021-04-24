use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (n, m): (usize, usize) = parse_line().unwrap();
    let a = {
        let mut x: Vec<usize> = parse_line().unwrap();
        x.sort();
        x.insert(0, 0);
        x.push(n + 1);

        let mut a: Vec<usize> = vec![];
        for i in 1..=m + 1 {
            let diff = x[i] - x[i - 1] - 1;
            if diff > 0 {
                a.push(diff);
            }
        }

        a
    };

    let d = a.iter().min().unwrap_or(&0);
    println!(
        "{}",
        a.iter().map(|n| ((n + d - 1) / d) as u64).sum::<u64>()
    );
}
