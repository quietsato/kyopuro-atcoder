use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [u64; n],
        mut t: [u64; n],
    }

    for i in 0..2 * n {
        t[(i + 1) % n] = t[(i + 1) % n].min(t[i % n] + s[i % n])
    }

    t.iter().for_each(|t| println!("{}", t))
}
