use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: [[u64; m]; n]
    }

    for i in 0..n {
        let mut row_prev = b[i][0];
        for j in 1..m {
            if row_prev + 1 == b[i][j] && (row_prev - 1) / 7 == (b[i][j] - 1) / 7 {
                row_prev = b[i][j];
            } else {
                println!("No");
                return;
            }
        }
    }

    for j in 0..m {
        let mut col_prev = b[0][j];
        for i in 1..n {
            if col_prev + 7 == b[i][j]  {
                col_prev = b[i][j];
            }
            else {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
