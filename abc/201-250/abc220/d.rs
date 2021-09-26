use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    const MOD: u128 = 998244353;

    let mut memo = vec![vec![0u128; 10]; n];
    memo[0][(a[0] + a[1]) % 10] += 1;
    memo[0][(a[0] * a[1]) % 10] += 1;

    for i in 1..n - 1 {
        for j in 0..10 {
            memo[i][(j + a[i + 1]) % 10] += memo[i - 1][j];
            memo[i][(j + a[i + 1]) % 10] %= MOD;
            memo[i][(j * a[i + 1]) % 10] += memo[i - 1][j];
            memo[i][(j * a[i + 1]) % 10] %= MOD;
        }
    }

    for a in &memo[n - 2] {
        println!("{}", a);
    }
}
