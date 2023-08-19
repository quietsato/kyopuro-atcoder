use std::collections::BTreeSet;
use std::iter::FromIterator;
use proconio::input;

fn main() {
    const MOD: u64 = 998244353;

    input! {
        n: i64,
        m: usize,
        (a,b,c,d,e,f):(i64,i64,i64,i64,i64,i64),
        xy: [(i64, i64); m]
    };
    let xy = BTreeSet::from_iter(xy.iter());

    let mut dp = vec![vec![vec![0; n as usize + 1]; n as usize + 1]; 2];
    dp[0][0][0] = 1;

    for i in 1..=n {
        for x in 0..=n {
            for y in 0..=n {
                dp[i as usize % 2][x as usize][y as usize] = 0;
            }
        }
        for x in 0..=i {
            for y in 0..=i - x {
                let z = i - x - y;
                if !xy.contains(&(x * a + y * c + z * e, x * b + y * d + z * f)) {
                    dp[i as usize % 2][x as usize][y as usize] += if x > 0 {
                        dp[(i as usize + 1) % 2][(x - 1) as usize][y as usize]
                    } else {
                        0
                    } + if y > 0 {
                        dp[(i as usize + 1) % 2][x as usize][(y - 1) as usize]
                    } else {
                        0
                    } + if z > 0 {
                        dp[(i as usize + 1) % 2][x as usize][y as usize]
                    } else {
                        0
                    };
                    dp[i as usize % 2][x as usize][y as usize] %= MOD;
                }
            }
        }
    }

    let mut ans = 0;
    for x in 0..=n {
        for y in 0..=n - x {
            ans += dp[n as usize % 2][x as usize][y as usize];
            ans %= MOD;
        }
    }
    println!("{}", ans);
}
