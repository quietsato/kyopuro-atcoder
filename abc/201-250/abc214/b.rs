use proconio::input;

fn main() {
    input! {
        s: u64,
        t: u64
    }

    let mut ans = 0;
    for a in 0..=s {
        for b in 0..=(s - a) {
            for c in 0..=(s - a - b) {
                if a * b * c <= t {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
