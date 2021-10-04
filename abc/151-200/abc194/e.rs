use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n]
    }

    let mut count = vec![0; n + 1];
    for &a in a.iter().take(m) {
        count[a] += 1;
    }

    let mut ans = count
        .iter()
        .enumerate()
        .skip_while(|(_, a)| **a > 0)
        .next()
        .unwrap()
        .0;

    for i in 0..n - m {
        count[a[i]] -= 1;
        count[a[i + m]] += 1;
        if count[a[i]] == 0 {
            ans = ans.min(a[i]);
        }
    }

    println!("{}", ans);
}
