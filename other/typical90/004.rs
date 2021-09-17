use itertools::Itertools;
use proconio::input;

// 004 Cross Sum (★2)
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u32; w];h]
    }

    let sum_row = (0..h).map(|i| a[i].iter().sum::<u32>()).collect_vec();
    let sum_col = (0..w)
        .map(|j| (0..h).map(|i| a[i][j]).sum::<u32>())
        .collect_vec();

    for i in 0..h {
        println!(
            "{}",
            (0..w)
                .map(|j| { (sum_row[i] + sum_col[j] - a[i][j]).to_string() })
                .join(" ")
        )
    }
}
