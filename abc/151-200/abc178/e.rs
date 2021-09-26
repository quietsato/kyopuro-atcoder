use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n]
    }

    let mut sum = Vec::with_capacity(n);
    let mut sub = Vec::with_capacity(n);
    for (x, y) in xy {
        sum.push(x + y);
        sub.push(x - y);
    }
    sum.sort();
    sub.sort();

    let ans = (sum[n - 1] - sum[0]).max(sub[n - 1] - sub[0]);
    println!("{}", ans);
}
