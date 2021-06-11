use proconio::input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        n: usize,
        t: [u64; n]
    }
    let t_sum = t.iter().sum::<u64>();

    let mut dp = vec![vec![false; t_sum as usize + 1]; n + 1];
    dp[0][0] = true;

    for i in 0..n {
        for j in 0..=t_sum as usize {
            dp[i + 1][j] |= dp[i][j];
            if j >= t[i] as usize {
                dp[i + 1][j] |= dp[i][j - t[i] as usize];
            }
        }
    }

    for i in ((t_sum + 1) / 2) as usize..=t_sum as usize {
        if dp[n][i] {
            println!("{}", i);
            return Ok(());
        }
    }

    Ok(())
}
