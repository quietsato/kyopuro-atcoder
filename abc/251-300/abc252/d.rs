use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut count = vec![0; 2 * 10usize.pow(5) + 1];
    for a in a {
        count[a] += 1;
    }
    let _an = count.iter().filter(|&&c| c > 0).count() as u64;

    let ans = {
        let n = n as u64;
        let mut ans = n * (n - 1) * (n - 2) / 6;
        for c in count {
            if c >= 2 {
                ans -= (n - c) * (c * (c - 1) / 2);
            }
            if c >= 3 {
                ans -= c * (c - 1) * (c - 2) / 6;
            }
        }
        ans
    };

    println!("{}", ans);
}
