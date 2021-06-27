use itertools::Itertools;
use whiteread::parse_line;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n: usize = parse_line()?;
    let MOD = 1_000_000_007i64;

    let m0 = {
        let mut m = 1;
        for _ in (0..n) {
            m *= 10;
            m %= MOD;
        }
        m
    };
    let m1 = {
        let mut m = 2;
        for _ in (0..n) {
            m *= 9;
            m %= MOD;
        }
        m
    };
    let m2 = {
        let mut m = 1;
        for _ in (0..n) {
            m *= 8;
            m %= MOD;
        }
        m
    };

    println!("{}", ((m0 - m1 + m2) + 2 * MOD) % MOD);

    Ok(())
}
