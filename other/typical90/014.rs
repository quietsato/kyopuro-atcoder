use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        mut b: [i64; n],
    }

    a.sort();
    b.sort();

    let ans = (0..n).map(|i| (a[i] - b[i]).abs()).sum::<i64>();

    println!("{}", ans);
}
