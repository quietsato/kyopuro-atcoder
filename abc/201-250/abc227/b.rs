use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [i64; n]
    }

    let mut ans = 0;

    'outer: for si in s {
        for a in 1..=334 {
            for b in 1..=334 {
                let s = 4 * a * b + 3 * a + 3 * b;
                if si == s {
                    continue'outer;
                }
            }
        }
        ans += 1;
    }

    println!("{}", ans);
}
