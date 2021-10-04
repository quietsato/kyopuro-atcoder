use proconio::input;

const MOD: i64 = 998244353;

fn main() {
    input! {
        n: usize,
        k: usize,
        lr: [(i64, i64); k]
    }

    let mut dp = vec![0i64; n];
    let mut sdp = vec![0i64; n + 1];
    dp[0] = 1;
    sdp[1] = 1;

    for i in 1..n as i64 {
        for &(l, r) in &lr {
            let (sr, sl) = ((i - l + 1).max(0) as usize, (i - r).max(0) as usize);
            dp[i as usize] += (sdp[sr] - sdp[sl]) % MOD;
        }

        sdp[(i + 1) as usize] = sdp[i as usize] + dp[i as usize];
        sdp[(i + 1) as usize] %= MOD;
    }

    println!("{}", ((dp[n - 1] % MOD) + MOD) % MOD);
}
