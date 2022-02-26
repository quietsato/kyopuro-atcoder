use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }
    for y in 0..=n - 6 {
        for x in 0..=n - 6 {
            // 縦
            let v = (0..6).any(|i| (0..6).filter(|j| s[y + i][x + j] == '#').count() >= 4);
            // 横
            let h = (0..6).any(|i| (0..6).filter(|j| s[y + j][x + i] == '#').count() >= 4);
            // 斜め
            let d = ((0..6).filter(|i| s[y + i][x + i] == '#').count() >= 4)
                || ((0..6).filter(|i| s[y + i][x + 5 - i] == '#').count() >= 4);
            if v || h || d {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
