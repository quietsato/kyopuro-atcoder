use proconio::input;

fn main() {
    input! {
        n: usize,
        q: [u64; n],
        a: [u64; n],
        b: [u64; n],
    };

    let mut ans = 0;

    for x in 0..=1_000_000 {
        // 料理Aをx個作れるか
        if !q.iter().zip(a.iter()).all(|(&q, &a)| a * x <= q) {
            break;
        }

        // 料理Bはいくつ作れるか
        let y = q
            .iter()
            .zip(a.iter())
            .zip(b.iter())
            .filter(|(_, &b)| b != 0)
            .map(|((&q, &a), &b)| (q - a * x) / b)
            .min()
            .unwrap();

        ans = ans.max(x + y);
    }

    println!("{}", ans);
}
