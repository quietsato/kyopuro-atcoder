use proconio::input;

fn main() {
    input! {
        n: i64,
        p: i64
    }

    let mut dp = vec![vec![0i64; n as usize + 4]; n as usize + 5];

    dp[0][0] = 1;

    for i in 0..n {
        if i > 0 {
            for j in 0..n + 3 {
                dp[i as usize][(j + 1) as usize] += dp[i as usize][j as usize];
                dp[i as usize][(j + 1) as usize] %= p;
            }
        }
        for j in 0..n {
            let base = if i == 0 { 26 } else { 25 };
            let ways = (dp[i as usize][j as usize] * base) % p;
            let (mut l, mut k) = (1, 1);
            while l <= n - j {
                let r = (l * 10).min(n - j + 3);
                let di = k + 1;
                // for dj in l..r {
                //     dp[(i + di) as usize][(j + dj) as usize] += ways;
                //     dp[(i + di) as usize][(j + dj) as usize] %= p;
                // }
                dp[(i + di) as usize][(j + l) as usize] += ways;
                dp[(i + di) as usize][(j + l) as usize] %= p;
                dp[(i + di) as usize][(j + r) as usize] -= ways;
                dp[(i + di) as usize][(j + r) as usize] %= p;
                l *= 10;
                k += 1;
            }
        }
    }

    println!(
        "{}",
        dp.iter()
            .take(n as usize)
            .fold(0, |ans, row| ((ans + row[n as usize]) % p + p) % p)
    );
}
