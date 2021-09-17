use proconio::input;

// 024 Select +/- One (★2)
fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n]
    }

    let op_count = a
        .iter()
        .zip(b.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i64>();

    if k < op_count || (k - op_count) & 1 == 1 {
        println!("No");
    } else {
        println!("Yes");
    }
}
