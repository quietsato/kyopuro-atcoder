use proconio::input;

// 033 Not Too Bright (★2)
fn main() {
    input! {
        h: usize,
        w: usize
    }

    println!(
        "{}",
        if h == 1 || w == 1 {
            h * w
        } else {
            ((h + 1) / 2) * ((w + 1) / 2)
        }
    );
}
