use proconio::input;

fn main() {
    input! {
        a: [i64; 3]
    }
    let mut x = 2 * a[1] - a[0] - a[2];

    let mut ans = 0;
    if x < 0 {
        ans += x.abs() / 2 + (x.abs() & 1);
        x = x.abs() & 1;
    }
    if x > 0 {
        ans += x;
    }

    println!("{}", ans);
}
