use proconio::input;

// 055 Select 5 (★2)
fn main() {
    input! {
        n: usize,
        p: u64,
        q: u64,
        a: [u64; n]
    }

    let mut ans = 0u64;
    for s in 0..n {
        for t in s + 1..n {
            for u in t + 1..n {
                for v in u + 1..n {
                    for w in v + 1..n {
                        if [a[s], a[t], a[u], a[v], a[w]]
                            .iter()
                            .fold(1, |s, &x| (s * x) % p)
                            % p
                            == q
                        {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
