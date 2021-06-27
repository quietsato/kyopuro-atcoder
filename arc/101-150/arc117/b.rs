use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    let a: Vec<u64> = {
        let mut x: Vec<u64> = parse_line().unwrap();
        x.sort();
        x.insert(0, 0);

        x.iter().zip(x.iter().skip(1)).map(|(x, y)| y - x + 1).collect()
    };

    let ans = {
        let MOD = 1_000_000_007u64;

        let mut ans = 1u64;

        for v in a {
            ans *= v;
            ans %= MOD;
        }

        ans
    };

    println!("{}", ans);
}
