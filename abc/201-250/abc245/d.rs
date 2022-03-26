use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i64; n + 1],
        mut c: [i64; n + m + 1]
    }

    let mut b = vec![];

    for i in 0..=m {
        let mul = c[n + m - i] / a[n];
        c.pop();
        for j in 1..=n {
            c[n + m - i - j] -= mul * a[n - j];
        }
        b.push(mul);
    }

    b.reverse();
    println!(
        "{}",
        b.iter()
            .map(|num| num.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
