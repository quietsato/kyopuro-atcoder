use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        x: u64
    }

    let s = a.iter().sum::<u64>();

    let (p, mut q) = (x / s, x % s);

    let mut ans = n as u64 * p;
    for i in 0.. {
        ans += 1;
        if q < a[i] {
            break;
        }
        q -= a[i];
    }

    println!("{}", ans);
}
