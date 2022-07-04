use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }

    let ans = {
        const INF: u64 = std::u64::MAX >> 1;

        // 行動 1を行わない場合
        let v1 = {
            let mut dp = vec![vec![0; 2]; n];
            dp[0][0] = 0;
            dp[0][1] = INF;

            for i in 1..n {
                dp[i][0] = dp[i - 1][1];
                dp[i][1] = (dp[i - 1][0]).min(dp[i - 1][1]) + a[i];
            }

            // 行動 N は必ず行う
            dp[n - 1][1]
        };

        // 行動1を行う場合
        let v2 = {
            let mut dp = vec![vec![0; 2]; n];
            dp[0][0] = INF;
            dp[0][1] = a[0];

            for i in 1..n {
                dp[i][0] = dp[i - 1][1];
                dp[i][1] = (dp[i - 1][0]).min(dp[i - 1][1]) + a[i];
            }

            // 行動 N は行っても行わなくてもよい
            dp[n - 1][0].min(dp[n - 1][1])
        };

        v1.min(v2)
    };

    println!("{}", ans);
}
