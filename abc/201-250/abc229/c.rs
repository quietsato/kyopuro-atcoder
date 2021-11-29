use proconio::input;

fn main() {
    input! {
        n: usize,
        mut w: i64,
        mut ab: [(i64, i64); n]
    }

    ab.sort();
    ab.reverse();

    let mut ans = 0;
    for &(a, b) in &ab {
        ans += a * w.min(b);
        w -= b;
        if w <= 0 {
            break;
        }
    }

    println!("{}", ans);
}
