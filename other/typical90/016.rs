use proconio::input;

fn main() {
    input! {
        n: u64,
        mut m: [u64; 3]
    }
    m.sort_by_key(|&v| std::cmp::Reverse(v));

    let mut ans = 10000;
    for a in (0..=(n / m[0]).min(9999)).rev() {
        for b in (0..=((n - a * m[0]) / m[1]).min(9999)).rev() {
            if (n - a * m[0] - b * m[1]) % m[2] == 0 {
                ans = ans.min(a + b + (n - a * m[0] - b * m[1]) / m[2]);
            }
        }
    }

    println!("{}", ans);
}
