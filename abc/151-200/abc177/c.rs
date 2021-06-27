use itertools::Itertools;
use whiteread::parse_line;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n: usize = parse_line::<usize>()?;
    let a: Vec<u128> = parse_line::<Vec<u64>>()?
        .iter()
        .map(|n| -> u128 { *n as u128 })
        .collect_vec();

    let ans = {
        let mut ans = 0;
        let mut a_sum: u128 = a.iter().sum();

        for i in 0..n {
            a_sum -= a[i];
            ans += a[i] * a_sum;
            ans %= 1_000_000_007u128;
        }

        ans
    };

    println!("{}", ans);

    Ok(())
}
