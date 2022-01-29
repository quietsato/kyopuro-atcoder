use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; 4 * n - 1]
    }

    let mut cnt = vec![0; n];
    for a in a {
        cnt[a] += 1;
    }

    let ans = cnt.into_iter().position(|c| c == 3).unwrap();
    println!("{}", ans + 1);
}
