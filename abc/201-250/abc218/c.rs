use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars;n],
        t: [Chars;n],
    }

    // Trim surrounding '.'s
    let s = s
        .into_iter()
        .skip_while(|row| row.iter().all(|c| c == &'.'))
        .collect_vec();
    let mut s = s
        .into_iter()
        .rev()
        .skip_while(|row| row.iter().all(|c| c == &'.'))
        .collect_vec();
    s.reverse();
    let t = t
        .into_iter()
        .skip_while(|row| row.iter().all(|c| c == &'.'))
        .collect_vec();
    let mut t = t
        .into_iter()
        .rev()
        .skip_while(|row| row.iter().all(|c| c == &'.'))
        .collect_vec();
    t.reverse();

    for i in 0..n {
        if s.iter().all(|row| row[n - i - 1] == '.') {
            for row in s.iter_mut() {
                row.remove(n - i - 1);
            }
        } else {
            break;
        }
    }
    while s.iter().all(|row| row[0] == '.') {
        for row in s.iter_mut() {
            row.remove(0);
        }
    }
    for i in 0..n {
        if t.iter().all(|row| row[n - i - 1] == '.') {
            for row in t.iter_mut() {
                row.remove(n - i - 1);
            }
        } else {
            break;
        }
    }
    while t.iter().all(|row| row[0] == '.') {
        for row in t.iter_mut() {
            row.remove(0);
        }
    }

    if s.len() == t.len() && s[0].len() == t[0].len() {
        // 0 deg
        if s.iter()
            .zip(t.iter())
            .all(|(s, t)| s.iter().zip(t.iter()).all(|(s, t)| s == t))
        {
            println!("Yes");
            return;
        }
        // 180 deg
        else if s
            .iter()
            .zip(t.iter().rev())
            .all(|(s, t)| s.iter().zip(t.iter().rev()).all(|(s, t)| s == t))
        {
            println!("Yes");
            return;
        }
    }
    if s.len() == t[0].len() && s[0].len() == t.len() {
        // 90 deg
        let mut flag = true;
        for i in 0..s.len() {
            for j in 0..s[0].len() {
                if s[i][j] != t[t.len() - j - 1][i] {
                    flag = false;
                }
            }
        }
        if flag {
            println!("Yes");
            return;
        }
        // -90 deg
        let mut flag = true;
        for i in 0..s.len() {
            for j in 0..s[0].len() {
                if s[i][j] != t[j][t[0].len() - i - 1] {
                    flag = false;
                }
            }
        }
        if flag {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
