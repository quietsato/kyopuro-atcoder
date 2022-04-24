use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n]
    }

    let mut cnt = vec![0u64; 2 * 100_000 + 1];
    for &a in &a {
        cnt[a as usize] += 1;
    }

    let mut ans = 0u64;
    for x in a.iter() {
        for d in 1..=((*x as f64).sqrt() as u64) {
            if x % d == 0 {
                ans += cnt[d as usize] * cnt[(x / d) as usize] * if d == (x / d) { 1 } else { 2 };
            }
        }
    }

    println!("{}", ans);
}
