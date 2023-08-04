use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        a: [Chars; n]
    };

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            match (a[i][j], a[j][i]) {
                ('W', 'L') | ('L', 'W') | ('D', 'D') => {}
                _ => {
                    println!("incorrect");
                    return;
                }
            }
        }
    }
    println!("correct");
}
