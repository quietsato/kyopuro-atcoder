use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        x: Usize1,
        a: [Usize1; n]
    }

    let mut known = vec![false; n];

    let mut ans = 1;
    let mut i = x;
    known[i] = true;
    while !known[a[i]] {
        known[a[i]] = true;
        ans += 1;
        i = a[i];
    }

    println!("{}", ans);
}
