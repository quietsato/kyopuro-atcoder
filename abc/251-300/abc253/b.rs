use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    let mut x1 = 0;
    let mut x2 = 0;
    let mut y1 = 0;
    let mut y2 = 0;
    let mut found1 = false;

    (0..h).for_each(|y| {
        (0..w).for_each(|x| {
            if s[y][x] == 'o' {
                if !found1 {
                    found1 = true;
                    x1 = x as i64;
                    y1 = y as i64;
                } else {
                    x2 = x as i64;
                    y2 = y as i64;
                }
            }
        });
    });

    println!("{}", (x1 - x2).abs() + (y1 - y2).abs());
}
