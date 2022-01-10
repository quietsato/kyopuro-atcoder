use proconio::input;

fn main() {
    input! {
        x: u64
    }

    let mut candidates = vec![];
    for a in 1..=9 {
        'outer: for d in -9..=8 {
            for l in 1..=18 {
                let mut v = vec![];
                for i in 0..l {
                    let digit = a + i * d;
                    if !(0..=9).contains(&digit) {
                        continue 'outer;
                    }
                    v.push(digit.to_string());
                }
                candidates.push(v.join("").parse::<u64>().unwrap());
            }
        }
    }

    println!(
        "{}",
        candidates.into_iter().filter(|&c| c >= x).min().unwrap()
    );
}
