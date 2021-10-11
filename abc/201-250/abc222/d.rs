use proconio::input;

const MOD: i64 = 998244353;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }

    let mut v = vec![vec![0; n]; 3001];
    let mut sum = vec![0; 3002];

    for i in a[0]..=b[0] {
        v[i as usize][0] = 1;
        sum[i as usize + 1] = sum[i as usize] + 1;
    }

    for i in 1..n {
        for j in a[i] as usize..=b[i] as usize {
            v[j][i] = sum[j.min(b[i - 1] as usize) + 1] - sum[a[i - 1] as usize];
            v[j][i] %= MOD;
        }
        for j in 0..=3000 {
            sum[j + 1] = sum[j] + v[j][i];
            sum[j + 1] %= MOD;
        }
    }

    println!(
        "{}",
        sum[b[n - 1] as usize + 1]
    );
}
