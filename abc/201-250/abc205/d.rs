use proconio::input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        n: usize,
        q: usize,
        mut a: [u64; n],
        k: [u64; q]
    }

    for k in k {
        if k < a[0] {
            println!("{}", k);
            continue;
        }

        let (mut l, mut r) = (0usize, n);
        while r - l > 1 {
            let m = (l + r) / 2;
            if a[m] < (k + m as u64 + 1) {
                l = m;
            } else {
                r = m;
            }
        }

        println!("{}", k + l as u64 + 1);
    }

    Ok(())
}

// 参考
// https://atcoder.jp/contests/abc205/submissions/23432015
