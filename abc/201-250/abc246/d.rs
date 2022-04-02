use proconio::input;

fn main() {
    input! {
        n: u64
    }

    let mut ans = std::u64::MAX;
    for a in 0..=1_000_000 {
        let mut l = 0;
        let mut r = 1_000_000;

        while l < r {
            let b = (l + r) / 2;
            let add = (a + b) * (a * a + b * b);
            if add >= n {
                ans = ans.min(add);
                r = b;
            } else {
                l = b + 1;
            }
        }
    }

    println!("{}", ans);
}
