use proconio::input;

fn main() {
    input! {
        mut x: i64,
        mut a: i64,
        mut d: i64,
        n: i64,
    }

    let last = a + (d * (n - 1));
    let (min, max) = (a.min(last), a.max(last));

    if x <= min {
        println!("{}", (min - x).abs());
        return;
    }
    if x >= max {
        println!("{}", (max - x).abs());
        return;
    }

    if d < 0 {
        a = last;
        d = -d;
    }

    let mut to_lower = (x - a) % d.abs();
    let mut to_upper = d.abs() - to_lower;
    if to_lower < 0 {
        to_lower += d.abs();
    }
    if to_upper < 0 {
        to_upper += d.abs();
    }

    println!("{}", to_lower.min(to_upper));
}
