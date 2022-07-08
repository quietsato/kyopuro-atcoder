use proconio::input;

#[rustfmt::skip]
fn main() {
    const MOD: u64 = 998244353;

    input! {
        h: u64,
        w: u64,
        k: usize,
        x1: u64,
        y1: u64,
        x2: u64,
        y2: u64,
    }

    let mut g = vec![0; 4];
    match (x1 == x2, y1 == y2) {
        (true, true) => g[3] = 1,
        (true, false) => g[1] = 1,
        (false, true) => g[2] = 1,
        (false, false) => g[0] = 1,
    }

    for _ in 1..=k {
        let mut next = vec![0; 4];

        // (x2, y2) に到達できるのは (x2, _) と (_, y2)
        next[3] = g[1] + g[2];
        next[3] %= MOD;

        // (_, y2) に到達できるのは (x2, y2), (_, y2), (_, _)
        next[2] = g[0] + (h - 2) * g[2] + (h - 1) * g[3];
        next[2] %= MOD;

        // (x2, _) に到達できるのは (x2, y2), (x2, _), (_, _)
        next[1] = g[0] + (w - 2) * g[1] + (w - 1) * g[3];
        next[1] %= MOD;

        // (_, _) に到達できるのは (x2, _), (_, y2), (_, _)
        next[0] = (h - 2 + w - 2) * g[0] + (h - 1) * g[1] + (w - 1) * g[2];
        next[0] %= MOD;

        g = next;
    }

    println!("{}", g[3] % MOD);
}
