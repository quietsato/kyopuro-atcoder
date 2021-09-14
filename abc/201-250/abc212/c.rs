use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i64; n],
        mut b: [i64; m],
    }

    a.sort();
    b.sort();

    let (mut i, mut j) = (0, 0);
    let mut ans = 1_000_000_000i64;
    while i < n && j < m {
        ans = ans.min((a[i] - b[j]).abs());
        if a[i] < b[j] {
            i += 1;
        } else {
            j += 1;
        }
    }

    println!("{}", ans);
}
