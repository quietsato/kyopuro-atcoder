use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };
    let pins = {
        let index = [
            [7i32, -1, 8, -1, 9, -1, 10],
            [-1, 4, -1, 5, -1, 6, -1],
            [-1, -1, 2, -1, 3, -1, -1],
            [-1, -1, -1, 1, -1, -1, -1],
        ];
        let mut pins = vec![vec![false; 7]; 4];
        for (i, &c) in s.iter().enumerate() {
            for (rowi, row) in index.iter().enumerate() {
                if let Some((coli, _)) = row.iter().find_position(|&&p| p == (i + 1) as i32) {
                    pins[rowi][coli] = c == '1';
                }
            }
        }
        pins
    };
    if pins[3][3] {
        println!("No");
        return;
    }
    let ans = {
        let left = (0..7)
            .find_position(|&coli| (0..4).any(|rowi| pins[rowi][coli]))
            .map(|(_, i)| i);
        let right = (0..7)
            .rev()
            .find_position(|&coli| (0..4).any(|rowi| pins[rowi][coli]))
            .map(|(_, i)| i);
        match (left, right) {
            (Some(left), Some(right)) => {
                (left + 1..right).any(|colmid| (0..4).all(|rowi| !pins[rowi][colmid]))
            }
            _ => false,
        }
    };
    println!("{}", if ans { "Yes" } else { "No" });
}
